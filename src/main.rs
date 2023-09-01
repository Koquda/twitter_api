use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};

use api::add_tweet::add_tweet;
use api::get_tweets::get_tweets;
use api::get_tweets_by_user::get_tweets_by_user;

pub mod db {
    pub mod establish_connection;
    pub mod tweets;
}
pub mod api {
    pub mod add_tweet;
    pub mod get_tweets;
    pub mod get_tweets_by_user;
}
pub mod models;
pub mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(get_tweets)
            .service(get_tweets_by_user)
            .service(add_tweet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
