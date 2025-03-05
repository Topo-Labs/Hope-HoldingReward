use crate::{refer::model::Refer, Database};
use async_trait::async_trait;
use chrono::prelude::Utc;
use mongodb::{
    bson::doc,
    results::{InsertManyResult, InsertOneResult},
};
use std::sync::Arc;
use utils::AppResult;

pub type DynReferRepository = Arc<dyn ReferRepositoryTrait + Send + Sync>;

// 主要用于Service中，表示提供了该Trait功能
#[async_trait]
pub trait ReferRepositoryTrait {
    // 插入某个上下级关系
    async fn create_refer(&self, lower: &str, upper: &str) -> AppResult<InsertOneResult>;

    // 批量插入上下级关系
    async fn create_refers(&self, refers: Vec<Refer>) -> AppResult<InsertManyResult>;

    // 获取某个地址的上级
    async fn get_upper(&self, id: &str) -> AppResult<Option<String>>;

    // 获取某个地址的上级&上上级
    async fn get_upper_chain(&self, email: &str) -> AppResult<Vec<String>>;

    // // 获取某个地址的所有下级
    // async fn get_lowers(&self, id: &str) -> AppResult<Option<User>>;

    // // 获取某个地址的所有下级和下下级
    // async fn get_lowers_chain(&self, email: &str) -> AppResult<Option<User>>;
}

#[async_trait]
impl ReferRepositoryTrait for Database {
    async fn create_refer(&self, lower: &str, upper: &str) -> AppResult<InsertOneResult> {
        let new_doc = Refer {
            lower: lower.to_string(),
            upper: upper.to_string(),
            timestamp: Utc::now(),
        };

        let refer = self.refers.insert_one(new_doc, None).await?;

        Ok(refer)
    }

    async fn create_refers(&self, refers: Vec<Refer>) -> AppResult<InsertManyResult> {
        let result = self.refers.insert_many(refers, None).await?;

        Ok(result)
    }

    async fn get_upper(&self, lower: &str) -> AppResult<Option<String>> {
        let filter = doc! {"lower": lower};
        let refer = self.refers.find_one(filter, None).await?;

        Ok(refer.map(|r| r.upper))
    }

    async fn get_upper_chain(&self, lower: &str) -> AppResult<Vec<String>> {
        let mut result = Vec::new();
        let mut current_lower = lower.to_string();

        // 获取上级和上上级(再高的级别就不获取了)
        for _ in 0..2 {
            if let Some(upper) = self.get_upper(&current_lower).await? {
                result.push(upper.clone());
                current_lower = upper;
            } else {
                break;
            }
        }

        Ok(result)
    }
}
