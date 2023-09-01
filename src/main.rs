use self::models::{NewTweet, Tweet};
use db::establish_connection::establish_connection;
use diesel::prelude::*;

pub mod db {
    pub mod establish_connection;
}
pub mod models;
pub mod schema;

fn main() {
    let connection = &mut establish_connection();

    show_tweets(connection);
    create_tweet(connection, 1, "contenido del tweet");
}

fn show_tweets(conn: &mut PgConnection) {
    use self::schema::tweets::dsl::*;

    let result = tweets.load::<Tweet>(conn).expect("Error loading tweets");

    println!("Displaying {} tweets", result.len());
    for tweet in result {
        println!("{}", tweet.content);
    }
}

fn create_tweet(conn: &mut PgConnection, user_id: i32, content: &str) -> Tweet {
    use crate::schema::tweets;

    let new_tweet = NewTweet { user_id, content };

    diesel::insert_into(tweets::table)
        .values(&new_tweet)
        .returning(Tweet::as_returning())
        .get_result(conn)
        .expect("Error saving new tweet")
}
