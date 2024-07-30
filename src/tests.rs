use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use std::sync::Arc;
use warp::test::request;

use crate::{db, filters, models};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

fn setup_test_db() -> Arc<db::DbPool> {
    let manager = ConnectionManager::<SqliteConnection>::new(":memory:");
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // Run migrations
    let conn = &mut pool.get().expect("Failed to get connection");
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");

    Arc::new(pool)
}

#[tokio::test]
async fn test_list_books() {
    let db_pool = setup_test_db();
    let api = filters::books(db_pool);

    let response = request().method("GET").path("/books").reply(&api).await;

    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_create_book() {
    let db_pool = setup_test_db();
    let api = filters::books(db_pool);

    let new_book = models::NewBook {
        title: "Test Book".to_string(),
        author: "Test Author".to_string(),
        date_published: "2024-07-30".to_string(),
        cover_image: "http://example.com/cover.jpg".to_string(),
    };

    let response = request()
        .method("POST")
        .path("/books")
        .json(&new_book)
        .reply(&api)
        .await;

    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_get_book() {
    let db_pool = setup_test_db();
    let api = filters::books(db_pool.clone());

    // First, create a book
    let new_book = models::NewBook {
        title: "Test Book".to_string(),
        author: "Test Author".to_string(),
        date_published: "2024-07-30".to_string(),
        cover_image: "http://example.com/cover.jpg".to_string(),
    };
    let book = db::create_book(&db_pool, new_book).unwrap();

    let book_id = book.id.expect("Book should have an ID");
    let response = request()
        .method("GET")
        .path(&format!("/books/{}", book_id))
        .reply(&api)
        .await;

    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_update_book() {
    let db_pool = setup_test_db();
    let api = filters::books(db_pool.clone());

    // First, create a book
    let new_book = models::NewBook {
        title: "Test Book".to_string(),
        author: "Test Author".to_string(),
        date_published: "2024-07-30".to_string(),
        cover_image: "http://example.com/cover.jpg".to_string(),
    };
    let book = db::create_book(&db_pool, new_book).unwrap();

    let book_id = book.id.expect("Book should have an ID");
    let updated_book = models::NewBook {
        title: "Updated Test Book".to_string(),
        author: "Updated Test Author".to_string(),
        date_published: "2023-01-02".to_string(),
        cover_image: "http://example.com/updated_cover.jpg".to_string(),
    };

    let response = request()
        .method("PUT")
        .path(&format!("/books/{}", book_id))
        .json(&updated_book)
        .reply(&api)
        .await;

    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_delete_book() {
    let db_pool = setup_test_db();
    let api = filters::books(db_pool.clone());

    // First, create a book
    let new_book = models::NewBook {
        title: "Test Book".to_string(),
        author: "Test Author".to_string(),
        date_published: "2024-07-30".to_string(),
        cover_image: "http://example.com/cover.jpg".to_string(),
    };
    let book = db::create_book(&db_pool, new_book).unwrap();

    let book_id = book.id.expect("Book should have an ID");
    let response = request()
        .method("DELETE")
        .path(&format!("/books/{}", book_id))
        .reply(&api)
        .await;

    assert_eq!(response.status(), 204);
}
