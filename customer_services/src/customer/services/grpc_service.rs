use anyhow::Result;
use futures::lock::Mutex;
use sqlx::pool::PoolConnection;
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use tonic::{Request, Response, Status};
use tracing::instrument;

use common::customer_pb::customer_services_server::CustomerServices;
use common::customer_pb::{
    CreateCustomerRequest, Customer, GetCustomerResponse, ListCustomerResponse,
    UpdateCustomerRequest,
};
use common::types::{GetByIdRequest, ListRequest};
use common::util::tools::grpc_error_handler;

use crate::customer::services::service::{CustomerService, CustomerServiceImpl};

#[derive(Debug)]
pub(crate) struct GrpcCustomerServicesImpl {
    pool: Pool<Postgres>,
}

impl GrpcCustomerServicesImpl {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    async fn get_session(&self) -> Result<Arc<Mutex<PoolConnection<Postgres>>>> {
        let conn = self.pool.acquire().await?;

        Ok(Arc::new(Mutex::new(conn)))
    }
}

#[tonic::async_trait]
impl CustomerServices for GrpcCustomerServicesImpl {
    // TODO: handle get_session error.
    #[instrument]
    async fn create(
        &self,
        request: Request<CreateCustomerRequest>,
    ) -> Result<Response<Customer>, Status> {
        let request = request.into_inner();
        let session = self.get_session().await.unwrap();

        let services = CustomerServiceImpl::new(session);

        services
            .create(request)
            .await
            .map(|e| Response::new(e.into()))
            .map_err(grpc_error_handler)
    }

    #[instrument]
    async fn update(
        &self,
        request: Request<UpdateCustomerRequest>,
    ) -> Result<Response<Customer>, Status> {
        let request = request.into_inner();
        let session = self.get_session().await.unwrap();

        let services = CustomerServiceImpl::new(session);

        services
            .update(request)
            .await
            .map(|e| e.into())
            .map(Response::new)
            .map_err(grpc_error_handler)
    }

    #[instrument]
    async fn get(
        &self,
        request: Request<GetByIdRequest>,
    ) -> Result<Response<GetCustomerResponse>, Status> {
        let id = request.into_inner().id;
        let session = self.get_session().await.unwrap();

        let services = CustomerServiceImpl::new(session);

        services
            .get(id as i64)
            .await
            .map(|s| s.map(|e| e.into()))
            .map(|c| Response::new(GetCustomerResponse { customer: c }))
            .map_err(grpc_error_handler)
    }

    #[instrument]
    async fn list(
        &self,
        request: Request<ListRequest>,
    ) -> Result<Response<ListCustomerResponse>, Status> {
        let request = request.into_inner();
        let session = self.get_session().await.unwrap();

        let services = CustomerServiceImpl::new(session);

        services
            .list(request)
            .await
            .map(|e| {
                let c = e.into_iter().map(|e| e.into()).collect::<_>();
                ListCustomerResponse { customers: c }
            })
            .map(Response::new)
            .map_err(grpc_error_handler)
    }
}
