use actix_web::{App, HttpServer};

use api::tweets::add_tweet::add_tweet;
use api::tweets::delete_tweet::delete_tweet;
use api::tweets::get_tweets::get_tweets;
use api::tweets::get_tweets_by_user::get_tweets_by_user;
use api::tweets::update_tweet::update_tweet;
use api::users::add_user::add_user;
use api::users::get_users::get_users;
use api::users::update_user::update_user;

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
        pub mod add_user;
        pub mod get_users;
        pub mod update_user;
    }
}
pub mod models;
pub mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_tweets)
            .service(get_tweets_by_user)
            .service(add_tweet)
            .service(get_users)
            .service(delete_tweet)
            .service(update_tweet)
            .service(add_user)
            .service(update_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
