use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use tonic::{Request, Response, Status};
use tracing::instrument;

use common::pb::{
    CreateProductRequest, GetProductRequest, GetProductResponse,
    ListProductRequest, ListProductResponse, Product, product_services_server::ProductServices, UpdateProductRequest,
};

use crate::product::services::service::{ProductService, ProductServiceImpl};

#[derive(Debug)]
pub struct ProductServicesImpl {
    session: Pool<Postgres>,
}

impl ProductServicesImpl {
    pub fn new(session: Pool<Postgres>) -> Self {
        Self { session }
    }
}

#[async_trait]
impl ProductServices for ProductServicesImpl {
    #[instrument]
    async fn create(
        &self,
        request: Request<CreateProductRequest>,
    ) -> Result<Response<Product>, Status> {
        tracing::info!(message = "Got a request to create a product.");

        let request = request.into_inner();

        let services = ProductServiceImpl::new(self.session.clone());

        services
            .create(request)
            .await
            .map(|p| {
                let p: Product = p.into();
                Response::new(p)
            })
            .map_err(|err| {
                let msg = err.to_string();
                tracing::error!(%msg);
                Status::failed_precondition(msg)
            })
    }

    async fn update(
        &self,
        request: Request<UpdateProductRequest>,
    ) -> Result<Response<Product>, Status> {
        let request = request.into_inner();

        let services = ProductServiceImpl::new(self.session.clone());

        services
            .update(request)
            .await
            .map(|p| {
                let p: Product = p.into();
                Response::new(p)
            })
            .map_err(|err| {
                let msg = err.to_string();
                tracing::error!(%msg);
                Status::failed_precondition(msg)
            })
    }

    async fn get(
        &self,
        request: Request<GetProductRequest>,
    ) -> Result<Response<GetProductResponse>, Status> {
        let request = request.into_inner();

        let services = ProductServiceImpl::new(self.session.clone());

        services
            .get(request.id as i64)
            .await
            .map(|p| {
                let p: Option<Product> = p.map(|e| e.into());
                Response::new(GetProductResponse { product: p })
            })
            .map_err(|err| {
                let msg = err.to_string();
                tracing::error!(%msg);
                Status::failed_precondition(msg)
            })
    }

    async fn list(
        &self,
        request: Request<ListProductRequest>,
    ) -> Result<Response<ListProductResponse>, Status> {
        let request = request.into_inner();

        let services = ProductServiceImpl::new(self.session.clone());

        services
            .list(request)
            .await
            .map(|p| {
                let p: Vec<Product> = p.into_iter().map(|e| e.into()).collect();
                Response::new(ListProductResponse { products: p })
            })
            .map_err(|err| {
                let msg = err.to_string();
                tracing::error!(%msg);
                Status::failed_precondition(msg)
            })
    }
}
