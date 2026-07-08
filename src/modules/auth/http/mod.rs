use actix_web::{Responder, web};
use chrono::{DateTime, Utc};

use crate::shared::{app_state::AppState, apperror::AppError};

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
