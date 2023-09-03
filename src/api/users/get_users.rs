use actix_web::{get, HttpResponse, Responder};

use crate::db;

#[get("/users")]
async fn get_users() -> impl Responder {
    let users = db::users::get_users();

    HttpResponse::Ok().json(users)
}
