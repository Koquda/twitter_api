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

pub fn get_user_by_id(user_id: &i32) -> Option<User> {
    use crate::schema::users::dsl::*;

    let conn = &mut establish_connection();

    let result = users
        .filter(id.eq(user_id))
        .first::<User>(conn)
        .optional()
        .expect("Error loading user");

    result
}

pub fn update_user(user: &User) -> Result<(), diesel::result::Error> {
    use crate::schema::users::dsl::*;

    let conn = &mut establish_connection();

    diesel::update(users.filter(id.eq(&user.id)))
        .set(user)
        .execute(conn)?;

    Ok(())
}

pub fn delete_user(user: &User) -> Result<(), diesel::result::Error> {
    use crate::schema::users::dsl::*;

    let conn = &mut establish_connection();

    diesel::delete(users.filter(id.eq(&user.id))).execute(conn)?;

    Ok(())
}
