use crate::{
    db::establish_connection::PoolWrapper,
    models::{NewUser, User},
};
use diesel::prelude::*;

pub fn get_users() -> Vec<User> {
    use crate::schema::users::dsl::*;

    let conn = &mut PoolWrapper::get_instance().get().unwrap();

    let result = users.load::<User>(conn).expect("Error loading users");

    result
}

pub fn add_user(user: &NewUser) -> Result<(), diesel::result::Error> {
    use crate::schema::users::dsl::*;

    let conn = &mut PoolWrapper::get_instance().get().unwrap();

    diesel::insert_into(users).values(user).execute(conn)?;

    Ok(())
}

pub fn get_user_by_id(user_id: &i32) -> Option<User> {
    use crate::schema::users::dsl::*;

    let conn = &mut PoolWrapper::get_instance().get().unwrap();

    let result = users
        .filter(id.eq(user_id))
        .first::<User>(conn)
        .optional()
        .expect("Error loading user");

    result
}

pub fn update_user(user: &User) -> Result<(), diesel::result::Error> {
    use crate::schema::users::dsl::*;

    let conn = &mut PoolWrapper::get_instance().get().unwrap();

    diesel::update(users.filter(id.eq(&user.id)))
        .set(user)
        .execute(conn)?;

    Ok(())
}

pub fn delete_user(user: &User) -> Result<(), diesel::result::Error> {
    use crate::schema::users::dsl::*;

    let conn = &mut PoolWrapper::get_instance().get().unwrap();

    diesel::delete(users.filter(id.eq(&user.id))).execute(conn)?;

    Ok(())
}
