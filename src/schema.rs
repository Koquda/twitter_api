// @generated automatically by Diesel CLI.

diesel::table! {
    tweets (id) {
        id -> Int4,
        user_id -> Int4,
        content -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    tweets,
    users,
);
