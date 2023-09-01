use actix_web::{post, HttpResponse, Responder};

use crate::{db::establish_connection::establish_connection, models::NewTweet};

#[post("/tweets/add")]
async fn add_tweet(req_body: String) -> impl Responder {
    let tweet: NewTweet = match serde_json::from_str(&req_body) {
        Ok(tweet) => tweet,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let connection = &mut establish_connection();
    match crate::db::tweets::add_tweet(connection, &tweet) {
        Ok(_) => HttpResponse::Ok().body("Tweet added correctly"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
