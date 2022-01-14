use crate::diesel::prelude::*;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use async_trait::async_trait;

// type and structure imports
use super::database_connection::PgPooledConnection;
use crate::models::ticket::Ticket;
use crate::models::user::User;
use crate::models::user_address::UserAddress;

// schema imports
use crate::schema::{
    user::dsl::user,
    user_address::dsl::{user_address, user_id as user_id_in_address, valid_from},
    ticket::dsl::{ticket, user_id as user_id_in_ticket, paid_at, created_at}
};

/// Empty structure containing methods to work with user
pub struct PgUserRepo {}

#[async_trait]
pub trait UserRepo {
    async fn get(desired_id: i32, connection: PgPooledConnection) -> anyhow::Result<User>;

    async fn current_address(
        desired_id: i32,
        connection: PgPooledConnection,
    ) -> anyhow::Result<UserAddress>;

    async fn current_ticket(
        desired_user_id: i32,
        connection: PgPooledConnection,
    ) -> anyhow::Result<Option<Ticket>>;
}

#[async_trait]
impl UserRepo for PgUserRepo {
    /// Get User specified by id
    ///
    /// Params
    /// ---
    /// - desired_id: ID of desired user
    ///
    /// Returns
    /// ---
    ///
    /// - Ok(User) if the user could be found and no error occurred while communicating with database
    /// - Err(_) if an error occurred
    async fn get(desired_id: i32, connection: PgPooledConnection) -> anyhow::Result<User> {
        let query_result: User = user.find(desired_id).get_result(&connection)?;

        Ok(query_result)
    }

    /// Get User's current address
    ///
    /// Paramsis myself. That is_null() exists at all feels very odd. The other syntax, which doesn't work apparently, is much more intuitive when writing Rust.
    /// Returns
    /// ---
    ///
    /// - Ok(User) if the user could be found, their address exists and no error occurred while communicating with database
    /// - Err(_) if an error occurred
    async fn current_address(
        desired_user_id: i32,
        connection: PgPooledConnection,
    ) -> anyhow::Result<UserAddress> {
        let query_result: UserAddress = user_address
            .filter(user_id_in_address.eq(desired_user_id))
            .order(valid_from.desc())
            .first(&connection)?;

        Ok(query_result)
    }

    /// Get User's current ticket
    ///
    /// Params
    /// ---
    /// - desired_user_id: ID of desired user
    ///
    /// Returns
    /// ---
    /// - Ok(Option(Ticket)) if the user could be found, and no errors occurred
    /// - Err(_) otherwise
    async fn current_ticket(
        desired_user_id: i32,
        connection: PgPooledConnection,
    ) -> anyhow::Result<Option<Ticket>> {
        let query_result: Option<Ticket> = ticket
            .filter(user_id_in_ticket.eq(desired_user_id))
            .filter(paid_at.is_null())
            .order(created_at.desc())
            .first(&connection).optional()?;
        
        Ok(query_result)
    }
}
