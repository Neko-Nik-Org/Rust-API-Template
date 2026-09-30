use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use super::service::AppError;
use serde::Serialize;


pub type ApiResponse = Result<HttpResponse, AppError>;


#[derive(Serialize)]
struct ErrorResp {
    error: String,
    code: u16
}


// ------- Implementations ------- //


impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::Redis(_) => StatusCode::BAD_GATEWAY,
            AppError::SerDe(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::Reqwest(_) => StatusCode::BAD_GATEWAY,
            AppError::Pgsql(_) => StatusCode::EXPECTATION_FAILED,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Unprocessable(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::PreconditionFailed(_) => StatusCode::PRECONDITION_FAILED,
            AppError::Gone(_) => StatusCode::GONE,
        }
    }

    fn error_response(&self) -> HttpResponse {
        log::error!("App error occurred: {}", self);

        HttpResponse::build(self.status_code())
            .json(ErrorResp {
                error: self.to_string(),
                code: self.status_code().as_u16()
            })
    }
}
