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

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = tweets)]
pub struct NewTweet<'a> {
    pub user_id: i32,
    pub content: &'a str,
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

#[derive(AsChangeset, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateUser<'a> {
    pub id: i32,
    pub username: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}

impl User {
    pub fn update_user_data(&mut self, user: UpdateUser) {
        self.username = user.username.to_string();
        self.email = user.email.to_string();
        self.password = user.password.to_string();
    }
}
