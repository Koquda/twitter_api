use actix_web::{get, web, HttpResponse, Responder};

use crate::db;

#[get("/tweets/{user_id}")]
async fn get_tweets_by_user(user_id: web::Path<i32>) -> impl Responder {
    let tweets = db::tweets::get_tweets_by_user(user_id.into_inner());

    HttpResponse::Ok().json(tweets)
}
