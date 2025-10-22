use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, utoipa::ToSchema)]
pub struct CreateGuitarSVGRequest {
    #[schema(example = "Em")]
    pub title: String,
    #[schema(example = "0,2,2,0,0,0")]
    pub tab: String,
}
