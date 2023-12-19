// use aws_sdk_dynamodb::{error::SdkError, types::AttributeValue};
use aws_sdk_dynamodb::error::SdkError;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;

use super::services::services::TodoService;

#[derive(Clone)]
pub struct AppState {
    pub todo_service: TodoService,
}

pub enum ApiError {
    DynamoDBError(aws_sdk_dynamodb::Error),
    ItemNotFound(DbError),
    Other(DbError),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub id: String,
    pub todo_id: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TodoView {
    pub todo_id: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TodoDeleteView {
    pub todo_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoCreate {
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoUpdate {
    pub description: String,
}

#[derive(Error, Debug)]
pub enum DbError {
    #[error("failed to parse response into items: {0}")]
    FromSerde(serde_dynamo::Error),

    #[error("aws_sdk_dynamodb error: {0}")]
    Dynamo(aws_sdk_dynamodb::Error),

    #[error("item not found")]
    NotFound,

    #[error("DynamoDB item error: {0}")]
    OperationError(String),
}

impl From<aws_sdk_dynamodb::Error> for ApiError {
    fn from(inner: aws_sdk_dynamodb::Error) -> Self {
        ApiError::DynamoDBError(inner)
    }
}

impl From<DbError> for ApiError {
    fn from(inner: DbError) -> Self {
        match inner {
            DbError::NotFound => ApiError::ItemNotFound(inner),
            DbError::OperationError(error) => ApiError::Other(DbError::OperationError(error)),
            DbError::FromSerde(error) => ApiError::Other(DbError::FromSerde(error)),
            DbError::Dynamo(error) => ApiError::Other(DbError::Dynamo(error)),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::DynamoDBError(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                std::format!("(Error)={:?}", error),
            ),
            ApiError::Other(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                std::format!("(Error)={:?}", error),
            ),

            ApiError::ItemNotFound(error) => {
                (StatusCode::NOT_FOUND, std::format!("(Error)={:?}", error))
            }
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

impl From<aws_sdk_dynamodb::Error> for DbError {
    fn from(err: aws_sdk_dynamodb::Error) -> Self {
        DbError::Dynamo(err)
    }
}

impl From<serde_dynamo::Error> for DbError {
    fn from(err: serde_dynamo::Error) -> Self {
        DbError::FromSerde(err)
    }
}

impl<E, R> From<SdkError<E, R>> for DbError
where
    E: std::fmt::Debug,
    R: std::fmt::Debug,
{
    fn from(err: SdkError<E, R>) -> Self {
        DbError::OperationError(format!("{err:?}"))
    }
}

impl From<Todo> for TodoView {
    fn from(todo: Todo) -> Self {
        TodoView {
            todo_id: todo.id,
            description: todo.description,
        }
    }
}
