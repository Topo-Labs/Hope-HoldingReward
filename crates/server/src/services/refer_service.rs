use async_trait::async_trait;
use database::refer::{model::Refer, repository::DynReferRepository};
use mongodb::results::{InsertManyResult, InsertOneResult};
use std::sync::Arc;
use utils::AppResult;

pub type DynReferService = Arc<dyn ReferServiceTrait + Send + Sync>;

#[async_trait]
pub trait ReferServiceTrait {
    async fn get_upper(&self, address: String) -> AppResult<Option<String>>;
    async fn get_uppers(&self, address: String) -> AppResult<Vec<String>>;
    async fn create_refer(&self, lower: &str, upper: &str) -> AppResult<InsertOneResult>;
    async fn create_refers(&self, refers: Vec<Refer>) -> AppResult<InsertManyResult>;
}

#[derive(Clone)]
pub struct ReferService {
    repository: DynReferRepository,
}

impl ReferService {
    pub fn new(repository: DynReferRepository) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl ReferServiceTrait for ReferService {
    async fn get_upper(&self, address: String) -> AppResult<Option<String>> {
        let upper = self.repository.get_upper(&address).await?;

        Ok(upper)
    }

    async fn get_uppers(&self, address: String) -> AppResult<Vec<String>> {
        let uppers = self.repository.get_uppers(&address).await?;

        Ok(uppers)
    }

    async fn create_refer(&self, lower: &str, upper: &str) -> AppResult<InsertOneResult> {
        let refer = self.repository.create_refer(lower, upper).await?;

        Ok(refer)
    }

    async fn create_refers(&self, refers: Vec<Refer>) -> AppResult<InsertManyResult> {
        let result = self.repository.create_refers(refers).await?;

        Ok(result)
    }
}
