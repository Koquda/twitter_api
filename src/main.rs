use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};

use api::tweets::add_tweet::add_tweet;
use api::tweets::delete_tweet::delete_tweet;
use api::tweets::get_tweets::get_tweets;
use api::tweets::get_tweets_by_user::get_tweets_by_user;
use api::tweets::update_tweet::update_tweet;
use api::users::get_users::get_users;

pub mod db {
    pub mod establish_connection;
    pub mod tweets;
    pub mod users;
}
pub mod api {
    pub mod tweets {
        pub mod add_tweet;
        pub mod delete_tweet;
        pub mod get_tweets;
        pub mod get_tweets_by_user;
        pub mod update_tweet;
    }
    pub mod users {
        pub mod get_users;
    }
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
            .service(get_users)
            .service(delete_tweet)
            .service(update_tweet)
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
