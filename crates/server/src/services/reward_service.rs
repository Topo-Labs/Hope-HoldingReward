// use crate::dtos::user_dto::GetUserDto;
use async_trait::async_trait;
use database::reward::{
    model::{Reward, RewardItem},
    repository::DynRewardRepository,
};
use mongodb::results::{InsertOneResult, UpdateResult};
use std::sync::Arc;
// use tracing::{error, info};
use utils::AppResult;

pub type DynRewardService = Arc<dyn RewardServiceTrait + Send + Sync>;

#[async_trait]
pub trait RewardServiceTrait {
    async fn create_reward(
        &self,
        address: String,
        rewards: Vec<RewardItem>,
    ) -> AppResult<InsertOneResult>;
    async fn set_reward(&self, address: String) -> AppResult<UpdateResult>;
    async fn set_rewards(&self, addresses: Vec<String>) -> AppResult<UpdateResult>;
    async fn get_reward(&self, address: String) -> AppResult<Option<Reward>>;
    async fn get_rewards_by_day(&self, day: String) -> AppResult<Vec<Reward>>;
}

#[derive(Clone)]
pub struct RewardService {
    repository: DynRewardRepository,
}

impl RewardService {
    pub fn new(repository: DynRewardRepository) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl RewardServiceTrait for RewardService {
    async fn create_reward(
        &self,
        address: String,
        rewards: Vec<RewardItem>,
    ) -> AppResult<InsertOneResult> {
        let reward = self.repository.create_reward(&address, rewards).await?;

        Ok(reward)
    }

    async fn set_reward(&self, address: String) -> AppResult<UpdateResult> {
        let reward = self.repository.set_reward(&address).await?;

        Ok(reward)
    }

    async fn set_rewards(&self, addresses: Vec<String>) -> AppResult<UpdateResult> {
        let rewards = self.repository.set_rewards(addresses).await?;

        Ok(rewards)
    }

    async fn get_reward(&self, address: String) -> AppResult<Option<Reward>> {
        let reward = self.repository.get_reward(&address).await?;

        Ok(reward)
    }

    async fn get_rewards_by_day(&self, day: String) -> AppResult<Vec<Reward>> {
        let rewards = self.repository.get_rewards_by_day(&day).await?;

        Ok(rewards)
    }
}
