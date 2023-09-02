use crate::{
    db::establish_connection::establish_connection,
    models::{NewUser, User},
};
use diesel::prelude::*;

pub fn get_users(conn: &mut PgConnection) -> Vec<User> {
    use crate::schema::users::dsl::*;

    let result = users.load::<User>(conn).expect("Error loading users");

    result
}

pub fn add_user(user: &NewUser) -> Result<(), diesel::result::Error> {
    use crate::schema::users::dsl::*;

    let conn = &mut establish_connection();

    diesel::insert_into(users).values(user).execute(conn)?;

    Ok(())
}
