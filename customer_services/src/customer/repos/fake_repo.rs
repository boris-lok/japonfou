use crate::customer::repos::repo::CustomerRepo;
use crate::ID_GENERATOR;
use async_trait::async_trait;
use common::customer_pb::{CreateCustomerRequest, UpdateCustomerRequest};
use common::json::customer::Customer;
use common::types::ListRequest;
use futures::lock::Mutex;
use futures::FutureExt;
use std::cmp::min;
use std::collections::HashMap;
use std::sync::Arc;

pub(crate) struct FakeCustomerRepo {
    session: Arc<Mutex<HashMap<i64, Customer>>>,
}

#[cfg(test)]
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
        let offset = (req.page * req.page_size) as usize;
        let end = offset + req.page_size as usize;
        let mut c = session
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

        let start = min(offset, c.len());
        let end = min(end, c.len());

        Ok(c.drain(start..end).collect::<Vec<_>>())
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

    async fn check_customer_is_exist(
        &self,
        phone: Option<String>,
        email: Option<String>,
    ) -> anyhow::Result<bool> {
        let session = self.session.lock().await;
        Ok(session
            .values()
            .filter(|&e| match (phone.clone(), email.clone()) {
                (Some(phone), Some(email)) => {
                    e.phone.clone().map_or(false, |e| e.eq(&phone))
                        && e.email.clone().map_or(false, |e| e.eq(&email))
                }
                (Some(phone), None) => e.phone.clone().map_or(false, |e| e.eq(&phone)),
                (None, Some(email)) => e.email.clone().map_or(false, |e| e.eq(&email)),
                _ => false,
            })
            .count()
            >= 1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn can_create_customer() {
        let repo = FakeCustomerRepo::new();
        let res = create_fake_customer(&repo, "boris".to_string(), None, None).await;
        assert!(res.is_ok());
        assert_eq!(res.as_ref().unwrap().name, "boris");
        assert_eq!(res.as_ref().unwrap().phone, None);
        assert_eq!(res.as_ref().unwrap().email, None);
    }

    #[tokio::test]
    async fn can_get_customer() {
        let repo = FakeCustomerRepo::new();
        let res = create_fake_customer(&repo, "boris".to_string(), None, None).await;
        let id = res.unwrap().id;
        let res = repo.get(id).await;
        assert!(res.is_ok());
        let res = res.unwrap();
        assert!(res.is_some());
        assert_eq!(res.as_ref().unwrap().name, "boris");
        assert_eq!(res.as_ref().unwrap().phone, None);
        assert_eq!(res.as_ref().unwrap().email, None);
    }

    #[tokio::test]
    async fn can_update_customer() {
        let repo = FakeCustomerRepo::new();
        let res = create_fake_customer(&repo, "boris".to_string(), None, None).await;
        let req = UpdateCustomerRequest {
            id: res.as_ref().unwrap().id as u64,
            name: None,
            email: Some("boris.lok@gmail.com".to_string()),
            phone: Some("1234567890".to_string()),
        };
        let res = repo.update(req).await;
        assert!(res.unwrap());
    }

    #[tokio::test]
    async fn can_list_customers() {
        let repo = FakeCustomerRepo::new();
        for i in 0..12 {
            let _ = create_fake_customer(&repo, format!("boris:{}", i), None, None).await;
        }
        let req = ListRequest {
            query: None,
            page: 0,
            page_size: 10,
        };
        let res = repo.list(req).await;
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.len(), 10);
    }

    #[tokio::test]
    async fn check_customer_is_exist() {
        let repo = FakeCustomerRepo::new();
        let email = Some("boris.lok@gmail.com".to_string());
        let phone = Some("123".to_string());
        let _ =
            create_fake_customer(&repo, "boris".to_string(), email.clone(), phone.clone()).await;

        let res = repo.check_customer_is_exist(phone, email).await;
        assert!(res.unwrap());
    }

    async fn create_fake_customer(
        repo: &dyn CustomerRepo,
        name: String,
        email: Option<String>,
        phone: Option<String>,
    ) -> anyhow::Result<Customer> {
        let req = CreateCustomerRequest { name, email, phone };
        repo.create(req).await
    }
}
