use serde::{Deserialize, Serialize};
use crate::models::user::UserModel;

// User model for API
#[derive(Serialize, Deserialize, Clone, utoipa::ToSchema)]
pub struct UserResponse {
    #[schema(example = "200")]
    pub status: String,
    pub user: UserModel
}

// App Response
#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct AppResponse {
    #[schema(example = "200")]
    pub status: String,
    #[schema(example = "Hello world!")]
    pub data: String,
}



// Health Check Response
#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct HealthResponse {
    #[schema(example = "OK")]
    pub status: String,
    #[schema(example = "1.0.0")]
    pub version: String,
}