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
        ticket::{CreateTicket, ObtainedTicket, Ticket},
        user_address::UserAddress,
    },
};

// schema imports
use crate::schema::{bet, game, game_match, game_match_event, ticket, user};

/// Structure containing a reference to a database connection pool
/// and methods to access the database
/// to work with Bet records
pub struct PgBetAndTicketRepo {
    pub pool: Arc<PgPool>,
}

#[async_trait]
impl Repo for PgBetAndTicketRepo {
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
    fn new(pool: &Arc<PgPool>) -> PgBetAndTicketRepo {
        PgBetAndTicketRepo {
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

// if the user has an invalid ticket OR user has NO ticket, then create a new one
// if the user has an open ticket that is valid, return the valid ticket

#[async_trait]
pub trait BetAndTicketRepo {
    // ===================== USERs TICKETS ===================== //

    /// Remove all invalid tickets of the user
    /// invalid tickets are the ones which have not been paid and their 'valid_until' field has surpassed the
    /// current date.
    ///
    /// Params
    /// ---
    /// - desired_user_id: ID of the user we wish to remove invalid tickets
    ///
    /// Returns
    /// ---
    /// - `Ok(Vec<i32>)` with ID's of removed tickets
    /// - `Err(_)` if an error occurred
    async fn remove_invalid_tickets(&self, desired_user_id: i32) -> anyhow::Result<Vec<i32>>;

    /// Get the current ticket
    /// Either open a new one if the old one is invalid, or the last one was submitted
    /// Or retrieve the valid unfinished ticket from the database
    async fn get_user_current_ticket(&self, desired_user_id: i32)
        -> anyhow::Result<ObtainedTicket>;

    // ===================== INDIVIDUAL TICKET METHODS ===================== //

    /// Retrieve bets associated to the ticket
    ///
    /// Params
    /// ---
    /// - desired_ticket_id: ID of the desired ticket
    ///
    /// Returns
    /// ---
    /// - `Ok(Vec<Bet>)` with all bets in the desired ticket
    /// - `Err(_)` if an error has occurred
    async fn get_bets(&self, desired_ticket_id: i32) -> anyhow::Result<Vec<Bet>>;

    /// Place a bet on a team in a match
    /// Ticket validity date will be re-calculated
    ///
    /// Params
    /// ---
    /// - desired_ticket_id: ID of the ticket we wish to place the bet on
    /// - new_bet: bet we wish to place
    ///
    /// Returns
    /// ---
    /// - `Ok(bet)` when the bet has been successful
    /// - `Err(_)` if an error has occurred
    async fn place_a_bet(&self, desired_ticket_id: i32, new_bet: CreateBet) -> anyhow::Result<Bet>;

    /// Discard a specific bet.
    /// The validity date of the ticket will be re-calculated
    ///
    /// Params
    /// ---
    /// - desired_ticket_id: ID of the ticket we wish to place the bet on
    /// - desired_bet_id: ID of the bet we wish to discard
    ///
    /// Returns
    /// ---
    /// - `Ok(())` if the bet was successfully discarded
    /// - `Err(_)` if an error occurrs
    async fn discard_a_bet(&self, desired_bet_id: i32) -> anyhow::Result<()>;

    /// Submit a ticket -> the ticket then gets submitted, 'paid' and shows up in the ticket history
    ///
    /// Params
    /// ---
    /// - desired_ticket_id: ID of the ticket we wish to pay for
    ///
    /// Returns
    /// ---
    /// - `Ok(id)` with ID of the newly submitted ticket
    /// - `Err(_)` if an error occurrs
    async fn submit_ticket(&self, desired_ticket_id: i32) -> anyhow::Result<i32>;
}

#[async_trait]
impl BetAndTicketRepo for PgBetAndTicketRepo {
    // ===================== USERs TICKETS ===================== //

    /// Remove all invalid tickets of the user
    /// invalid tickets are the ones which have not been paid and their 'valid_until' field has expired.
    async fn remove_invalid_tickets(&self, desired_user_id: i32) -> anyhow::Result<Vec<i32>> {
        let invalid_ticket_ids: Vec<i32> = delete(
            ticket::table.filter(
                ticket::user_id
                    .eq(desired_user_id)
                    .and(ticket::valid_until.le(TimeHandling::store())),
            ),
        )
        .returning(ticket::id)
        .get_results(&self.get_connection().await?)?;

        Ok(invalid_ticket_ids)
    }

    /// Get the current ticket
    /// Either open a new one if the old one is invalid, or the last one was paid for
    /// Or retrieve the unfinished ticket from the database
    async fn get_user_current_ticket(
        &self,
        desired_user_id: i32,
    ) -> anyhow::Result<ObtainedTicket> {
        // firstly remove all invalid tickets
        self.remove_invalid_tickets(desired_user_id).await?;
        // obtain connection that is used throughout this method
        let connection: PgPooledConnection = self.get_connection().await?;

        let potential_open_ticket: Vec<Ticket> = ticket::table
            .filter(ticket::user_id.eq(desired_user_id))
            .order(ticket::created_at.desc())
            .get_results(&connection)?;

        // check for open tickets
        match potential_open_ticket.len() {
            // no tickets were open -> either first time using it, or need for new ticket after submitting the old one
            0 => {
                let new_open_ticket: Ticket = insert_into(ticket::table)
                    .values(CreateTicket::new(desired_user_id, ""))
                    .get_result(&connection)?;

                Ok(ObtainedTicket::NoTicketFound(new_open_ticket))
            }
            // found a valid open ticket
            1 => Ok(ObtainedTicket::StillValid(potential_open_ticket[0].clone())),
            // when used correctly, this will never happen
            _ => {
                anyhow::bail!("More than one ticket found open! Please, contact site administrator")
            }
        }
    }

    // ===================== INDIVIDUAL TICKET METHODS ===================== //

    /// Retrieve bets associated to the ticket
    async fn get_bets(&self, desired_ticket_id: i32) -> anyhow::Result<Vec<Bet>> {
        let query_result: Vec<Bet> = bet::table
            .filter(bet::ticket_id.eq(desired_ticket_id))
            .order(bet::created_at.desc())
            .get_results(&self.get_connection().await?)?;

        Ok(query_result)
    }

    /// Place a bet on a team in a match
    /// Ticket validity date will be re-calculated
    async fn place_a_bet(&self, desired_ticket_id: i32, new_bet: CreateBet) -> anyhow::Result<Bet> {
        let connection = self.get_connection().await?;

        // check if the game is played right now
        let is_game_played: GameMatchEvent = game_match_event::table
            .filter(game_match_event::game_match_id.eq(new_bet.game_match_id))
            .order(game_match_event::created_at)
            .first(&connection)?;

        let event = is_game_played.extract_event()?;

        // retrieve played until date
        let played_until = match event {
            GameMatchEventType::Live(date) | GameMatchEventType::Overtime(date) => {
                if date <= Utc::now() {
                    anyhow::bail!("Cannot place a bet on a match that is in the past");
                }
                date
            }
            _ => anyhow::bail!("The game is not currently played!"),
        };

        // retrieve the ticket
        let desired_ticket: Ticket = ticket::table.find(desired_ticket_id).first(&connection)?;

        // update the new ticket validity
        if TimeHandling::load_timestamp(&desired_ticket.valid_until)? >= played_until {
            let _ = update(ticket::table.filter(ticket::id.eq(desired_ticket.id)))
                .set(ticket::valid_until.eq(played_until.to_string()))
                .execute(&connection)?;
        }

        let query_result: Bet = insert_into(bet::table)
            .values(new_bet)
            .get_result(&connection)?;

        Ok(query_result)
    }

    /// Discard a specific bet.
    /// Ticket validity date will be re-calculated
    async fn discard_a_bet(&self, desired_bet_id: i32) -> anyhow::Result<()> {
        let connection: PgPooledConnection = self.get_connection().await?;

        // let be

        todo!()
    }

    /// Submit a ticket -> the ticket then gets submitted, 'paid' and shows up in the ticket history
    async fn submit_ticket(&self, desired_ticket_id: i32) -> anyhow::Result<i32> {
        todo!()
    }
}
