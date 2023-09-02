use actix_web::{post, HttpResponse, Responder};

use crate::{
    db::{self, tweets::get_tweet_by_id},
    models::UpdateTweet,
};

#[post("/tweets/update")]
async fn update_tweet(req_body: String) -> impl Responder {
    let tweet: UpdateTweet = match serde_json::from_str(&req_body) {
        Ok(tweet) => tweet,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let mut tweet_to_update = match get_tweet_by_id(&tweet.id) {
        Some(tweet) => tweet,
        None => return HttpResponse::NotFound().body("Tweet not found"),
    };

    tweet_to_update.set_content(tweet.content);

    match db::tweets::update_tweet(&tweet_to_update) {
        Ok(_) => HttpResponse::Ok().json(tweet_to_update),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
