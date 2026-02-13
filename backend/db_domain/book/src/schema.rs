// @generated automatically by Diesel CLI.

diesel::table! {
    books (id) {
        id -> Int4,
        title -> Text,
        division_id -> Int4,
        borrowed_by_user_id -> Nullable<Int4>,
        borrowed_at -> Nullable<Timestamptz>,
    }
}
