use actix_web::{get, HttpResponse, Responder};

use crate::db;

#[get("/tweets")]
async fn get_tweets() -> impl Responder {
    let tweets = db::tweets::get_tweets();

    HttpResponse::Ok().json(tweets)
}
