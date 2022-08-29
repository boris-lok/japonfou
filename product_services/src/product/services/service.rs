use std::sync::Arc;

use async_trait::async_trait;
use futures::lock::Mutex;
use rust_decimal::Decimal;
use sqlx::pool::PoolConnection;
use sqlx::Postgres;

use common::json::product::Product;
use common::product_pb::{CreateProductRequest, UpdateProductRequest};
use common::types::ListRequest;
use common::util::alias::AppResult;
use common::util::errors::AppError;
use common::util::tools::{begin_transaction, commit_transaction, database_error_handler};

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
    session: Arc<Mutex<PoolConnection<Postgres>>>,
    repo: Box<dyn ProductRepo + Send + Sync>,
}

impl ProductServiceImpl {
    pub fn new(session: Arc<Mutex<PoolConnection<Postgres>>>) -> Self {
        let repo = Box::new(ProductRepoImpl::new(session.clone()));
        Self { session, repo }
    }
}

#[async_trait]
impl ProductService for ProductServiceImpl {
    async fn get(&self, id: i64) -> AppResult<Option<Product>> {
        self.repo.get(id).await.map_err(database_error_handler)
    }

    async fn create(&self, request: CreateProductRequest) -> AppResult<Product> {
        self.repo
            .create(request)
            .await
            .map_err(database_error_handler)
    }

    async fn update(&self, request: UpdateProductRequest) -> AppResult<Product> {
        let _ = begin_transaction(self.session.clone()).await;
        let old_product = self.repo.get(request.id as i64).await.ok().flatten();

        if let Some(p) = old_product {
            let is_affected = self.repo.update(request.clone()).await;

            let _ = commit_transaction(self.session.clone()).await;

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
        }

        Err(AppError::DatabaseError(
            "failed to update product.".to_string(),
        ))
    }

    async fn list(&self, request: ListRequest) -> AppResult<Vec<Product>> {
        self.repo
            .list(request)
            .await
            .map_err(database_error_handler)
    }
}
