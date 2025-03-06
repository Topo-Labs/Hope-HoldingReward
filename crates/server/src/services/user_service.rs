// use crate::dtos::user_dto::GetUserDto;
use async_trait::async_trait;
use database::user::{model::User, repository::DynUserRepository};
// use mongodb::results::InsertOneResult;
use std::sync::Arc;
// use tracing::{error, info};
use utils::AppResult;

pub type DynUserService = Arc<dyn UserServiceTrait + Send + Sync>;

#[async_trait]
pub trait UserServiceTrait {
    async fn get_user(&self, address: String) -> AppResult<Option<User>>;
}

#[derive(Clone)]
pub struct UserService {
    repository: DynUserRepository,
}

impl UserService {
    pub fn new(repository: DynUserRepository) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl UserServiceTrait for UserService {
    async fn get_user(&self, address: String) -> AppResult<Option<User>> {
        // let address = request.address.unwrap();

        let user = self.repository.get_user(&address).await?;

        Ok(user)
    }
}
