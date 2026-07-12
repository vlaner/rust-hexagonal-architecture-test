use std::sync::Arc;

use crate::modules::auth::api::UserServiceApi;
use crate::shared::apperror::AppError;
use actix_web::{Responder, web};
use chrono::{DateTime, Utc};

pub struct AppState {
    pub user_service: Arc<dyn UserServiceApi>,
}

#[derive(serde::Deserialize)]
pub struct UidParam {
    uid: uuid::Uuid,
}

#[derive(serde::Serialize)]
struct UserResponse {
    id: uuid::Uuid,
    username: String,
    created_at: DateTime<Utc>,
}

pub async fn index(
    state: web::Data<AppState>,
    query: web::Query<UidParam>,
) -> Result<impl Responder, AppError> {
    let user = state.user_service.get_user(query.uid).await?;

    Ok(web::Json(UserResponse {
        id: user.id,
        username: user.username,
        created_at: user.created_at,
    }))
}
