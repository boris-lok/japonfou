use std::sync::Arc;

use async_trait::async_trait;
use futures::lock::Mutex;
use sqlx::pool::PoolConnection;
use sqlx::Postgres;

use common::customer_pb::{CreateCustomerRequest, UpdateCustomerRequest};
use common::json::customer::Customer;
use common::types::ListRequest;
use common::util::alias::AppResult;
use common::util::errors::AppError;
use common::util::tools::{
    begin_transaction, commit_transaction, database_error_handler, rollback_transaction,
};

use crate::customer::repos::postgres_repo::CustomerRepoImpl;
use crate::customer::repos::repo::CustomerRepo;

#[async_trait]
pub(crate) trait CustomerService {
    async fn get(&self, id: i64) -> AppResult<Option<Customer>>;

    async fn create(&self, request: CreateCustomerRequest) -> AppResult<Customer>;

    async fn list(&self, request: ListRequest) -> AppResult<Vec<Customer>>;

    async fn update(&self, request: UpdateCustomerRequest) -> AppResult<Customer>;
}

pub(crate) struct CustomerServiceImpl {
    session: Arc<Mutex<PoolConnection<Postgres>>>,
    repo: Box<dyn CustomerRepo + Sync + Send>,
}

impl CustomerServiceImpl {
    pub(crate) fn new(session: Arc<Mutex<PoolConnection<Postgres>>>) -> Self {
        let repo = CustomerRepoImpl::new(session.clone());
        Self {
            session,
            repo: Box::new(repo),
        }
    }
}

#[async_trait]
impl CustomerService for CustomerServiceImpl {
    async fn get(&self, id: i64) -> AppResult<Option<Customer>> {
        self.repo.get(id).await.map_err(database_error_handler)
    }

    async fn create(&self, request: CreateCustomerRequest) -> AppResult<Customer> {
        let is_exist = self
            .repo
            .check_customer_is_exist(request.phone.clone(), request.email.clone())
            .await;

        if is_exist.is_err() {
            return Err(AppError::DatabaseError(
                is_exist.err().unwrap().to_string(),
            ));
        }

        if is_exist.is_ok() && is_exist.unwrap() {
            return Err(AppError::BadRequest("customer already exist.".to_string()));
        }

        self.repo
            .create(request)
            .await
            .map_err(database_error_handler)
    }

    async fn list(&self, request: ListRequest) -> AppResult<Vec<Customer>> {
        self.repo
            .list(request)
            .await
            .map_err(database_error_handler)
    }

    async fn update(&self, request: UpdateCustomerRequest) -> AppResult<Customer> {
        let _ = begin_transaction(self.session.clone()).await;
        let old_customer = self.repo.get(request.id as i64).await.ok().flatten();

        if old_customer.is_none() {
            let _ = rollback_transaction(self.session.clone()).await;
            return Err(AppError::BadRequest(format!(
                "Can't find the customer by id {}",
                request.id
            )));
        }

        let old_customer = old_customer.unwrap();

        let is_affected = self.repo.update(request.clone()).await;

        let _ = commit_transaction(self.session.clone()).await;

        if is_affected.is_ok() {
            let new_customer = Customer {
                name: request.name.unwrap_or(old_customer.name),
                email: request.email.to_owned(),
                phone: request.phone.to_owned(),
                ..old_customer
            };

            return Ok(new_customer);
        }

        return Ok(old_customer);
    }
}

#[cfg(test)]
mod test {
    use common::config::postgres_config::PostgresConfig;
    use common::util::connections::create_database_connection;

    use crate::customer::repos::fake_repo::FakeCustomerRepo;

    use super::*;

    impl CustomerServiceImpl {
        fn fake(session: Arc<Mutex<PoolConnection<Postgres>>>) -> Self {
            let repo = FakeCustomerRepo::new();
            Self {
                session,
                repo: Box::new(repo),
            }
        }
    }

    #[tokio::test]
    async fn can_create_customer() {
        let env_file = concat!(env!("CARGO_MANIFEST_DIR"), "/", "env", "/", "dev.env");
        let _ = dotenv::from_path(env_file);

        let database_config = PostgresConfig::new();

        let database_connection = create_database_connection(database_config)
            .await
            .expect("Can't connect to database.");

        let session = database_connection.acquire().await.unwrap();

        let fake_service = CustomerServiceImpl::fake(Arc::new(Mutex::new(session)));

        let req = CreateCustomerRequest {
            name: "boris".to_string(),
            email: None,
            phone: None,
        };

        let res = fake_service.create(req).await;

        assert!(res.is_ok());

        let id = res.unwrap().id;

        let customer = fake_service.repo.get(id).await;

        assert!(customer.is_ok());
        let customer = customer.unwrap();
        assert!(customer.is_some());
    }
}
