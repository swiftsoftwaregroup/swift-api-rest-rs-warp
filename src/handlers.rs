use warp::{Reply, Rejection};
use crate::db;
use crate::models::NewBook;
use crate::errors::Error;
use std::sync::Arc;

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
    db::get_all_books(&db)
        .map(|books| warp::reply::json(&books))
        .map_err(|e| warp::reject::custom(Error::DatabaseError(e)))
}

#[utoipa::path(
    post,
    path = "/books",
    request_body = NewBook,
    responses(
        (status = 200, description = "Book created successfully", body = Book),
        (status = 400, description = "Invalid book data"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Books"
)]
pub async fn create_book(new_book: NewBook, db: Arc<db::DbPool>) -> Result<impl Reply, Rejection> {
    if new_book.title.is_empty() || new_book.author.is_empty() {
        return Err(warp::reject::custom(Error::InvalidData));
    }

    db::create_book(&db, new_book)
        .map(|book| warp::reply::json(&book))
        .map_err(|e| warp::reject::custom(Error::DatabaseError(e)))
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
    match db::get_book(&db, id) {
        Ok(book) => Ok(warp::reply::json(&book)),
        Err(diesel::result::Error::NotFound) => Err(warp::reject::custom(Error::NotFound)),
        Err(e) => Err(warp::reject::custom(Error::DatabaseError(e))),
    }
}

#[utoipa::path(
    put,
    path = "/books/{id}",
    request_body = NewBook,
    responses(
        (status = 200, description = "Book updated successfully", body = Book),
        (status = 400, description = "Invalid book data"),
        (status = 404, description = "Book not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("id" = i32, Path, description = "Book id")
    ),
    tag = "Books"
)]
pub async fn update_book(id: i32, updated_book: NewBook, db: Arc<db::DbPool>) -> Result<impl Reply, Rejection> {
    if updated_book.title.is_empty() || updated_book.author.is_empty() {
        return Err(warp::reject::custom(Error::InvalidData));
    }

    match db::update_book(&db, id, updated_book) {
        Ok(book) => Ok(warp::reply::json(&book)),
        Err(diesel::result::Error::NotFound) => Err(warp::reject::custom(Error::NotFound)),
        Err(e) => Err(warp::reject::custom(Error::DatabaseError(e))),
    }
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
    match db::delete_book(&db, id) {
        Ok(_) => Ok(warp::reply::with_status("Book deleted", warp::http::StatusCode::NO_CONTENT)),
        Err(diesel::result::Error::NotFound) => Err(warp::reject::custom(Error::NotFound)),
        Err(e) => Err(warp::reject::custom(Error::DatabaseError(e))),
    }
}
