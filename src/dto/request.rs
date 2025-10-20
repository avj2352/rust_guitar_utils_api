use serde::{Deserialize, Serialize};

// Create User Request
#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct CreateUserJSONRequest {
    #[schema(example = "John Doe")]
    pub name: String,
    #[schema(example = "john@example.com")]
    pub email: String,
}

#[derive(Serialize, Deserialize, Clone, utoipa::ToSchema)]
pub struct CreateGuitarSVGRequest {
    #[schema(example = "Em")]
    pub title: String,
    #[schema(example = "022000")]
    pub tab: String,
}

#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct CreateUserFormRequest {
    #[schema(example = "John Doe")]
    pub name: String,
    #[schema(example = "john@example.com")]
    pub email: String,
}

#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct UserQueryParams {
    #[schema(example = "John Doe")]
    pub name: Option<String>,
    #[schema(example = 8)]
    pub age: Option<u32>,
}

#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct UserFilterQueryParams {
    #[schema(example = "some")]
    pub filter: Option<String>,
}
