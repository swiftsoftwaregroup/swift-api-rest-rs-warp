use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;

use crate::models::{Book, NewBook};

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn establish_connection() -> DbPool {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| ":memory:".to_string());
    create_connection_pool(&database_url)
}

pub fn create_connection_pool(database_url: &str) -> DbPool {
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}

pub fn create_book(pool: &DbPool, new_book: NewBook) -> QueryResult<Book> {
    use crate::schema::books::dsl::*;
    let conn = &mut pool.get().unwrap();
    diesel::insert_into(books).values(&new_book).execute(conn)?;

    books.order(id.desc()).first(conn)
}

pub fn get_all_books(pool: &DbPool) -> QueryResult<Vec<Book>> {
    use crate::schema::books::dsl::*;
    let conn = &mut pool.get().unwrap();

    books.load::<Book>(conn)
}

pub fn get_book(pool: &DbPool, book_id: i32) -> QueryResult<Book> {
    use crate::schema::books::dsl::*;
    let conn = &mut pool.get().unwrap();

    books.filter(id.eq(Some(book_id))).first(conn)
}

pub fn update_book(pool: &DbPool, book_id: i32, updated_book: NewBook) -> QueryResult<Book> {
    use crate::schema::books::dsl::*;
    let conn = &mut pool.get().unwrap();
    diesel::update(books.filter(id.eq(Some(book_id))))
        .set((
            title.eq(updated_book.title),
            author.eq(updated_book.author),
            date_published.eq(updated_book.date_published),
            cover_image.eq(updated_book.cover_image),
        ))
        .execute(conn)?;

    books.filter(id.eq(Some(book_id))).first(conn)
}

pub fn delete_book(pool: &DbPool, book_id: i32) -> QueryResult<usize> {
    use crate::schema::books::dsl::*;
    let conn = &mut pool.get().unwrap();
    diesel::delete(books.filter(id.eq(Some(book_id)))).execute(conn)
}
