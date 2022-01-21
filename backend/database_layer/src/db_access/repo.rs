use crate::connection::{PgPool, PgPooledConnection};
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait Repo {
    fn new(pool: &Arc<PgPool>) -> Self;

    async fn get_connection(&self) -> anyhow::Result<PgPooledConnection>;
}
