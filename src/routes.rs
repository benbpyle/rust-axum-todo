use crate::data::models::{
    ApiError, AppState, Todo, TodoCreate, TodoDeleteView, TodoUpdate, TodoView,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

/// create_todo handles the POST route / at the root level
pub async fn create_todo(
    State(state): State<AppState>,
    Json(payload): Json<TodoCreate>,
) -> Result<Json<Todo>, ApiError> {
    let r = state.todo_service.create_todo(&payload.description).await?;

    Ok(r.into())
}

/// find_todo_by_id handles the GET route /:id at the todo level
pub async fn find_todo_by_id(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<TodoView>, ApiError> {
    let r = state.todo_service.find_todo_by_id(&id).await?;
    let t: TodoView = r.into();
    Ok(Json(t))
}

/// delete_todo_by_id handles the DELETE route /:id at the todo level
pub async fn delete_todo_by_id(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<TodoDeleteView>, ApiError> {
    let r = state.todo_service.delete_to_by_id(&id).await?;
    let t: String = r.into();
    Ok(Json(TodoDeleteView { todo_id: t }))
}

/// update_todo_by_id handles the PUT route /:id at the todo level
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

/// health handles the GET route /health at the root level
pub async fn health() -> impl IntoResponse {
    (StatusCode::OK, Json("Healthy".to_string()))
}

/// handler_404 handles the 404 route at the root level
pub async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, Json("Path not found"))
}
