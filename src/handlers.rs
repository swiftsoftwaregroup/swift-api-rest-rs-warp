use crate::db;
use crate::errors::Error;
use crate::models::NewBook;
use std::sync::Arc;
use warp::{Rejection, Reply};

#[utoipa::path(
    get,
    path = "/books",
    responses(
        (status = 200, description = "List of all books", body = Vec<Book>),
        (status = 500, description = "Internal server error")
    ),
    tag = "Books"
)]
pub async fn list_books(db: Arc<db::DbPool>) -> Result<impl Reply, Rejection> {
    let books =
        db::get_all_books(&db).map_err(|e| warp::reject::custom(Error::DatabaseError(e)))?;
    Ok(warp::reply::json(&books))
}

#[utoipa::path(
    post,
    path = "/books",
    request_body = NewBook,
    responses(
        (status = 200, description = "Book created successfully", body = Book),
        (status = 500, description = "Internal server error")
    ),
    tag = "Books"
)]
pub async fn create_book(new_book: NewBook, db: Arc<db::DbPool>) -> Result<impl Reply, Rejection> {
    let book = db::create_book(&db, new_book)
        .map_err(|e| warp::reject::custom(Error::DatabaseError(e)))?;
    Ok(warp::reply::json(&book))
}

#[utoipa::path(
    get,
    path = "/books/{id}",
    responses(
        (status = 200, description = "Book found", body = Book),
        (status = 404, description = "Book not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("id" = i32, Path, description = "Book id")
    ),
    tag = "Books"
)]
pub async fn get_book(id: i32, db: Arc<db::DbPool>) -> Result<impl Reply, Rejection> {
    let book = db::get_book(&db, id).map_err(|e| match e {
        diesel::result::Error::NotFound => warp::reject::custom(Error::NotFound),
        _ => warp::reject::custom(Error::DatabaseError(e)),
    })?;
    Ok(warp::reply::json(&book))
}

#[utoipa::path(
    put,
    path = "/books/{id}",
    request_body = NewBook,
    responses(
        (status = 200, description = "Book updated successfully", body = Book),
        (status = 404, description = "Book not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("id" = i32, Path, description = "Book id")
    ),
    tag = "Books"
)]
pub async fn update_book(
    id: i32,
    updated_book: NewBook,
    db: Arc<db::DbPool>,
) -> Result<impl Reply, Rejection> {
    let book = db::update_book(&db, id, updated_book).map_err(|e| match e {
        diesel::result::Error::NotFound => warp::reject::custom(Error::NotFound),
        _ => warp::reject::custom(Error::DatabaseError(e)),
    })?;
    Ok(warp::reply::json(&book))
}

#[utoipa::path(
    delete,
    path = "/books/{id}",
    responses(
        (status = 204, description = "Book deleted successfully"),
        (status = 404, description = "Book not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("id" = i32, Path, description = "Book id")
    ),
    tag = "Books"
)]
pub async fn delete_book(id: i32, db: Arc<db::DbPool>) -> Result<impl Reply, Rejection> {
    db::delete_book(&db, id).map_err(|e| match e {
        diesel::result::Error::NotFound => warp::reject::custom(Error::NotFound),
        _ => warp::reject::custom(Error::DatabaseError(e)),
    })?;
    Ok(warp::reply::with_status(
        "Book deleted",
        warp::http::StatusCode::NO_CONTENT,
    ))
}
