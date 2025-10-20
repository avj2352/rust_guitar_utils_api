mod dto;
mod models;
mod util;

// ..web service
use actix_web::{get, App, HttpRequest, HttpResponse, HttpServer, Responder};
use scalar_doc::scalar_actix;
use util::helper::json_to_yaml;
use utoipa::OpenApi;
// ..custom
use crate::dto::response::{AppResponse, HealthResponse, UserResponse};

// Health Check Endpoint
#[utoipa::path(
    get,
    tag = "health",
    path = "/health",
    responses(
        (status = 200, description = "Health check successful", body = HealthResponse)
    )
)]
#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse {
        status: "OK".to_string(),
        version: "1.0.0".to_string(),
    })
}

// Example of a URI
#[utoipa::path(
    get,
    tag = "health",
    path = "/greet/{name}",
    responses(
        (status = 200, description = "Health check which accepts a uri value", body = AppResponse)
    ),
    params(
        ("name" = String, Path, description = "Name to greet")
    )
)]
#[get("/greet/{name}")]
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("world");
    HttpResponse::Ok().json(AppResponse {
        status: "200".to_string(),
        data: format!("Hello {}!", &name),
    })
}

// OpenAPI Documentation
#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        health_check,
        greet,
    ),
    components(schemas(HealthResponse, AppResponse, UserResponse)),
    tags(
        (name = "health", description = "Health check endpoint."),
        (name = "greet", description = "Greet user endpoint."),
        (name = "users", description = "User management endpoints.")
    )
)]
struct ApiDoc;

// Scalar Documentation Endpoint
#[get("/docs")]
async fn docs() -> impl Responder {
    scalar_actix::ActixDocumentation::new("User Management API", "/openapi.json")
        .theme(scalar_doc::Theme::Purple)
        .service()
}

// OpenAPI JSON Endpoint
#[get("/openapi.json")]
async fn openapi_json() -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(ApiDoc::openapi().to_json().unwrap())
}

// OpenAPI YAML Endpoint
#[get("/openapi.yaml")]
async fn openapi_yaml() -> impl Responder {
    let json_string = ApiDoc::openapi().to_json().unwrap();
    let yaml_string = json_to_yaml(&json_string).unwrap();
    HttpResponse::Ok()
        .content_type("application/yaml")
        .body(yaml_string)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // endpoints
            .service(health_check)
            .service(greet)
            // docs
            .service(docs)
            .service(openapi_json)
            .service(openapi_yaml)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
