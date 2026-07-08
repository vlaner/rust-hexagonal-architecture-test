use std::sync::Arc;

use crate::modules::auth::service::user::UserServiceApi;

pub struct AppState {
    pub user_service: Arc<dyn UserServiceApi>,
}
