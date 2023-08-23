use rocket::http::Status;
use rocket::response;
use rocket::response::Responder;
use rocket::serde::json::Json;
use rocket::Request;
use rocket::Response;

use crate::messages::ErrorResponse;

pub enum ApiError {
    Database(String),
    NotFound,
    OpenAPI(String),
}

impl<'r, 'o: 'r> Responder<'r, 'o> for ApiError {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        let (error_message, status) = match self {
            ApiError::NotFound => ("no record with that id".into(), Status::NotFound),
            ApiError::Database(m) => (m, Status::InternalServerError),
            ApiError::OpenAPI(m) => (m, Status::InternalServerError),
        };
        Response::build_from(Json(ErrorResponse { error_message }).respond_to(req)?)
            .status(status)
            .ok()
    }
}

impl From<diesel::result::Error> for ApiError {
    fn from(value: diesel::result::Error) -> Self {
        match value {
            diesel::result::Error::NotFound => Self::NotFound,
            e => Self::Database(format!("Database Error: {}", e)),
        }
    }
}

impl From<openai_api_rust::Error> for ApiError {
    fn from(value: openai_api_rust::Error) -> Self {
        Self::OpenAPI(format!("OpenAPI Error: {}", value))
    }
}
