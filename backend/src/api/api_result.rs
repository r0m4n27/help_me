use rocket::serde::json::Json;

use crate::models::QueriesError;

#[derive(Debug, Responder)]
pub enum ApiError {
    #[response(status = 500, content_type = "json")]
    Database(Json<ErrorMessage>),
    #[response(status = 404, content_type = "json")]
    NotFound(Json<ErrorMessage>),
    #[response(status = 400, content_type = "json")]
    BadRequest(Json<ErrorMessage>),
}

impl From<QueriesError> for ApiError {
    fn from(err: QueriesError) -> Self {
        match err {
            QueriesError::Database(err) => {
                error!("Db error: {}", err);

                ApiError::Database(ErrorMessage::new(
                    "Database couldn't handle request!".to_string(),
                ))
            }
            // Thiserror just forwards the internal error
            // so we don't need to call err.to_string()
            QueriesError::ItemNotFound(err) => ApiError::NotFound(ErrorMessage::new(err)),
            QueriesError::IllegalState(err) => ApiError::BadRequest(ErrorMessage::new(err)),
        }
    }
}
#[derive(Debug, Serialize)]
pub struct ErrorMessage {
    message: String,
}

impl ErrorMessage {
    pub fn new(message: String) -> Json<Self> {
        Json(ErrorMessage { message })
    }
}

pub type ApiResult<T> = Result<Json<T>, ApiError>;

// A normal function has to be used
// because an impl block can't be used for a type outside of it's crate
pub fn ok<T, R: From<T>>(data: T) -> ApiResult<R> {
    ApiResult::Ok(Json(R::from(data)))
}
