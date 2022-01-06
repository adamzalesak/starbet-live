
use diesel::prelude::*;
use diesel::pg::PgConnection;
use anyhow::Result;

pub async fn establish_connection(database_url: &str) -> Result<PgConnection> {
    Ok(PgConnection::establish(&database_url)?)
}
