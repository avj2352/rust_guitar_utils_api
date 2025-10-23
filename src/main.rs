mod dto;
mod services;
mod util;

// ..web service
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use scalar_doc::scalar_actix;
use util::helper::json_to_yaml;
use utoipa::OpenApi;
// ..custom
use crate::dto::request::CreateGuitarSVGRequest;
use crate::dto::response::{AppResponse, GuitarSVGResponse, HealthResponse};
use crate::services::chord_v2::create_svg;

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

// Sample URI GET request
#[utoipa::path(
    get,
    tag = "health",
    path = "/about",
    responses(
        (status = 200, description = "about the webservice", body = AppResponse)
    ),
)]
#[get("/about")]
async fn about_webservice() -> impl Responder {
    HttpResponse::Ok().json(AppResponse {
        status: "200".to_string(),
        data: format!("Microservice to render guitar tabs into SVG frames"),
    })
}

// Guitar collection
#[utoipa::path(
    post,
    tag = "guitar",
    path = "/guitar/utils/gen_svg",
    request_body = CreateGuitarSVGRequest,
    responses(
        (status = 201, description = "generate svg frame based on guitar tab annotation", body = String, content_type = "image/svg+xml")
    )
)]
#[post("/guitar/utils/gen_svg")]
async fn gen_svg_chord(payload: web::Json<CreateGuitarSVGRequest>) -> impl Responder {
    let svg_content: String = create_svg(payload.tab.as_str(), payload.title.as_str());
    HttpResponse::Created()
        .content_type("image/svg+xml")
        .body(svg_content)
}

// OpenAPI Documentation
#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        health_check,
        about_webservice,
        gen_svg_chord,
    ),
    components(schemas(HealthResponse, AppResponse, GuitarSVGResponse, CreateGuitarSVGRequest)),
    tags(
        (name = "health", description = "Health check endpoint."),
        (name = "about", description = "about webservice"),
        (name = "guitar", description = "collection of guitar utils api endpoints")
    )
)]
struct ApiDoc;

// Scalar Documentation Endpoint
#[get("/docs")]
async fn docs() -> impl Responder {
    scalar_actix::ActixDocumentation::new("Rust Guitar Utils Webservice", "/openapi.json")
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
    let host = "127.0.0.1";
    let port = 8080;
    println!("server is running at: http://{}:{}/docs", &host, &port);
    HttpServer::new(|| {
        App::new()
            // endpoints
            .service(health_check)
            .service(about_webservice)
            .service(gen_svg_chord)
            // docs
            .service(docs)
            .service(openapi_json)
            .service(openapi_yaml)
    })
    .bind((host, port))?
    .run()
    .await
}
