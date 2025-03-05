use crate::{
    reward::model::{Reward, RewardItem},
    Database,
};
use anyhow::Context;
use async_trait::async_trait;
use chrono::{NaiveDate, TimeZone, Utc};
// use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId, DateTime},
    results::{DeleteResult, InsertOneResult, UpdateResult},
};
use std::sync::Arc;
use tokio_stream::StreamExt;
use utils::AppResult;

pub type DynRewardRepository = Arc<dyn RewardRepositoryTrait + Send + Sync>;

#[async_trait]
pub trait RewardRepositoryTrait {
    // 创建奖励
    async fn create_reward(
        &self,
        user_address: &str,
        rewards: Vec<RewardItem>,
    ) -> AppResult<InsertOneResult>;

    // 将某笔奖励设置为已发放
    async fn set_reward(&self, user_address: &str) -> AppResult<UpdateResult>;

    // 批量设置奖励(用于项目方确认奖励已发放)
    async fn set_rewards(&self, users: Vec<String>) -> AppResult<UpdateResult>;

    // 获取某个用户所触发的奖励
    async fn get_reward(&self, user_address: &str) -> AppResult<Option<Reward>>;

    // 获取某一天的所有奖励
    async fn get_rewards_by_day(&self, day: &str) -> AppResult<Vec<Reward>>;

    // async fn delete_user(&self, id: &str) -> AppResult<DeleteResult>;
    //
    // async fn get_all_users(&self) -> AppResult<Vec<User>>;
}

#[async_trait]
impl RewardRepositoryTrait for Database {
    async fn create_reward(
        &self,
        user_address: &str,
        rewards: Vec<RewardItem>,
    ) -> AppResult<InsertOneResult> {
        let new_doc = Reward {
            is_rewarded: false,
            user_address: user_address.to_string(),
            rewards,
            timestamp: Utc::now(),
        };

        let reward = self.rewards.insert_one(new_doc, None).await?;

        Ok(reward)
    }

    async fn set_reward(&self, user_address: &str) -> AppResult<UpdateResult> {
        let filter = doc! {"user_address": user_address};
        let update = doc! {
            "$set":
                {
                    "is_rewarded": true,
                },
        };

        let updated_doc = self.users.update_one(filter, update, None).await?;

        Ok(updated_doc)
    }

    async fn set_rewards(&self, users: Vec<String>) -> AppResult<UpdateResult> {
        let filter = doc! {"user_address": {"$in": users}};
        let update = doc! {
            "$set":
                {
                    "is_rewarded": true,
                },
        };

        let updated_doc = self.users.update_many(filter, update, None).await?;

        Ok(updated_doc)
    }

    async fn get_reward(&self, user_address: &str) -> AppResult<Option<Reward>> {
        let filter = doc! {"user_address": user_address};
        let reward = self.rewards.find_one(filter, None).await?;

        Ok(reward)
    }

    async fn get_rewards_by_day(&self, day: &str) -> AppResult<Vec<Reward>> {
        let naive_date =
            NaiveDate::parse_from_str(day, "%Y-%m-%d").context("Invalid date format")?;

        let start_of_day = Utc.from_utc_datetime(&naive_date.and_hms_opt(0, 0, 0).unwrap());
        let end_of_day = Utc.from_utc_datetime(&naive_date.and_hms_opt(23, 59, 59).unwrap());

        let filter = doc! {
            "timestamp": {
                "$gte": DateTime::from_millis(start_of_day.timestamp_millis()),
                "$lte": DateTime::from_millis(end_of_day.timestamp_millis())
            }
        };

        let mut cursor = self.rewards.find(filter, None).await?;

        let mut rewards = Vec::new();
        while let Some(doc) = cursor.try_next().await? {
            rewards.push(doc);
        }

        Ok(rewards)
    }
}
