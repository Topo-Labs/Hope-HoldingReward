use crate::{user::model::User, Database};
use async_trait::async_trait;
use chrono::Utc;
use mongodb::{bson::doc, results::InsertOneResult};
use std::sync::Arc;
use utils::AppResult;

pub type DynUserRepository = Arc<dyn UserRepositoryTrait + Send + Sync>;

// 主要用于Service中，表示提供了该Trait功能
#[async_trait]
pub trait UserRepositoryTrait {
    async fn create_user(
        &self,
        address: &str,
        amount: f64,
        price: f64,
    ) -> AppResult<InsertOneResult>;

    async fn get_user(&self, address: &str) -> AppResult<Option<User>>;
}

#[async_trait]
impl UserRepositoryTrait for Database {
    async fn create_user(
        &self,
        address: &str,
        amount: f64,
        price: f64,
    ) -> AppResult<InsertOneResult> {
        let new_doc = User {
            address: address.to_string(),
            amount: amount.floor().to_string(),
            price: format!("{:.20}", price),
            timestamp: Utc::now(),
        };

        let user = self.users.insert_one(new_doc, None).await?;

        Ok(user)
    }

    async fn get_user(&self, address: &str) -> AppResult<Option<User>> {
        let filter = doc! {"address": address};
        let user_detail = self.users.find_one(filter, None).await?;

        Ok(user_detail)
    }
}
