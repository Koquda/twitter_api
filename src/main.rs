use db::establish_connection::establish_connection;
use diesel::prelude::*;

use crate::models::Tweet;

pub mod db {
    pub mod establish_connection;
}
pub mod models;
pub mod schema;

fn main() {
    show_tweets();
}

fn show_tweets() {
    use self::schema::tweets::dsl::*;

    let connection = &mut establish_connection();
    let result = tweets
        .load::<Tweet>(connection)
        .expect("Error loading tweets");

    println!("Displaying {} tweets", result.len());
    for tweet in result {
        println!("{}", tweet.content);
    }
}
