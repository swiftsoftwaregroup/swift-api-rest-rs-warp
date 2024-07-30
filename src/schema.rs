// @generated automatically by Diesel CLI.

diesel::table! {
    books (id) {
        id -> Nullable<Integer>,
        title -> Text,
        author -> Text,
        date_published -> Text,
        cover_image -> Text,
    }
}
