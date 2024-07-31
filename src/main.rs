#[cfg(test)]
mod tests;

mod db;
mod errors;
mod handlers;
mod models;
mod schema;

use models::{Book, NewBook};
use std::sync::Arc;
use warp::Filter;

use utoipa::OpenApi;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::list_books,
        crate::handlers::create_book,
        crate::handlers::get_book,
        crate::handlers::update_book,
        crate::handlers::delete_book
    ),
    components(
        schemas(Book, NewBook)
    ),
    tags(
        (name = "Books", description = "Book management operations")
    ),
    info(
        title = "Book Management API",
        version = "1.0.0",
        description = "A simple API for managing books"
    )
)]
struct ApiDocs;

#[tokio::main]
async fn main() {
    let pool = db::establish_connection();

    // Run migrations
    pool.get()
        .expect("Failed to get DB connection")
        .run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");

    let pool = Arc::new(pool);

    let api = filters::books(pool);
    
    let api_docs = warp::path("openapi.json")
        .and(warp::get())
        .map(|| warp::reply::json(&ApiDocs::openapi()));

    let swagger_ui = warp::path("docs")
        .and(warp::get())
        .and(warp::path::end())
        .map(|| {
            warp::reply::html(r#"
                <!DOCTYPE html>
                <html lang="en">
                <head>
                    <meta charset="utf-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1" />
                    <meta name="description" content="SwaggerUI" />
                    <title>SwaggerUI</title>
                    <link rel="stylesheet" href="https://unpkg.com/swagger-ui-dist@5.17.14/swagger-ui.css" />
                </head>
                <body>
                    <div id="swagger-ui"></div>
                    <script src="https://unpkg.com/swagger-ui-dist@5.17.14/swagger-ui-bundle.js" crossorigin></script>
                    <script>
                        window.onload = () => {
                            window.ui = SwaggerUIBundle({
                                url: '/openapi.json',
                                dom_id: '#swagger-ui',
                            });
                        };
                    </script>
                </body>
                </html>
                "#.to_string())
        });

    let routes = api
        .or(api_docs)
        .or(swagger_ui)
        .with(warp::cors().allow_any_origin())
        .recover(errors::handle_rejection);

    println!("Server started at http://localhost:8001");
    println!("API documentation available at http://localhost:8001/docs/");
    warp::serve(routes).run(([0, 0, 0, 0], 8001)).await;
}

mod filters {
    use super::*;
    use crate::handlers;
    use std::sync::Arc;
    use warp::{Filter, Rejection, Reply};

    pub fn books(
        db: Arc<db::DbPool>,
    ) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        get_books(db.clone())
            .or(get_book(db.clone()))
            .or(create_book(db.clone()))
            .or(update_book(db.clone()))
            .or(delete_book(db))
    }

    pub fn get_books(
        db: Arc<db::DbPool>,
    ) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        warp::path("books")
            // Careful! Omitting the following line would make this filter match requests to /books/:i32 as well.
            .and(warp::path::end())        
            .and(warp::get())
            .and(with_db(db))
            .and_then(handlers::list_books)
    }

    pub fn create_book(
        db: Arc<db::DbPool>,
    ) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        warp::path("books")
            .and(warp::post())
            .and(warp::body::json())
            .and(with_db(db))
            .and_then(handlers::create_book)
    }

    pub fn get_book(
        db: Arc<db::DbPool>,
    ) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        warp::path!("books" / i32)
            .and(warp::get())
            .and(with_db(db))
            .and_then(handlers::get_book)
    }

    pub fn update_book(
        db: Arc<db::DbPool>,
    ) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        warp::path!("books" / i32)
            .and(warp::put())
            .and(warp::body::json())
            .and(with_db(db))
            .and_then(handlers::update_book)
    }

    pub fn delete_book(
        db: Arc<db::DbPool>,
    ) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        warp::path!("books" / i32)
            .and(warp::delete())
            .and(with_db(db))
            .and_then(handlers::delete_book)
    }

    fn with_db(
        db: Arc<db::DbPool>,
    ) -> impl Filter<Extract = (Arc<db::DbPool>,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || db.clone())
    }
}
