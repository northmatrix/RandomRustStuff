use std::net::SocketAddr;
use axum::{response::IntoResponse, routing::post, Json};
use hyper::StatusCode;
use utoipa::{OpenApi, ToSchema};
use validator::Validate;

#[derive(OpenApi)]
#[openapi(paths(register))]
struct ApiDoc;

/// Todo struct to use for both the echo route and OpenAPI schema
#[derive(serde::Serialize, serde::Deserialize, ToSchema, Clone,Validate)]
struct Register {
    #[schema(example="JohnDoe")]
    #[validate(length(max=24,message="Username must be less than 24 characters long"))]
    username: String,
    #[schema(example="Xi481caqe9ro8r7isq821")]
    #[validate(length(min=8,message="Password must be at least 8 characters long"))]
    password: String,
}

#[derive(serde::Serialize,ToSchema)]
struct ErrorResponse {
    #[schema(example="validation_error")]
    error_type: String,
    #[schema(example="Password must have a minimum length of 8")]
    message: String,
}

/// Echo route that returns the received data
#[utoipa::path(
    post,
    path = "/register",
    request_body = Register,
    responses(
        (status = 200, description = "Registration is a success is a success", body = Register),
        (status = 400, description = "Registration validation failed", body = ErrorResponse)
    )
)]
async fn register(Json(register): Json<Register>) -> Result<(StatusCode,Json<Register>),(StatusCode,Json<ErrorResponse>)> {
    if let Err(e) = register.validate() {
        return Err((StatusCode::BAD_REQUEST,Json(ErrorResponse { error_type: "validation_error".to_string(), message: format!("{}", e) })))
    }
    Ok((StatusCode::OK, Json(register)))
}

#[tokio::main]
async fn main() {
    let socket_address: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    let listener = tokio::net::TcpListener::bind(socket_address).await.unwrap();

    let app = axum::Router::new()
        .route("/register", post(register))
        .merge(utoipa_swagger_ui::SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}



