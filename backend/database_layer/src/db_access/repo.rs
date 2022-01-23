use crate::connection::{PgPool, PgPooledConnection};
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait Repo {
    /// Create a new repository for a given table (or real world entity)
    /// which encapsulates necessarry functions to operate with the database
    ///
    /// Params
    /// ---
    /// - pool: a reference to the PgPool structure, that needs to be initialized
    ///         it can then use it as an internal reference, obtaining a pooled
    ///         connection when needed
    ///
    /// Returns
    /// ---
    /// - a REPO type structure
    fn new(pool: &Arc<PgPool>) -> Self;

    /// Retrieve a connection to the database
    ///
    /// Params
    /// ---
    /// - self: a REPO trait structure
    ///
    /// Returns
    /// - Ok(PgPooledConnection) if the connection could be obtained
    /// - Err(_) otherwise
    async fn get_connection(&self) -> anyhow::Result<PgPooledConnection>;
}
