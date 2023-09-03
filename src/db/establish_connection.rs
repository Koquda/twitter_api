use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenvy::dotenv;
use lazy_static::lazy_static;
use std::env;

pub struct PoolWrapper;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

lazy_static! {
    static ref POOL: Pool = {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        pool
    };
}

impl PoolWrapper {
    pub fn get_instance() -> &'static Pool {
        &POOL
    }
}
