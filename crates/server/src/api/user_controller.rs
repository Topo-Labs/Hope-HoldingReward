use crate::services::Services;
use axum::{extract::Path, routing::get, Extension, Json, Router};
use database::user::model::User;
use utils::{AppError, AppResult};

pub struct UserController;
impl UserController {
    pub fn app() -> Router {
        Router::new().route("/user/:address", get(Self::user))
    }

    pub async fn user(
        Extension(services): Extension<Services>,
        Path(address): Path<String>,
    ) -> AppResult<Json<User>> {
        match services.user.get_user(address.to_string()).await? {
            Some(user) => Ok(Json(user)),
            None => Err(AppError::NotFound(format!(
                "User with address {} not found.",
                address
            ))),
        }
    }
}
