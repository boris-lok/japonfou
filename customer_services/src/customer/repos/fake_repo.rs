use crate::customer::repos::repo::CustomerRepo;
use crate::ID_GENERATOR;
use async_trait::async_trait;
use common::customer_pb::{CreateCustomerRequest, UpdateCustomerRequest};
use common::json::customer::Customer;
use common::types::ListRequest;
use futures::lock::Mutex;
use futures::FutureExt;
use std::collections::HashMap;
use std::sync::Arc;

pub(crate) struct FakeCustomerRepo {
    session: Arc<Mutex<HashMap<i64, Customer>>>,
}

impl FakeCustomerRepo {
    pub(crate) fn new() -> Self {
        Self {
            session: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl CustomerRepo for FakeCustomerRepo {
    async fn get(&self, id: i64) -> anyhow::Result<Option<Customer>> {
        let session = self.session.lock().await;
        return Ok(session.get(&id).map(|c| c.to_owned()));
    }

    async fn create(&self, req: CreateCustomerRequest) -> anyhow::Result<Customer> {
        let id = async move { ID_GENERATOR.clone().lock().unwrap().next_id() }
            .boxed()
            .await as i64;

        let mut session = self.session.lock().await;
        let c = Customer {
            id,
            name: req.name,
            email: req.email,
            phone: req.phone,
            created_at: chrono::Utc::now(),
            updated_at: None,
        };
        session.insert(id, c.clone());
        return Ok(c);
    }

    async fn list(&self, req: ListRequest) -> anyhow::Result<Vec<Customer>> {
        let session = self.session.lock().await;
        let c = session
            .values()
            .filter(|e| {
                if let Some(q) = req.query.to_owned() {
                    return e.name.to_lowercase().contains(&q.to_lowercase())
                        || e.email
                            .as_ref()
                            .map(|e| e.to_lowercase().contains(&q.to_lowercase()))
                            .unwrap_or(true)
                        || e.phone
                            .as_ref()
                            .map(|e| e.to_lowercase().contains(&q.to_lowercase()))
                            .unwrap_or(false);
                } else {
                    true
                }
            })
            .map(|c| c.to_owned())
            .collect::<Vec<_>>();
        Ok(c)
    }

    async fn update(&self, req: UpdateCustomerRequest) -> anyhow::Result<bool> {
        let id = req.id as i64;
        let mut session = self.session.lock().await;
        let mut c = session.get(&id).unwrap().to_owned();
        if let Some(name) = req.name {
            c.name = name;
        }
        if let Some(email) = req.email {
            c.email = Some(email);
        }
        if let Some(phone) = req.phone {
            c.phone = Some(phone);
        }
        session.insert(id, c);
        return Ok(true);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn can_create_customer() {
        let c = CreateCustomerRequest {
            name: "boris".to_string(),
            email: None,
            phone: None,
        };

        let repo = FakeCustomerRepo::new();
        let res = repo.create(c).await;
        assert!(res.is_ok());
        assert_eq!(res.as_ref().unwrap().name, "boris");
        assert_eq!(res.as_ref().unwrap().phone, None);
        assert_eq!(res.as_ref().unwrap().email, None);
        assert_ne!(Some(res.unwrap().created_at), None)
    }
}
