use crate::models::{NewTweet, Tweet};
use diesel::prelude::*;

pub fn get_tweets(conn: &mut PgConnection) -> Vec<Tweet> {
    use crate::schema::tweets::dsl::*;

    let result = tweets.load::<Tweet>(conn).expect("Error loading tweets");

    result
}

pub fn get_tweets_by_user(conn: &mut PgConnection, id_user: i32) -> Vec<Tweet> {
    use crate::schema::tweets::dsl::*;

    let result = tweets
        .filter(user_id.eq(id_user))
        .load::<Tweet>(conn)
        .expect("Error loading tweets");

    result
}

pub fn create_tweet(conn: &mut PgConnection, user_id: i32, content: &str) -> Tweet {
    use crate::schema::tweets;

    let new_tweet = NewTweet { user_id, content };

    diesel::insert_into(tweets::table)
        .values(&new_tweet)
        .returning(Tweet::as_returning())
        .get_result(conn)
        .expect("Error saving new tweet")
}

pub fn add_tweet(
    connection: &mut PgConnection,
    new_tweet: &NewTweet,
) -> Result<(), diesel::result::Error> {
    use crate::schema::tweets::dsl::*;

    diesel::insert_into(tweets)
        .values(new_tweet)
        .execute(connection)?;

    Ok(())
}
