use async_trait::async_trait;
use std::sync::Arc;

use crate::db_models::user_address::CreateUserAddress;
use crate::diesel::prelude::*;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::diesel::{insert_into, update};

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
    ticket::dsl::{created_at as ticket_created_at, paid_at, ticket, user_id as user_id_ticket},
    user::{dsl::id as user_id, table as user_table},
    user_address::dsl::{
        id as user_address_id, user_address as user_address_table, user_id as user_id_address,
        valid_from,
    },
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
    async fn get(&self, desired_user_id: i32) -> anyhow::Result<User>;

    /// Create a new User record in the database
    ///
    /// Params
    /// ---
    /// - new_user: structure for database insert of a User record
    ///
    /// Returns
    /// ---
    /// - Ok(new_user_id, new_address_id) with user id and address id after successful creation
    /// - Err(_) if an error occurrs
    async fn create<'a>(
        &self,
        new_user: CreateUser<'a>,
        new_user_address: CreateUserAddress,
    ) -> anyhow::Result<(i32, i32)>;

    /// Edit User's information
    ///
    /// Params
    /// ---
    /// - desired_user_id: ID of the user we want to edit information of
    /// - edited_record: record containing (possibly) old and new information, which
    ///                  overwrite the original info in the database
    ///
    /// Returns
    /// ---
    /// - Ok(()) if the operation has been done successfully
    /// - Err(_) if an error occurrs
    async fn edit<'a>(
        &self,
        desired_user_id: i32,
        edited_record: CreateUser<'a>,
    ) -> anyhow::Result<()>;

    /// Add a new address for the user
    ///
    /// Params
    /// ---
    /// - desired_user_id: ID of the user we want to add the new address for
    /// - new_address: new address that will be tied to the user
    ///
    /// Returns
    /// ---
    /// - Ok(id) with the ID of the new address if everything went alright
    /// - Err(_) if an error occurred
    async fn add_new_address(
        &self,
        desired_user_id: i32,
        new_address: CreateUserAddress,
    ) -> anyhow::Result<i32>;

    /// Get User's current address
    ///
    /// Params
    /// ---
    /// - desired_user_id: ID of the user we wish to get the address of
    ///
    /// Returns
    /// ---
    /// - Ok(User) if the user could be found, their address exists and no error occurred while communicating with database
    /// - Err(_) if an error occurred
    async fn get_current_address(&self, desired_user_id: i32) -> anyhow::Result<UserAddress>;

    /// Same as get_current_address but only the id is retrieved
    ///
    /// Params
    /// ---
    /// - desired_user_id: ID of the user we wish to get the address of
    ///
    /// Returns
    /// ---
    /// - Ok(User) if the user could be found, their address exists and no error occurred while communicating with database
    /// - Err(_) if an error occurred
    async fn get_current_address_id(&self, desired_user_id: i32) -> anyhow::Result<i32>;

    /// Edit user's current address
    ///
    /// Params
    /// ---
    /// - desired_user_id: ID of the user we wish to change the address of
    /// - edited_address: new
    async fn edit_current_address(
        &self,
        desired_user_id: i32,
        edited_address: CreateUserAddress,
    ) -> anyhow::Result<()>;

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
    async fn get_current_ticket(&self, desired_user_id: i32) -> anyhow::Result<Option<Ticket>>;
}

#[async_trait]
impl UserRepo for PgUserRepo {
    /// Get a user from the database
    async fn get(&self, desired_id: i32) -> anyhow::Result<User> {
        let query_result: User = user_table
            .find(desired_id)
            .get_result(&self.get_connection().await?)?;

        Ok(query_result)
    }

    /// Create a new user in the database
    async fn create<'a>(
        &self,
        new_user: CreateUser<'a>,
        new_user_address: CreateUserAddress,
    ) -> anyhow::Result<(i32, i32)> {
        let new_user_id: i32 = insert_into(user_table)
            .values(new_user)
            .returning(user_id)
            .get_result(&self.get_connection().await?)?;

        let new_user_address_id = self.add_new_address(new_user_id, new_user_address).await?;

        Ok((new_user_id, new_user_address_id))
    }

    /// Edit User's information
    async fn edit<'a>(
        &self,
        desired_user_id: i32,
        edited_record: CreateUser<'a>,
    ) -> anyhow::Result<()> {
        let _ = update(user_table.find(desired_user_id))
            .set(edited_record)
            .execute(&self.get_connection().await?)?;

        Ok(())
    }

    /// Add a new address for the user, replacing the old one
    /// (the old one will still be linked to all previous tickets as a billing address)
    async fn add_new_address(
        &self,
        desired_user_id: i32,
        new_address: CreateUserAddress,
    ) -> anyhow::Result<i32> {
        let store_address = new_address.store(desired_user_id);

        let query_result: i32 = insert_into(user_address_table)
            .values(store_address)
            .returning(user_address_id)
            .get_result(&self.get_connection().await?)?;

        Ok(query_result)
    }

    /// Get user's current address
    async fn get_current_address(&self, desired_user_id: i32) -> anyhow::Result<UserAddress> {
        let query_result: UserAddress = user_address_table
            .filter(user_id_address.eq(desired_user_id))
            .order(valid_from.desc())
            .first(&self.get_connection().await?)?;

        Ok(query_result)
    }

    /// Get user's current address' ID
    async fn get_current_address_id(&self, desired_user_id: i32) -> anyhow::Result<i32> {
        let query_result: i32 = user_address_table
            .filter(user_id_address.eq(desired_user_id))
            .order(valid_from.desc())
            .select(user_address_id)
            .get_result(&self.get_connection().await?)?;

        Ok(query_result)
    }

    /// Edit user's current address
    async fn edit_current_address(
        &self,
        desired_user_id: i32,
        edited_address: CreateUserAddress,
    ) -> anyhow::Result<()> {
        let current_address_id = self.get_current_address_id(desired_user_id);
        let _ = update(user_address_table.find(current_address_id.await?))
            .set(edited_address.store(desired_user_id))
            .execute(&self.get_connection().await?)?;

        Ok(())
    }

    /// Get User's current ticket
    async fn get_current_ticket(&self, desired_user_id: i32) -> anyhow::Result<Option<Ticket>> {
        let query_result: Option<Ticket> = ticket
            .filter(user_id_ticket.eq(desired_user_id))
            .filter(paid_at.is_null())
            .order(ticket_created_at.desc())
            .first(&self.get_connection().await?)
            .optional()?;

        Ok(query_result)
    }
}
