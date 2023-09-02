use crate::schema::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(AsChangeset, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = tweets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Tweet {
    pub id: i32,
    pub user_id: i32,
    pub content: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = tweets)]
pub struct NewTweet<'a> {
    pub user_id: i32,
    pub content: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateTweet<'a> {
    pub id: i32,
    pub content: &'a str,
}

impl Tweet {
    pub fn set_content(&mut self, content: &str) {
        self.content = content.to_string();
    }
}
