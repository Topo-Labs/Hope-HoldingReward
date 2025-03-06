use crate::services::Services;
use axum::{extract::Path, routing::get, Extension, Json, Router};
use utils::{AppError, AppResult};

pub struct ReferController;
impl ReferController {
    pub fn app() -> Router {
        Router::new()
            .route("/refer/upper/:address", get(Self::upper))
            .route("/refer/uppers/:address", get(Self::uppers))
    }

    pub async fn upper(
        Extension(services): Extension<Services>,
        Path(address): Path<String>,
    ) -> AppResult<Json<String>> {
        match services.refer.get_upper(address.to_string()).await? {
            Some(upper) => Ok(Json(upper)),
            None => Err(AppError::NotFound(format!(
                "Upper of address {} not found.",
                address
            ))),
        }
    }

    pub async fn uppers(
        Extension(services): Extension<Services>,
        Path(address): Path<String>,
    ) -> AppResult<Json<Vec<String>>> {
        let uppers = services.refer.get_uppers(address.to_string()).await?;

        Ok(Json(uppers))
    }
}
