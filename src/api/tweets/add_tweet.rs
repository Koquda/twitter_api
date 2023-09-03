use actix_web::{post, HttpResponse, Responder};

use crate::models::NewTweet;

#[post("/tweets/add")]
async fn add_tweet(req_body: String) -> impl Responder {
    let tweet: NewTweet = match serde_json::from_str(&req_body) {
        Ok(tweet) => tweet,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    match crate::db::tweets::add_tweet(&tweet) {
        Ok(_) => HttpResponse::Ok().body("Tweet added correctly"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
