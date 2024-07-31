use thiserror::Error;
use warp::reject::Reject;

#[derive(Error, Debug)]
pub enum Error {
    #[error("database error: {0}")]
    DatabaseError(#[from] diesel::result::Error),
    #[error("not found")]
    NotFound,
    #[error("invalid data")]
    InvalidData,
}

impl Reject for Error {}

pub async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(error) = err.find::<Error>() {
        let (code, message) = match error {
            Error::DatabaseError(_) => (
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error",
            ),
            Error::NotFound => (warp::http::StatusCode::NOT_FOUND, "Not Found"),
            Error::InvalidData => (warp::http::StatusCode::BAD_REQUEST, "Invalid Data"),
        };
        Ok(warp::reply::with_status(message.to_string(), code))
    } else if err.is_not_found() {
        Ok(warp::reply::with_status(
            "Not Found".to_string(),
            warp::http::StatusCode::NOT_FOUND,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Internal Server Error".to_string(),
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}
