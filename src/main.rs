mod dto;
mod models;
mod util;

// ..web service
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use scalar_doc::scalar_actix;
use util::helper::json_to_yaml;
use utoipa::OpenApi;
// ..custom
use crate::dto::request::CreateGuitarSVGRequest;
use crate::dto::response::{AppResponse, GuitarSVGResponse, HealthResponse, UserResponse};

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
    path = "/greet/{name}",
    responses(
        (status = 200, description = "sample GET request which accepts a URI parameter", body = AppResponse)
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

// Guitar collection
#[utoipa::path(
    post,
    tag = "guitar",
    path = "/guitar/utils/gen_svg",
    request_body = CreateGuitarSVGRequest,
    responses(
    (status = 201, description = "Generate Guitar SVG", body = GuitarSVGResponse)
    )
)]
#[post("/guitar/utils/gen_svg")]
async fn gen_svg_chord(payload: web::Json<CreateGuitarSVGRequest>) -> impl Responder {
    let svg_response = GuitarSVGResponse {
        status: "201".to_string(),
        data: payload.into_inner(),
    };
    HttpResponse::Created().json(svg_response)
}

// OpenAPI Documentation
#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        health_check,
        greet,
        gen_svg_chord,
    ),
    components(schemas(HealthResponse, AppResponse, UserResponse, GuitarSVGResponse, CreateGuitarSVGRequest)),
    tags(
        (name = "health", description = "Health check endpoint."),
        (name = "greet", description = "Greet user endpoint."),
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
            .service(greet)
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
