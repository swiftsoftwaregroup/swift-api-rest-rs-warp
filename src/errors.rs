use std::convert::Infallible;
use thiserror::Error;
use warp::{http::StatusCode, Rejection, Reply};

#[derive(Error, Debug)]
pub enum Error {
    #[error("database error: {0}")]
    DatabaseError(#[from] diesel::result::Error),
    #[error("not found")]
    NotFound,
}

impl warp::reject::Reject for Error {}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "Not Found";
    } else if let Some(error) = err.find::<Error>() {
        match error {
            Error::DatabaseError(_) => {
                code = StatusCode::INTERNAL_SERVER_ERROR;
                message = "Internal Server Error";
            }
            Error::NotFound => {
                code = StatusCode::NOT_FOUND;
                message = "Not Found";
            }
        }
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "Method Not Allowed";
    } else {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Internal Server Error";
    }

    Ok(warp::reply::with_status(message.to_string(), code))
}
