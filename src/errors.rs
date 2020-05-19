use actix_web::error::ResponseError;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use serde::Serialize;
use std::fmt::{Display, Error, Formatter};
use std::result::Result;

#[derive(Debug)]
pub enum AppErrorType {
    DbError,
    NotFoundError,
}

#[derive(Debug)]
pub struct AppError {
    pub message: Option<String>,
    pub cause: Option<String>,
    pub error_type: AppErrorType,
}

#[derive(Serialize)]
pub struct AppErrorResponse {
    pub error: String,
}

impl AppError {
    fn message(&self) -> String {
        match &*self {
            AppError {
                message: Some(message),
                cause: _,
                error_type: _,
            } => message.clone(),
            AppError {
                message: None,
                cause: _,
                error_type: AppErrorType::DbError,
            } => "Requested item not found".to_string(),
            _ => "Unexpected error".to_string(),
        }
    }

    pub fn db_error(err: impl ToString) -> AppError {
        AppError {
            message: None,
            cause: Some(err.to_string()),
            error_type: AppErrorType::DbError,
        }
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self.error_type {
            AppErrorType::DbError => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::NotFoundError => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(AppErrorResponse {
            error: self.message(),
        })
    }
}
