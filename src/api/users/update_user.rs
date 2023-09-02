use actix_web::{post, HttpResponse, Responder};

use crate::{
    db::{self, users::get_user_by_id},
    models::UpdateUser,
};

#[post("/users/update")]
async fn update_user(req_body: String) -> impl Responder {
    let user: UpdateUser = match serde_json::from_str(&req_body) {
        Ok(user) => user,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let mut user_to_update = match get_user_by_id(&user.id) {
        Some(user) => user,
        None => return HttpResponse::NotFound().body("User not found, check if it exists"),
    };

    user_to_update.update_user_data(user);

    match db::users::update_user(&user_to_update) {
        Ok(_) => HttpResponse::Ok().json(user_to_update),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
