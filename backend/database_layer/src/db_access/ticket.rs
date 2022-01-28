use async_trait::async_trait;
use std::sync::Arc;

use crate::connection::{PgPool, PgPooledConnection};
use crate::diesel::{delete, insert_into, prelude::*, sql_query, update, QueryDsl, RunQueryDsl};
use crate::type_storing::time_handling::TimeHandling;
use chrono::{DateTime, Duration, Utc};

// type and structure imports
use crate::{
    db_access::repo::Repo,
    db_models::{
        bet::{Bet, CreateBet},
        ticket::{CreateTicket, Ticket},
    },
};

// schema imports
use crate::schema::{bet, game, game_match, ticket, user};

/// Structure containing a reference to a database connection pool
/// and methods to access the database
/// to work with Bet records
pub struct PgTicketRepo {
    pub pool: Arc<PgPool>,
}

#[async_trait]
impl Repo for PgTicketRepo {
    /// Create a new Bet repo with a reference to an initialized pool.
    ///
    /// Params
    /// ---
    /// - pool: A reference to an already initialized database connection pool,
    ///         used for connecting to the database
    ///
    /// Returns
    /// ---
    /// - new Team repo
    fn new(pool: &Arc<PgPool>) -> PgTicketRepo {
        PgTicketRepo {
            pool: Arc::clone(pool),
        }
    }

    /// Get a connection from the pool
    ///
    /// Returns
    /// ---
    /// - Ok(pooled_connection) if no error occurs
    /// - Err(_) if the wait for another connection is too long
    async fn get_connection(&self) -> anyhow::Result<PgPooledConnection> {
        Ok(self.pool.get()?)
    }
}

#[async_trait]
pub trait TicketRepo {
    async fn open_ticket();
}
