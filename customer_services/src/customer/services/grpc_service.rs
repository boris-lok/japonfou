use sqlx::{Pool, Postgres};
use tonic::{Request, Response, Status};
use tracing::instrument;

use common::pb::{
    CreateCustomerRequest, Customer, GetCustomerRequest, GetCustomerResponse, ListCustomerRequest,
    ListCustomerResponse, UpdateCustomerRequest,
};
use common::pb::customer_services_server::CustomerServices;

use crate::customer::services::service::{CustomerService, CustomerServiceImpl};

#[derive(Debug)]
pub struct CustomerServicesImpl {
    session: Pool<Postgres>,
}

impl CustomerServicesImpl {
    pub fn new(session: Pool<Postgres>) -> Self {
        Self { session }
    }
}

#[tonic::async_trait]
impl CustomerServices for CustomerServicesImpl {
    #[instrument]
    async fn create(
        &self,
        request: Request<CreateCustomerRequest>,
    ) -> Result<Response<Customer>, Status> {
        tracing::info!(message = "Got a request to create a customer.");

        let request = request.into_inner();

        let services = CustomerServiceImpl::new(self.session.clone());

        let customer = services.create(request).await.map(|e| e.into());

        customer.map(Response::new).map_err(|err| {
            let msg = err.to_string();
            tracing::error!(message = "failed to create a customer", %msg);
            Status::failed_precondition(msg)
        })
    }

    #[instrument]
    async fn update(
        &self,
        request: Request<UpdateCustomerRequest>,
    ) -> Result<Response<Customer>, Status> {
        let request = request.into_inner();
        let id = request.id;
        tracing::info!(message = "Got a request to update a customer", %id);

        let services = CustomerServiceImpl::new(self.session.clone());

        let customer = services.update(request).await.map(|e| e.into());

        customer.map(Response::new).map_err(|err| {
            let msg = err.to_string();
            tracing::error!(message = "failed to update a customer", %msg);
            Status::failed_precondition(msg)
        })
    }

    #[instrument]
    async fn get(
        &self,
        request: Request<GetCustomerRequest>,
    ) -> Result<Response<GetCustomerResponse>, Status> {
        let id = request.into_inner().id;
        tracing::info!(message = "Get a request to get a customer", %id);

        let services = CustomerServiceImpl::new(self.session.clone());

        let customer = services.get(id as i64).await.map(|s| s.map(|e| e.into()));

        customer
            .map(|c| Response::new(GetCustomerResponse { customer: c }))
            .map_err(|err| {
                let msg = err.to_string();
                tracing::error!(message = "failed to get a customer", %msg);
                Status::failed_precondition(msg)
            })
    }

    #[instrument]
    async fn list(
        &self,
        request: Request<ListCustomerRequest>,
    ) -> Result<Response<ListCustomerResponse>, Status> {
        let request = request.into_inner();

        let services = CustomerServiceImpl::new(self.session.clone());

        let customers = services.list(request).await.map(|e| {
            let c = e.into_iter().map(|e| e.into()).collect::<_>();

            ListCustomerResponse { customers: c }
        });

        customers.map(Response::new).map_err(|err| {
            let msg = err.to_string();
            tracing::error!(message = "failed to update a customer", %msg);
            Status::failed_precondition(msg)
        })
    }
}
