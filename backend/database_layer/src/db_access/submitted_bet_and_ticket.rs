use async_trait::async_trait;
use std::sync::Arc;

use crate::connection::{PgPool, PgPooledConnection};
use crate::diesel::{delete, insert_into, prelude::*, sql_query, update, QueryDsl, RunQueryDsl};
use crate::type_storing::time_handling::TimeHandling;
use chrono::{DateTime, Duration, Utc};

// type and structure imports
use crate::{
    db_access::{
        repo::Repo,
        user::{PgUserRepo, UserRepo},
    },
    db_models::{
        bet::{Bet, CreateBet},
        game_match::GameMatch,
        game_match_event::{GameMatchEvent, GameMatchEventType},
        submitted_bet::{CreateSubmittedBet, SubmittedBet},
        submitted_ticket::{CreateSubmittedTicket, SubmittedTicket},
        ticket::{CreateTicket, ObtainedTicket, Ticket},
        user_address::UserAddress,
    },
};

// schema imports
use crate::schema::{bet, game, game_match, game_match_event, ticket, user};

pub struct PgSubmittedBetAndTicketRepo {
    pub pool: Arc<PgPool>,
}

#[async_trait]
impl Repo for PgSubmittedBetAndTicketRepo {
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
    fn new(pool: &Arc<PgPool>) -> PgSubmittedBetAndTicketRepo {
        PgSubmittedBetAndTicketRepo {
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
pub trait SubmittedBetAndTicketRepo {
    async fn get_all(
        &self,
        desired_user_id: i32,
    ) -> anyhow::Result<Vec<(SubmittedBet, Vec<SubmittedBet>)>>;
}
