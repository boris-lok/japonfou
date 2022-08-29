use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use futures::lock::Mutex;
use sqlx::{Pool, Postgres};
use sqlx::pool::PoolConnection;
use tonic::{Request, Response, Status};
use tracing::instrument;

use common::product_pb::{
    CreateProductRequest, GetProductResponse, ListProductResponse, Product, UpdateProductRequest,
};
use common::product_pb::product_services_server::ProductServices;
use common::types::{GetByIdRequest, ListRequest};
use common::util::tools::grpc_error_handler;

use crate::product::services::service::{ProductService, ProductServiceImpl};

#[derive(Debug)]
pub struct ProductServicesImpl {
    pool: Pool<Postgres>,
}

impl ProductServicesImpl {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    async fn get_session(&self) -> Result<Arc<Mutex<PoolConnection<Postgres>>>> {
        let conn = self.pool.acquire().await?;

        Ok(Arc::new(Mutex::new(conn)))
    }
}

#[async_trait]
impl ProductServices for ProductServicesImpl {
    #[instrument]
    async fn create(
        &self,
        request: Request<CreateProductRequest>,
    ) -> Result<Response<Product>, Status> {
        let request = request.into_inner();
        let session = self.get_session().await.unwrap();

        let services = ProductServiceImpl::new(session);

        services
            .create(request)
            .await
            .map(|p| {
                let p: Product = p.into();
                Response::new(p)
            })
            .map_err(grpc_error_handler)
    }

    async fn update(
        &self,
        request: Request<UpdateProductRequest>,
    ) -> Result<Response<Product>, Status> {
        let request = request.into_inner();
        let session = self.get_session().await.unwrap();

        let services = ProductServiceImpl::new(session);

        services
            .update(request)
            .await
            .map(|p| {
                let p: Product = p.into();
                Response::new(p)
            })
            .map_err(grpc_error_handler)
    }

    async fn get(
        &self,
        request: Request<GetByIdRequest>,
    ) -> Result<Response<GetProductResponse>, Status> {
        let request = request.into_inner();
        let session = self.get_session().await.unwrap();

        let services = ProductServiceImpl::new(session);

        services
            .get(request.id as i64)
            .await
            .map(|p| {
                let p: Option<Product> = p.map(|e| e.into());
                Response::new(GetProductResponse { product: p })
            })
            .map_err(grpc_error_handler)
    }

    async fn list(
        &self,
        request: Request<ListRequest>,
    ) -> Result<Response<ListProductResponse>, Status> {
        let request = request.into_inner();
        let session = self.get_session().await.unwrap();

        let services = ProductServiceImpl::new(session);

        services
            .list(request)
            .await
            .map(|p| {
                let p: Vec<Product> = p.into_iter().map(|e| e.into()).collect();
                Response::new(ListProductResponse { products: p })
            })
            .map_err(grpc_error_handler)
    }
}
