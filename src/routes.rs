use crate::data::models::{
    ApiError, AppState, Todo, TodoCreate, TodoDeleteView, TodoUpdate, TodoView,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

pub async fn create_todo(
    State(state): State<AppState>,
    Json(payload): Json<TodoCreate>,
) -> Result<Json<Todo>, ApiError> {
    let r = state.todo_service.create_todo(&payload.description).await?;

    Ok(r.into())
}

pub async fn find_todo_by_id(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<TodoView>, ApiError> {
    let r = state.todo_service.find_todo_by_id(&id).await?;
    let t: TodoView = r.into();
    Ok(Json(t))
}

pub async fn delete_todo_by_id(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<TodoDeleteView>, ApiError> {
    let r = state.todo_service.delete_to_by_id(&id).await?;
    let t: String = r.into();
    Ok(Json(TodoDeleteView { todo_id: t }))
}

pub async fn update_todo_by_id(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<TodoUpdate>,
) -> Result<Json<TodoView>, ApiError> {
    println!("(ID)={}", id);
    let r = state
        .todo_service
        .update_todo_by_id(&id, &payload.description)
        .await?;

    let t: TodoView = r.into();
    Ok(Json(t))
}

pub async fn health() -> impl IntoResponse {
    (StatusCode::OK, Json("Healthy".to_string()))
}

pub async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, Json("Path not found"))
}
