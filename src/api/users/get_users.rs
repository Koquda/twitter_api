use actix_web::{get, HttpResponse, Responder};

use crate::db::{self, establish_connection::establish_connection};

#[get("/users")]
async fn get_users() -> impl Responder {
    let connection = &mut establish_connection();

    let users = db::users::get_users(connection);

    HttpResponse::Ok().json(users)
}
