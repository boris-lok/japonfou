use async_trait::async_trait;
use rust_decimal::Decimal;
use sqlx::{Pool, Postgres};

use common::json::product::Product;
use common::product_pb::{CreateProductRequest, UpdateProductRequest};
use common::types::ListRequest;
use common::util::alias::AppResult;
use common::util::errors::AppError;

use crate::product::repos::postgres_repo::ProductRepoImpl;
use crate::product::repos::repo::ProductRepo;

#[async_trait]
pub trait ProductService {
    async fn get(&self, id: i64) -> AppResult<Option<Product>>;

    async fn create(&self, request: CreateProductRequest) -> AppResult<Product>;

    async fn update(&self, request: UpdateProductRequest) -> AppResult<Product>;

    async fn list(&self, request: ListRequest) -> AppResult<Vec<Product>>;
}

pub struct ProductServiceImpl {
    pool: Pool<Postgres>,
}

impl ProductServiceImpl {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProductService for ProductServiceImpl {
    async fn get(&self, id: i64) -> AppResult<Option<Product>> {
        let repo = ProductRepoImpl;

        repo.get(id, &self.pool.clone()).await.map_err(|e| {
            let msg = e.to_string();
            AppError::DatabaseError(msg)
        })
    }

    async fn create(&self, request: CreateProductRequest) -> AppResult<Product> {
        let repo = ProductRepoImpl;

        let mut tx = self.pool.begin().await.unwrap();

        let product = repo.create(request, &mut *tx).await.map_err(|e| {
            let msg = e.to_string();
            AppError::DatabaseError(msg)
        });

        let _ = tx.commit().await.unwrap();

        product
    }

    async fn update(&self, request: UpdateProductRequest) -> AppResult<Product> {
        let repo = ProductRepoImpl;

        let mut tx = self.pool.begin().await.unwrap();

        let old_product = repo.get(request.id as i64, &mut *tx).await.ok().flatten();

        if let Some(p) = old_product {
            let is_affected = repo.update(request.clone(), &mut *tx).await;

            let _ = tx.commit().await.unwrap();

            if is_affected.is_ok() {
                let currency = request.currency.map(|c| c as i16).unwrap_or(p.currency);
                let price = request
                    .price
                    .map(|e| Decimal::from_f64_retain(e).unwrap())
                    .unwrap_or(p.price);
                let new_product = Product {
                    name: request.name.unwrap_or(p.name),
                    currency,
                    price,
                    ..p
                };

                return Ok(new_product);
            }

            return Ok(p);
        } else {
            let _ = tx.rollback().await.unwrap();
        }

        Err(AppError::DatabaseError(
            "failed to update product.".to_string(),
        ))
    }

    async fn list(&self, request: ListRequest) -> AppResult<Vec<Product>> {
        let repo = ProductRepoImpl;

        repo.list(request, &self.pool.clone()).await.map_err(|e| {
            let msg = e.to_string();
            AppError::DatabaseError(msg)
        })
    }
}
