use actix_web::{post, web, HttpResponse, Responder};

use crate::{db, models::User};

#[post("/users/delete/{id}")]
async fn delete_user(id: web::Path<i32>) -> impl Responder {
    let user: User = match db::users::get_user_by_id(&id) {
        Some(user) => user,
        None => return HttpResponse::NotFound().body("User not found"),
    };

    match crate::db::users::delete_user(&user) {
        Ok(_) => HttpResponse::Ok().body("User deleted correctly"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
