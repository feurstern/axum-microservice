use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> DbPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE URL MUST BE SET!");
    let connection_manager = ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::builder()
        .build(connection_manager)
        .expect("Failed to create the pool!")
}
