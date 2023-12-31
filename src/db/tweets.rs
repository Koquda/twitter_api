use crate::{
    db::establish_connection::PoolWrapper,
    models::{NewTweet, Tweet},
};
use diesel::prelude::*;

pub fn get_tweets() -> Vec<Tweet> {
    use crate::schema::tweets::dsl::*;

    let conn = &mut PoolWrapper::get_instance().get().unwrap();

    let result = tweets.load::<Tweet>(conn).expect("Error loading tweets");

    result
}

pub fn get_tweets_by_user(id_user: i32) -> Vec<Tweet> {
    use crate::schema::tweets::dsl::*;

    let conn = &mut PoolWrapper::get_instance().get().unwrap();

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

pub fn add_tweet(new_tweet: &NewTweet) -> Result<(), diesel::result::Error> {
    use crate::schema::tweets::dsl::*;

    let connection = &mut PoolWrapper::get_instance().get().unwrap();

    diesel::insert_into(tweets)
        .values(new_tweet)
        .execute(connection)?;

    Ok(())
}

pub fn get_tweet_by_id(tweet_id: &i32) -> Option<Tweet> {
    use crate::schema::tweets::dsl::*;

    let connection = &mut PoolWrapper::get_instance().get().unwrap();
    let result = tweets.filter(id.eq(tweet_id)).load::<Tweet>(connection);

    match result {
        Ok(mut tweets_vec) => tweets_vec.pop(),
        Err(_) => None,
    }
}

pub fn delete_tweet(tweet: &Tweet) -> Result<(), diesel::result::Error> {
    use crate::schema::tweets::dsl::*;

    let connection = &mut PoolWrapper::get_instance().get().unwrap();
    diesel::delete(tweets.filter(id.eq(tweet.id))).execute(connection)?;

    Ok(())
}

pub fn update_tweet(tweet_to_update: &Tweet) -> Result<(), diesel::result::Error> {
    use crate::schema::tweets::dsl::*;

    let connection = &mut PoolWrapper::get_instance().get().unwrap();
    diesel::update(tweets.filter(id.eq(tweet_to_update.id)))
        .set(tweet_to_update)
        .execute(connection)?;

    Ok(())
}
