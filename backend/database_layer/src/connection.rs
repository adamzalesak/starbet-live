use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
    Connection,
};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

/// Initialize pool of connections
///
/// Params
/// ---
/// - database_url: Connection string
///
/// Returns
/// ---
/// - Ok(connection_pool) if the pool has been created successfully
/// - Err(_) otherwise
fn initialize_pool(database_url: &str) -> anyhow::Result<PgPool> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    // TODO on production -> configure the builder correctly
    Ok(Pool::builder().max_size(1500).build(manager)?)
}

/// Establish a pooled connection to the database.
///
/// Params
/// ---
/// - database_url: connection slice, used by diesel to connect to the database
///
/// Returns
/// - Ok(PgPool) = after successfull database connection and pool creation
/// - Err(_) = after an error occurred
pub async fn db_connect_create_pool(database_url: &str) -> anyhow::Result<PgPool> {
    PgConnection::establish(database_url)?;
    println!("Database connection has been successful.");

    initialize_pool(database_url)
}
