use serde::{Deserialize, Serialize};
use actix_multipart::form::{MultipartForm, tempfile::TempFile, text::Text};
use utoipa::ToSchema;

// Create User Request
#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct CreateUserJSONRequest {
    #[schema(example = "John Doe")]
    pub name: String,
    #[schema(example = "john@example.com")]
    pub email: String,
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

#[derive(Debug, MultipartForm, ToSchema)]
pub struct FileUploadForm {
    /// A text field
    #[schema(value_type = String)]
    pub other_param: Text<String>,
    /// The file to upload
    #[schema(value_type = String, format = Binary)]
    pub file: TempFile,
}