use actix_web::{post, HttpResponse, Responder};

use crate::models::NewUser;

#[post("/users/add")]
async fn add_user(req_body: String) -> impl Responder {
    let user: NewUser = match serde_json::from_str(&req_body) {
        Ok(user) => user,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    match crate::db::users::add_user(&user) {
        Ok(_) => HttpResponse::Ok().body("User registered correctly"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
