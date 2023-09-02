use actix_web::{post, web, HttpResponse, Responder};

use crate::{db, models::Tweet};

#[post("/tweets/delete/{id}")]
async fn delete_tweet(id: web::Path<i32>) -> impl Responder {
    let tweet: Tweet = match db::tweets::get_tweet_by_id(&id) {
        Some(tweet) => tweet,
        None => return HttpResponse::NotFound().body("Tweet not found"),
    };

    match crate::db::tweets::delete_tweet(&tweet) {
        Ok(_) => HttpResponse::Ok().body("Tweet deleted correctly"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
