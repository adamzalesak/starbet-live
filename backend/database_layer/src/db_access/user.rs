use async_trait::async_trait;
use std::sync::Arc;

use crate::diesel::{delete, insert_into, prelude::*, update, QueryDsl, RunQueryDsl};

use crate::connection::{PgPool, PgPooledConnection};

// type and structure imports
use super::repo::Repo;
use crate::db_models::{
    game_match_event::GameMatchEventType,
    ticket::{CreateTicket, ObtainedTicket, Ticket},
    user::{CreateUser, User},
    user_address::{CreateUserAddress, UserAddress},
};

// schema imports
use crate::schema::{bet, game_match, game_match_event, ticket, user, user_address};

/// Structure containing a reference to a database connection pool
/// and methods to access the database
/// to work with User records
pub struct PgUserRepo {
    pub pool: Arc<PgPool>,
}

#[async_trait]
impl Repo for PgUserRepo {
    /// Create a new User repo with a reference to an initialized pool.
    fn new(pool: &Arc<PgPool>) -> PgUserRepo {
        PgUserRepo {
            pool: Arc::clone(pool),
        }
    }

    /// Get a connection from the pool
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
    /// - `Ok(User)` if the user could be found and no error occurred while communicating with database
    /// - `Err(_)` if an error occurred
    async fn get(&self, desired_user_id: i32) -> anyhow::Result<User>;

    /// Create a new User record in the database
    ///
    /// Params
    /// ---
    /// - new_user: structure for database insert of a User record
    ///
    /// Returns
    /// ---
    /// - `Ok(new_user_id, new_address_id)` with user id and address id after successful creation
    /// - `Err(_)` if an error occurrs
    async fn create(
        &self,
        new_user: CreateUser,
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
    /// - `Ok(())` if the operation has been done successfully
    /// - `Err(_)` if an error occurrs
    async fn edit(&self, desired_user_id: i32, edited_record: CreateUser) -> anyhow::Result<()>;

    /// Add a new address for the user
    ///
    /// Params
    /// ---
    /// - desired_user_id: ID of the user we want to add the new address for
    /// - new_address: new address that will be tied to the user
    ///
    /// Returns
    /// ---
    /// - `Ok(id)` with the ID of the new address if everything went alright
    /// - `Err(_)` if an error occurred
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
    /// - `Ok(User)` if the user could be found, their address exists and no error occurred while communicating with database
    /// - `Err(_)` if an error occurred
    async fn get_current_address(&self, desired_user_id: i32) -> anyhow::Result<UserAddress>;

    /// Get user's balance
    ///
    /// Params
    /// ---
    /// - desired_user_id: ID of the user we wish to get the balance of
    ///
    /// Returns
    /// ---
    /// - `Ok(balance)` with the balance of the user
    /// - `Err(_)` if an error occurrs
    async fn get_balance(&self, desired_user_id: i32) -> anyhow::Result<String>;

    /// Add balance to the user's account
    /// Fails if the balance specified is negative
    ///
    /// Params
    /// ---
    /// - desired_user_id: ID of the user we wish to add the balance to
    /// - desired_amount: amount of money we wish to add to the user's account
    ///
    /// Returns
    /// ---
    /// - `Ok(())` if the operation was successful
    /// - `Err(_)` otherwise
    async fn add_balance(&self, desired_user_id: i32, desired_amount: f64) -> anyhow::Result<()>;

    /// Withdraw the user's balance
    /// Fails if the balance specified is higher than the current balance
    ///
    /// Params
    /// ---
    /// - desired_user_id: ID of the user we wish to withdraw the balance of
    /// - desired_withdrawal: amount of money the user wants to withdraw
    async fn spend_balance(
        &self,
        desired_user_id: i32,
        desired_spending: f64,
    ) -> anyhow::Result<()>;
}

#[async_trait]
impl UserRepo for PgUserRepo {
    /// Get a user from the database
    async fn get(&self, desired_id: i32) -> anyhow::Result<User> {
        let query_result: User = user::table
            .find(desired_id)
            .get_result(&self.get_connection().await?)?;

        Ok(query_result)
    }

    /// Create a new user in the database
    async fn create(
        &self,
        new_user: CreateUser,
        new_user_address: CreateUserAddress,
    ) -> anyhow::Result<(i32, i32)> {
        let connection: PgPooledConnection = self.get_connection().await?;

        // check if the user already exists
        let already_exists: Vec<User> = user::table
            .filter(user::email.eq(new_user.email.clone()))
            .get_results(&connection)?;

        if already_exists.len() != 0 {
            anyhow::bail!("The user already exists!");
        }

        let new_user_id: i32 = insert_into(user::table)
            .values(new_user)
            .returning(user::id)
            .get_result(&connection)?;

        let new_user_address_id = self.add_new_address(new_user_id, new_user_address).await?;

        Ok((new_user_id, new_user_address_id))
    }

    /// Edit User's information
    async fn edit(&self, desired_user_id: i32, edited_record: CreateUser) -> anyhow::Result<()> {
        let connection: PgPooledConnection = self.get_connection().await?;

        // check if the user already exists
        let already_exists: Vec<User> = user::table
            .filter(user::email.eq(edited_record.email.clone()))
            .get_results(&connection)?;

        match already_exists.len() {
            0 => {}
            1 => {
                let found_record: User = already_exists[0].clone();

                if &found_record.email == &edited_record.email && found_record.id != desired_user_id
                {
                    anyhow::bail!(
                        "Cannot change the email address to an address that is already in use!"
                    );
                }
            }
            _ => {
                anyhow::bail!(
                    "Internal error. Multiple accounts with the same email have been found"
                )
            }
        }

        let _ = update(user::table.find(desired_user_id))
            .set(edited_record)
            .execute(&connection)?;

        Ok(())
    }

    /// Get user's current address
    async fn get_current_address(&self, desired_user_id: i32) -> anyhow::Result<UserAddress> {
        let query_result: UserAddress = user_address::table
            .filter(user_address::user_id.eq(desired_user_id))
            .order(user_address::valid_from.desc())
            .first(&self.get_connection().await?)?;

        Ok(query_result)
    }

    /// Add a new address for the user, replacing the old one
    /// (the old one will still be linked to all previous tickets as a billing address)
    async fn add_new_address(
        &self,
        desired_user_id: i32,
        new_address: CreateUserAddress,
    ) -> anyhow::Result<i32> {
        let store_address = new_address.store(desired_user_id);

        let query_result: i32 = insert_into(user_address::table)
            .values(store_address)
            .returning(user_address::id)
            .get_result(&self.get_connection().await?)?;

        Ok(query_result)
    }

    /// Get user's balance
    async fn get_balance(&self, desired_user_id: i32) -> anyhow::Result<String> {
        let query_result: String = user::table
            .find(desired_user_id)
            .select(user::balance)
            .get_result(&self.get_connection().await?)?;

        Ok(query_result)
    }

    /// Add balance to the user's account
    /// Fails if the balance specified is negative
    async fn add_balance(&self, desired_user_id: i32, desired_amount: f64) -> anyhow::Result<()> {
        if desired_amount <= 0.0 {
            anyhow::bail!("Cannot 'add' a negative balance!")
        }

        // retrieve balance
        let user_balance: String = self.get_balance(desired_user_id).await?;
        let converted_balance: f64 = user_balance.parse::<f64>()?;

        // update the balance
        let _ = update(user::table.find(desired_user_id))
            .set(user::balance.eq((converted_balance + desired_amount).to_string()))
            .execute(&self.get_connection().await?)?;

        Ok(())
    }

    /// Withdraw the user's balance
    /// Fails if the balance specified is higher than the current balance
    async fn spend_balance(
        &self,
        desired_user_id: i32,
        desired_spending: f64,
    ) -> anyhow::Result<()> {
        if desired_spending <= 0.0 {
            anyhow::bail!("Cannot 'withdraw' a negative balance!")
        }

        // retrieve balance
        let user_balance: String = self.get_balance(desired_user_id).await?;
        let converted_balance: f64 = user_balance.parse::<f64>()?;

        if converted_balance < desired_spending {
            anyhow::bail!("Cannot withdraw more money that user has.")
        }

        // update the balance
        let _ = update(user::table.find(desired_user_id))
            .set(user::balance.eq((converted_balance - desired_spending).to_string()))
            .execute(&self.get_connection().await?)?;

        Ok(())
    }
}
