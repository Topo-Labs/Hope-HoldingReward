use crate::{
    dtos::reward_dto::{SetRewardDto, SetRewardsDto},
    extractors::validation_extractor::ValidationExtractor,
    services::Services,
};
use axum::{
    extract::Path,
    routing::{get, post},
    Extension, Json, Router,
};
use database::reward::model::Reward;
use mongodb::results::UpdateResult;
use utils::{AppError, AppResult};

pub struct RewardController;
impl RewardController {
    pub fn app() -> Router {
        Router::new()
            .route("/reward", post(Self::set_reward))
            .route("/rewards", post(Self::set_rewards))
            .route("/reward/:address", get(Self::get_reward))
            .route("/rewards_by_day/:day", get(Self::get_rewards_by_day))
    }

    pub async fn set_reward(
        Extension(services): Extension<Services>,
        ValidationExtractor(req): ValidationExtractor<SetRewardDto>,
    ) -> AppResult<Json<UpdateResult>> {
        let reward = services.reward.set_reward(req.address).await?;

        Ok(Json(reward))
    }

    pub async fn set_rewards(
        Extension(services): Extension<Services>,
        ValidationExtractor(req): ValidationExtractor<SetRewardsDto>,
    ) -> AppResult<Json<UpdateResult>> {
        let rewards = services.reward.set_rewards(req.addresses).await?;

        Ok(Json(rewards))
    }

    pub async fn get_reward(
        Extension(services): Extension<Services>,
        Path(address): Path<String>,
    ) -> AppResult<Json<Reward>> {
        match services.reward.get_reward(address.to_string()).await? {
            Some(reward) => Ok(Json(reward)),
            None => Err(AppError::NotFound(format!(
                "Reward with address {} not found.",
                address
            ))),
        }
    }

    pub async fn get_rewards_by_day(
        Extension(services): Extension<Services>,
        Path(day): Path<String>,
    ) -> AppResult<Json<Vec<Reward>>> {
        let rewards = services.reward.get_rewards_by_day(day.to_string()).await?;

        Ok(Json(rewards))
    }
}
