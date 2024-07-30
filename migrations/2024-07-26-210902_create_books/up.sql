-- This file is used to create the table books in the database.
CREATE TABLE books (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  title TEXT NOT NULL,
  author TEXT NOT NULL,
  date_published TEXT NOT NULL,
  cover_image TEXT NOT NULL
);
