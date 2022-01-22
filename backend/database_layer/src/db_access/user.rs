use async_trait::async_trait;
use std::sync::Arc;

use crate::diesel::insert_into;
use crate::diesel::prelude::*;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;

use crate::connection::PgPool;
use crate::connection::PgPooledConnection;

// type and structure imports
use super::repo::Repo;
use crate::db_models::{
    ticket::Ticket,
    user::{CreateUser, User},
    user_address::UserAddress,
};

// schema imports
use crate::schema::{
    ticket::dsl::{created_at, paid_at, ticket, user_id as user_id_in_ticket},
    user::{
        dsl::{id as user_id, user},
        table as user_table,
    },
    user_address::dsl::{user_address, user_id as user_id_in_address, valid_from},
};

/// Structure containing a reference to a database connection pool
/// and methods to access the database
/// to work with User records
pub struct PgUserRepo {
    pub pool: Arc<PgPool>,
}

#[async_trait]
impl Repo for PgUserRepo {
    /// Create a new User repo with a reference to an initialized pool.
    ///
    /// Params
    /// ---
    /// - pool: A reference to an already initialized database connection pool,
    ///         used for connecting to the database
    ///
    /// Returns
    /// ---
    /// - new User repo
    fn new(pool: &Arc<PgPool>) -> PgUserRepo {
        PgUserRepo {
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
pub trait UserRepo {
    async fn get(&self, desired_user_id: i32) -> anyhow::Result<User>;

    async fn current_address(&self, desired_user_id: i32) -> anyhow::Result<UserAddress>;

    async fn current_ticket(&self, desired_user_id: i32) -> anyhow::Result<Option<Ticket>>;

    async fn create<'a>(&self, new_user: CreateUser<'a>) -> anyhow::Result<i32>;

    // async fn edit<'a>(&self, edit_user: CreateUser<'a>) -> anyhow::Result<()>;
}

#[async_trait]
impl UserRepo for PgUserRepo {
    /// Get User record specified by id
    ///
    /// Params
    /// ---
    /// - desired_id: ID of desired user
    ///
    /// Returns
    /// ---
    /// - Ok(User) if the user could be found and no error occurred while communicating with database
    /// - Err(_) if an error occurred
    async fn get(&self, desired_id: i32) -> anyhow::Result<User> {
        let query_result: User = user
            .find(desired_id)
            .get_result(&self.get_connection().await?)?;

        Ok(query_result)
    }

    /// Get User's current address
    ///
    /// Paramsis myself. That is_null() exists at all feels very odd. The other syntax, which doesn't work apparently, is much more intuitive when writing Rust.
    /// Returns
    /// ---
    /// - Ok(User) if the user could be found, their address exists and no error occurred while communicating with database
    /// - Err(_) if an error occurred
    async fn current_address(&self, desired_user_id: i32) -> anyhow::Result<UserAddress> {
        let query_result: UserAddress = user_address
            .filter(user_id_in_address.eq(desired_user_id))
            .order(valid_from.desc())
            .first(&self.get_connection().await?)?;

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
    async fn current_ticket(&self, desired_user_id: i32) -> anyhow::Result<Option<Ticket>> {
        let query_result: Option<Ticket> = ticket
            .filter(user_id_in_ticket.eq(desired_user_id))
            .filter(paid_at.is_null())
            .order(created_at.desc())
            .first(&self.get_connection().await?)
            .optional()?;

        Ok(query_result)
    }

    /// Create a new User record in the database
    ///
    /// Params
    /// ---
    /// - new_user: structure for database insert of a User record
    ///
    /// Returns
    /// ---
    /// - Ok(id) with user id after successful creation
    /// - Err(_) if an error occurrs
    async fn create<'a>(&self, new_user: CreateUser<'a>) -> anyhow::Result<i32> {
        let id: i32 = insert_into(user_table)
            .values(new_user)
            .returning(user_id)
            .get_result(&self.get_connection().await?)?;

        Ok(id)
    }
}
