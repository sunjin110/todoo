mod handlers;
mod repositorys;

use axum::extract::Extension;
use axum::routing::post;
use axum::{routing::get, Router};
use serde::{Deserialize, Serialize};
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;

use crate::handlers::create_todo;
use crate::repositorys::{TodoRepository, TodoRepositoryForMemory};

const DEFAULT_LOG_LEVEL: &'static str = "info";

#[tokio::main]
async fn main() {
    // logging
    let log_level = env::var("RUST_LOG").unwrap_or(DEFAULT_LOG_LEVEL.to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();

    // routing
    let repository = TodoRepositoryForMemory::new();

    let app = create_app(repository);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);

    tracing::info!("start server -> localhost:3000");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn create_app<T: TodoRepository>(repository: T) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/todos", post(create_todo::<T>))
        .layer(Extension(Arc::new(repository)))
}

async fn root() -> &'static str {
    return "Hello World!";
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct CreateUser {
    username: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct User {
    id: u64,
    username: String,
}

#[cfg(test)]
mod test {
    use hyper::{Body, Request};
    use tower::ServiceExt;

    use super::*;

    #[tokio::test]
    async fn should_return_hello_world() {
        let req = Request::builder().uri("/").body(Body::empty()).unwrap();

        let repository = TodoRepositoryForMemory::new();

        let res = create_app(repository).oneshot(req).await.unwrap();

        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();

        let body = String::from_utf8(bytes.to_vec()).unwrap();

        assert_eq!(body, "Hello World!");
    }
}
