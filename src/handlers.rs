use std::sync::Arc;

use axum::{extract::Extension, response::IntoResponse, Json};
use hyper::StatusCode;

use crate::repositorys::{CreateTodo, TodoRepository};

pub async fn create_todo<T: TodoRepository>(
    Json(payload): Json<CreateTodo>,
    Extension(repository): Extension<Arc<T>>,
) -> impl IntoResponse {
    let todo = repository.create(payload);
    (StatusCode::CREATED, Json(todo))
}
