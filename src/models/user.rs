use serde::{Deserialize, Serialize};

// User model for API
#[derive(Serialize, Deserialize, Clone, utoipa::ToSchema)]
pub struct UserModel {
    #[schema(example = 1)]
    pub id: u64,
    #[schema(example = "John Doe")]
    pub name: String,
    #[schema(example = "john@example.com")]
    pub email: String,
}