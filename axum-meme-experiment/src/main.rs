use std::net::SocketAddr;
use axum::{routing::post, Json};
use utoipa::{OpenApi, ToSchema};

#[derive(OpenApi)]
#[openapi(paths(echo))]
struct ApiDoc;

/// Todo struct to use for both the echo route and OpenAPI schema
#[derive(serde::Serialize, serde::Deserialize, ToSchema, Clone)]
struct Todo {
    id: i32,
    value: String,
    done: bool,
}

/// Echo route that returns the received data
#[utoipa::path(
    post,
    path = "/echo",
    request_body = Todo,
    responses(
        (status = 200, description = "Echoed Todo", body = Todo)
    )
)]
async fn echo(Json(todo): Json<Todo>) -> Json<Todo> {
    Json(todo)
}

#[tokio::main]
async fn main() {
    let socket_address: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    let listener = tokio::net::TcpListener::bind(socket_address).await.unwrap();

    let app = axum::Router::new()
        .route("/echo", post(echo))
        .merge(utoipa_swagger_ui::SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
