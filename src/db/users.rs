use crate::models::User;
use diesel::prelude::*;

pub fn get_users(conn: &mut PgConnection) -> Vec<User> {
    use crate::schema::users::dsl::*;

    let result = users.load::<User>(conn).expect("Error loading users");

    result
}
