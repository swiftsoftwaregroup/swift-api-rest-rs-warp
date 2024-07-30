use crate::schema::books;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Queryable, Serialize, Deserialize, ToSchema)]
pub struct Book {
    #[schema(example = 1)]
    pub id: Option<i32>,
    #[schema(example = "The Rust Programming Language")]
    pub title: String,
    #[schema(example = "Steve Klabnik and Carol Nichols")]
    pub author: String,
    #[schema(example = "2018-08-12")]
    pub date_published: String,
    #[schema(example = "https://example.com/book-cover.jpg")]
    pub cover_image: String,
}

#[derive(Insertable, Serialize, Deserialize, ToSchema)]
#[diesel(table_name = books)]
pub struct NewBook {
    #[schema(example = "The Rust Programming Language")]
    pub title: String,
    #[schema(example = "Steve Klabnik and Carol Nichols")]
    pub author: String,
    #[schema(example = "2018-08-12")]
    pub date_published: String,
    #[schema(example = "https://example.com/book-cover.jpg")]
    pub cover_image: String,
}
