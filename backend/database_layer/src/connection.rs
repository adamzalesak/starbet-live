use anyhow::Result;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::Connection;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

/// Initialize pool of connections
fn initialize_pool(database_url: &str) -> Result<PgPool> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Ok(Pool::builder().build(manager)?)
}

/// Establish pooled connection to the database.
///
/// Params
/// ---
/// - database_url: connection slice, used by diesel to connect to the database
///
/// Returns
/// - Ok(PgPool) = after successfull database connection and pool creation
/// - Err(_) = after an error occurred
pub async fn db_connect_create_pool(database_url: &str) -> Result<PgPool> {
    PgConnection::establish(database_url)?;
    println!("Database connection has been successful.");

    initialize_pool(database_url)
}
