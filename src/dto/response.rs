use super::request::CreateGuitarSVGRequest;
use serde::{Deserialize, Serialize};

// App Response
#[derive(Serialize, Deserialize, Clone, utoipa::ToSchema)]
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

// Guitar SVG Response
#[derive(Serialize, Deserialize, Clone, utoipa::ToSchema)]
pub struct GuitarSVGResponse {
    #[schema(example = "200")]
    pub status: String,
    pub data: CreateGuitarSVGRequest,
}
