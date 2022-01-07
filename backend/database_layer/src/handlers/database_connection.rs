use anyhow::Result;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::Connection;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

fn initialize_pool(database_url: &str) -> Result<PgPool> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Ok(Pool::builder().build(manager)?)
}

pub async fn establish_connection(database_url: &str) -> Result<PgPool> {
    PgConnection::establish(&database_url)?;
    println!("Database connection has been successful.");

    initialize_pool(database_url)
}
