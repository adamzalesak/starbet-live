use async_trait::async_trait;
use std::collections::HashSet;
use std::sync::Arc;

use crate::connection::{PgPool, PgPooledConnection};
use crate::diesel::{delete, insert_into, prelude::*, QueryDsl, RunQueryDsl};
use crate::type_storing::time_handling::TimeHandling;

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
    },
};

// schema imports
use crate::schema::{bet, game_match, game_match_event, submitted_bet, submitted_ticket, ticket};

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
    /// - `pool`: A reference to an already initialized database connection pool,
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
    /// - `Ok(pooled_connection)` if no error occurs
    /// - `Err(_)` if the wait for another connection is too long
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
    /// - `desired_user_id`: ID of the user we wish to remove invalid tickets
    ///
    /// Returns
    /// ---
    /// - `Ok(Vec<i32>)` with ID's of removed tickets
    /// - `Err(_)` if an error occurred
    async fn remove_invalid_tickets(&self, desired_user_id: i32) -> anyhow::Result<()>;

    /// Get the current ticket
    /// Either open a new one if the old one is invalid, or the last one was submitted
    /// Or retrieve the valid unfinished ticket from the database
    ///
    /// Params
    /// ---
    /// - `desired_user_id`: ID of the user we wish to retrieve the ticket of
    ///
    /// Returns
    /// ---
    /// - `Ok(ObtainedTicket)` with the obtained ticket for the user
    /// - `Err(_)` if an error occurred
    async fn get_user_current_ticket(&self, desired_user_id: i32)
        -> anyhow::Result<ObtainedTicket>;

    // ===================== INDIVIDUAL TICKET METHODS ===================== //

    /// Retrieve bets associated to the ticket
    ///
    /// Params
    /// ---
    /// - `desired_ticket_id`: ID of the desired ticket
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
    /// - `desired_ticket_id`: ID of the ticket we wish to place the bet on
    /// - `new_bet`: bet we wish to place
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
    /// - `desired_ticket_id`: ID of the ticket we wish to place the bet on
    /// - `desired_bet_id`: ID of the bet we wish to discard
    ///
    /// Returns
    /// ---
    /// - `Ok(())` if the bet was successfully discarded
    /// - `Err(_)` if an error occurrs
    async fn discard_a_bet(
        &self,
        desired_ticket_id: i32,
        desired_bet_id: i32,
    ) -> anyhow::Result<()>;

    /// Submit a ticket -> the ticket then gets submitted, 'paid' and shows up in the ticket history
    ///
    /// Params
    /// ---
    /// - `desired_ticket_id`: ID of the ticket we wish to pay for
    ///
    /// Returns
    /// ---
    /// - `Ok(id)` with ID of the newly submitted ticket
    /// - `Err(_)` if an error occurrs
    async fn submit_ticket(&self, desired_ticket_id: i32, paid_price: f64) -> anyhow::Result<i32>;
}

#[async_trait]
impl BetAndTicketRepo for PgBetAndTicketRepo {
    // ===================== USERs TICKETS ===================== //

    /// Remove all invalid tickets of the user
    /// invalid tickets are the ones which have not been paid and their 'valid_until' field has expired.
    async fn remove_invalid_tickets(&self, desired_user_id: i32) -> anyhow::Result<()> {
        let tickets_and_bets_and_events: Vec<(Ticket, GameMatchEvent)> = ticket::table
            .filter(ticket::user_id.eq(desired_user_id))
            .inner_join(bet::table.inner_join(
                game_match_event::table.on(game_match_event::game_match_id.eq(bet::game_match_id)),
            ))
            .order((bet::id, game_match_event::created_at.desc()))
            .distinct_on(bet::id)
            .select((ticket::all_columns, game_match_event::all_columns))
            .get_results(&self.get_connection().await?)?;

        // obtain unique ticket ids
        let tickets_to_remove: HashSet<i32> = tickets_and_bets_and_events
            .iter()
            .filter(|(_, event)| {
                event.event_type != GameMatchEventType::Live.to_string()
                    && event.event_type != GameMatchEventType::Overtime.to_string()
            })
            .map(|(ticket, _)| ticket.id)
            .collect();

        // convert to vector for diesel
        let tickets_to_remove: Vec<i32> = tickets_to_remove.into_iter().collect();

        let _ = delete(ticket::table.filter(ticket::id.eq_any(tickets_to_remove)));

        Ok(())
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
                    .values(CreateTicket::new(desired_user_id))
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
            .order(game_match_event::created_at.desc())
            .first(&connection)?;

        let event = is_game_played.extract_event()?;

        // if the game is not live or overtime, the placing of the bet cannot happen
        match event {
            GameMatchEventType::Live | GameMatchEventType::Overtime => {}
            _ => anyhow::bail!("The game is not currently played!"),
        };

        // retrieve the ticket
        let desired_ticket: Ticket = ticket::table.find(desired_ticket_id).first(&connection)?;

        // does the same person already have a bet on the match in the ticket?
        let already_has_bet: usize = bet::table
            .filter(
                bet::game_match_id
                    .eq(new_bet.game_match_id)
                    .and(bet::ticket_id.eq(desired_ticket_id)),
            )
            .execute(&connection)?;

        if already_has_bet != 0 {
            anyhow::bail!("Cannot put more bets on the same match!");
        }

        let query_result: Bet = insert_into(bet::table)
            .values(new_bet)
            .get_result(&connection)?;

        Ok(query_result)
    }

    /// Discard a specific bet.
    /// Ticket validity date will be re-calculated
    async fn discard_a_bet(
        &self,
        desired_ticket_id: i32,
        desired_bet_id: i32,
    ) -> anyhow::Result<()> {
        let connection: PgPooledConnection = self.get_connection().await?;

        // check if the bet is in ticket
        let is_bet_in_ticket: usize = bet::table
            .filter(
                bet::id
                    .eq(desired_bet_id)
                    .and(bet::ticket_id.eq(desired_ticket_id)),
            )
            .execute(&connection)?;

        if is_bet_in_ticket == 0 {
            anyhow::bail!("The bet does not belong to the ticket!")
        } else if is_bet_in_ticket != 1 {
            anyhow::bail!("Inconsistent data in the database. The bet is present twice!");
        }

        // remove the bet from the ticket
        let _ = delete(bet::table.filter(bet::id.eq(desired_bet_id))).execute(&connection)?;

        Ok(())
    }

    /// Submit a ticket -> the ticket then gets submitted, 'paid' and shows up in the ticket history
    async fn submit_ticket(&self, desired_ticket_id: i32, paid_price: f64) -> anyhow::Result<i32> {
        let connection: PgPooledConnection = self.get_connection().await?;

        // obtain current ticket, along with the bets
        let tickets_bets_games_and_latest_event: Vec<(Ticket, Bet, GameMatch, GameMatchEvent)> =
            ticket::table
                .filter(ticket::id.eq(desired_ticket_id))
                .inner_join(
                    bet::table.inner_join(game_match::table.inner_join(game_match_event::table)),
                )
                .order((bet::id, game_match_event::created_at.desc()))
                .distinct_on(bet::id)
                .select((
                    ticket::all_columns,
                    bet::all_columns,
                    game_match::all_columns,
                    game_match_event::all_columns,
                ))
                .get_results(&connection)?;

        // the ticket is empty
        if tickets_bets_games_and_latest_event.is_empty() {
            anyhow::bail!("Cannot submit an empty ticket!");
        }

        // find if there are matches that are not live
        let are_some_not_live: Option<i32> = tickets_bets_games_and_latest_event
            .iter()
            .map(|(_, _, _, event)| event.event_type.clone())
            .filter(|event_type| {
                event_type != &GameMatchEventType::Live.to_string()
                    && event_type != &GameMatchEventType::Overtime.to_string()
            })
            .map(|_| 1)
            .reduce(|element, element_two| element + element_two);

        // not all matches are live!
        if are_some_not_live.is_some() {
            anyhow::bail!("Cannot submit ticket with bets on matches that have ended or have not been played yet!");
        }

        // obtain the ticket
        let ticket: Ticket = tickets_bets_games_and_latest_event[0].0.clone();
        let bets_and_matches: Vec<(Bet, GameMatch)> = tickets_bets_games_and_latest_event
            .into_iter()
            .map(|(_, bet, game_match, _)| (bet, game_match))
            .collect();

        // check user balance first
        let user_repo = PgUserRepo::new(&self.pool);
        let balance: f64 = user_repo.get_balance(ticket.user_id).await?.parse()?;

        if balance < paid_price {
            anyhow::bail!("You do not have enough balance to do that!")
        } else if paid_price < 0.0 {
            anyhow::bail!("Cannot pay with negative amount of currency")
        }

        // create the submit ticket now and create the submit bets now
        let submitted_ticket_id: i32 = insert_into(submitted_ticket::table)
            .values(ticket.submit(paid_price, &bets_and_matches)?)
            .returning(submitted_ticket::id)
            .get_result(&connection)?;

        let submitted_bets = Bet::submit_bets(submitted_ticket_id, &bets_and_matches)?;

        // add bets to the submitted ticket
        let _ = insert_into(submitted_bet::table)
            .values(submitted_bets)
            .execute(&connection)?;

        // get ids of the bets that need to be deleted
        let original_bets_id: Vec<i32> = bets_and_matches.iter().map(|(bet, _)| bet.id).collect();

        // delete bets that are bound to the ticket
        let _ = delete(bet::table.filter(bet::id.eq_any(original_bets_id))).execute(&connection)?;

        // delete ticket
        let _ = delete(ticket::table.filter(ticket::id.eq(ticket.id))).execute(&connection)?;

        user_repo.spend_balance(ticket.user_id, paid_price).await?;

        Ok(submitted_ticket_id)
    }
}
