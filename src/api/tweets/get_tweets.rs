use actix_web::{get, HttpResponse, Responder};

use crate::db::{self, establish_connection::establish_connection};

#[get("/tweets")]
async fn get_tweets() -> impl Responder {
    let connection = &mut establish_connection();

    let tweets = db::tweets::get_tweets(connection);

    HttpResponse::Ok().json(tweets)
}
