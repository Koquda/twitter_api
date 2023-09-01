use crate::schema::*;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = tweets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Tweet {
    pub id: i32,
    pub user_id: i32,
    pub content: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
}
