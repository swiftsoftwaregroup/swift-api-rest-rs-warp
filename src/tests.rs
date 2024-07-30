use actix_web::{test, web, App};

use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use serde_json::json;

use crate::{create_book, db, delete_book, get_all_books, get_book, models, update_book};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

fn setup_test_db() -> db::DbPool {
    let manager = ConnectionManager::<SqliteConnection>::new(":memory:");
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // Run migrations
    let conn = &mut pool.get().expect("Failed to get connection");
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");

    pool
}

#[actix_web::test]
async fn test_create_book() {
    let pool = setup_test_db();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/books", web::post().to(create_book)),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/books")
        .set_json(json!({
            "title": "Test Book",
            "author": "Test Author",
            "date_published": "2023-01-01",
            "cover_image": "http://example.com/cover.jpg"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_get_all_books() {
    let pool = setup_test_db();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/books", web::get().to(get_all_books)),
    )
    .await;

    let req = test::TestRequest::get().uri("/books").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_get_book() {
    let pool = setup_test_db();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/books", web::post().to(create_book))
            .route("/books/{id}", web::get().to(get_book)),
    )
    .await;

    // First, create a book
    let create_req = test::TestRequest::post()
        .uri("/books")
        .set_json(json!({
            "title": "Test Book",
            "author": "Test Author",
            "date_published": "2023-01-01",
            "cover_image": "http://example.com/cover.jpg"
        }))
        .to_request();

    let create_resp: models::Book = test::call_and_read_body_json(&app, create_req).await;

    // Then, get the created book
    let get_req = test::TestRequest::get()
        .uri(&format!("/books/{}", create_resp.id.unwrap()))
        .to_request();

    let resp = test::call_service(&app, get_req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_update_book() {
    let pool = setup_test_db();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/books", web::post().to(create_book))
            .route("/books/{id}", web::put().to(update_book)),
    )
    .await;

    // First, create a book
    let create_req = test::TestRequest::post()
        .uri("/books")
        .set_json(json!({
            "title": "Test Book",
            "author": "Test Author",
            "date_published": "2023-01-01",
            "cover_image": "http://example.com/cover.jpg"
        }))
        .to_request();

    let create_resp: models::Book = test::call_and_read_body_json(&app, create_req).await;

    // Then, update the created book
    let update_req = test::TestRequest::put()
        .uri(&format!("/books/{}", create_resp.id.unwrap()))
        .set_json(json!({
            "title": "Updated Test Book",
            "author": "Updated Test Author",
            "date_published": "2023-01-02",
            "cover_image": "http://example.com/updated_cover.jpg"
        }))
        .to_request();

    let resp = test::call_service(&app, update_req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_delete_book() {
    let pool = setup_test_db();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/books", web::post().to(create_book))
            .route("/books/{id}", web::delete().to(delete_book)),
    )
    .await;

    // First, create a book
    let create_req = test::TestRequest::post()
        .uri("/books")
        .set_json(json!({
            "title": "Test Book",
            "author": "Test Author",
            "date_published": "2023-01-01",
            "cover_image": "http://example.com/cover.jpg"
        }))
        .to_request();

    let create_resp: models::Book = test::call_and_read_body_json(&app, create_req).await;

    // Then, delete the created book
    let delete_req = test::TestRequest::delete()
        .uri(&format!("/books/{}", create_resp.id.unwrap()))
        .to_request();

    let resp = test::call_service(&app, delete_req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::NO_CONTENT);
}
