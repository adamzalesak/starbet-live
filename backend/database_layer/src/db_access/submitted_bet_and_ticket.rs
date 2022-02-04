use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;

use crate::connection::{PgPool, PgPooledConnection};
use crate::diesel::{prelude::*, update, QueryDsl, RunQueryDsl};

// type and structure imports
use crate::{
    db_access::{
        repo::Repo,
        user::{PgUserRepo, UserRepo},
    },
    db_models::{submitted_bet::SubmittedBet, submitted_ticket::SubmittedTicket},
};

// schema imports
use crate::schema::{submitted_bet, submitted_ticket};

pub struct PgSubmittedBetAndTicketRepo {
    pub pool: Arc<PgPool>,
}

impl PgSubmittedBetAndTicketRepo {
    /// Evaluate all bets that have not yet been evaluated -> showing user whether they won the bet or not
    ///
    /// Params
    /// ---
    /// - desired_user_id: ID of the user we wish to evaluate submitted bets of
    ///
    /// Returns
    /// ---
    /// - `Ok(())` after this method has ran successfully
    /// - `Err(_)` otherwise
    async fn evaluate_submitted_tickets(&self, desired_user_id: i32) -> anyhow::Result<()> {
        let connection: PgPooledConnection = self.get_connection().await?;
        let pg_user = PgUserRepo::new(&self.pool);

        let tickets_to_reevaluate: Vec<(SubmittedTicket, SubmittedBet)> = submitted_ticket::table
            .filter(
                submitted_ticket::user_id
                    .eq(desired_user_id)
                    .and(submitted_ticket::won.is_null()),
            )
            .inner_join(submitted_bet::table)
            .get_results(&connection)?;

        let mut bind_match_and_bets: HashMap<SubmittedTicket, Vec<SubmittedBet>> = HashMap::new();

        // bind the bets to the ticket
        for (ticket, bet) in tickets_to_reevaluate {
            let bets_vector = bind_match_and_bets.entry(ticket).or_insert_with(Vec::new);
            bets_vector.push(bet);
        }

        let mut lost_matches: Vec<i32> = Vec::new();
        let mut won_price: f64 = 0.0;
        let mut won_matches: Vec<i32> = Vec::new();

        // look through the bets and set lost and won matches accordingly
        for (ticket, bets) in bind_match_and_bets.iter() {
            let win_status: Vec<Option<bool>> = bets.iter().map(|bet| bet.won).collect();
            let total_ratio = ticket.total_ratio.parse::<f64>().ok();
            let price_paid = ticket.price_paid.parse::<f64>().ok();

            // if any bet.won is false, the match is lost
            if win_status.contains(&Some(false)) {
                lost_matches.push(ticket.id);
            // this means there was no loss, also if all matches are over, this means the bet is won
            } else if !win_status.contains(&None) {
                won_matches.push(ticket.id);

                match (total_ratio, price_paid) {
                    (Some(ratio), Some(price)) => won_price += ratio * price,
                    _ => {
                        anyhow::bail!("There has been an internal error while adding the won price")
                    }
                }
            }
        }

        // set lost matches
        let _ = update(submitted_ticket::table.filter(submitted_ticket::id.eq_any(lost_matches)))
            .set(submitted_ticket::won.eq(false))
            .execute(&connection)?;

        // set won matches
        let _ = update(submitted_ticket::table.filter(submitted_ticket::id.eq_any(won_matches)))
            .set(submitted_ticket::won.eq(true))
            .execute(&connection)?;

        // add balance to the user
        if won_price > 0.0 {
            pg_user.add_balance(desired_user_id, won_price).await?;
        }

        Ok(())
    }
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
    /// Retrieve all user's previously submitted tickets
    ///
    /// Params
    /// ---
    /// - desired_user_id: ID of the user who's tickets we wish to display
    ///
    /// Returns
    /// ---
    /// - `Ok(Vec<(SubmittedTicket, Vec<SubmittedBet>)>)` with a list of tickets (each ticket has a list of its bets)
    /// - `Err(_) if any errrors have occurred during this operation
    async fn get_all(
        &self,
        desired_user_id: i32,
    ) -> anyhow::Result<Vec<(SubmittedTicket, Vec<SubmittedBet>)>>;

    /// Retrieve all bets that are bound to a certain ticket
    ///
    /// Params
    /// ---
    /// - desired_ticket_id: ID of the ticket we wish to get its bets of
    ///
    /// Returns
    /// ---
    /// - `Ok(Vec<SubmittedBets>)` if the ticket exists and we retrieved its bets
    /// - `Err(_)` if any errors have occurred during this operation
    async fn get_bets(&self, desired_ticket_id: i32) -> anyhow::Result<Vec<SubmittedBet>>;
}

#[async_trait]
impl SubmittedBetAndTicketRepo for PgSubmittedBetAndTicketRepo {
    /// Retrieve all user's previously submitted tickets
    async fn get_all(
        &self,
        desired_user_id: i32,
    ) -> anyhow::Result<Vec<(SubmittedTicket, Vec<SubmittedBet>)>> {
        // firstly evaluate all submitted tickets
        self.evaluate_submitted_tickets(desired_user_id).await?;

        // perform join, only one call for the database needed
        let query_result: Vec<(SubmittedTicket, SubmittedBet)> = submitted_ticket::table
            .filter(submitted_ticket::user_id.eq(desired_user_id))
            .inner_join(submitted_bet::table)
            .order(submitted_ticket::submitted_at.desc())
            .get_results(&self.get_connection().await?)?;

        let mut dedup_output: HashMap<SubmittedTicket, Vec<SubmittedBet>> = HashMap::new();

        // deduplicating the list
        for (ticket, bet) in query_result {
            let bets_vector = dedup_output.entry(ticket).or_insert_with(Vec::new);
            bets_vector.push(bet);
        }

        // output as a vector
        Ok(Vec::from_iter(dedup_output.into_iter()))
    }

    /// Retrieve all bets that are bound to a certain ticket
    async fn get_bets(&self, desired_ticket_id: i32) -> anyhow::Result<Vec<SubmittedBet>> {
        let query_result: Vec<SubmittedBet> = submitted_bet::table
            .filter(submitted_bet::submitted_ticket_id.eq(desired_ticket_id))
            .get_results(&self.get_connection().await?)?;

        Ok(query_result)
    }
}
