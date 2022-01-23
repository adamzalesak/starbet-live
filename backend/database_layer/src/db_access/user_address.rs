use async_trait::async_trait;
use std::sync::Arc;

use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::diesel::{insert_into, update};

use crate::connection::PgPool;
use crate::connection::PgPooledConnection;

// type and structure imports
use super::repo::Repo;
use crate::db_models::user_address::{CreateUserAddress, UserAddress};

// schema imports
use crate::schema::user_address::{
    dsl::{id as user_address_id, user_address},
    table as user_address_table,
};

/// Structure containing a reference to a database connection pool
/// and methods to access the database
/// to work with UserAddress records
pub struct PgUserAddressRepo {
    pub pool: Arc<PgPool>,
}

#[async_trait]
impl Repo for PgUserAddressRepo {
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
    fn new(pool: &Arc<PgPool>) -> PgUserAddressRepo {
        PgUserAddressRepo {
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
pub trait UserAddressRepo {
    /// Get UserAddress record specified by id
    ///
    /// Params
    /// ---
    /// - desired_address_id: specific address id we're interested in
    ///
    /// Returns
    /// ---
    /// - Ok(address) if we found the address we were looking for
    /// - Err(_) if an error occurrs
    async fn get(&self, desired_address_id: i32) -> anyhow::Result<UserAddress>;

    /// Create a new UserAddress
    ///
    /// Params
    /// ---
    /// - new_address: structure for database insert of a UserAddress record
    ///
    /// Returns
    /// - Ok(id) with UserAddress id after successful creation
    /// - Err(_) if an error occurs
    async fn create(&self, new_address: CreateUserAddress) -> anyhow::Result<i32>;

    /// Edit an already existing UserAddress record
    ///
    /// Params
    /// ---
    /// - desired_address_id: i32
    /// - new_record: new information we wish to update
    ///
    /// Returns
    /// - Ok(()) if the operation has been successful
    /// - Err(_) if something went wrong
    async fn edit(
        &self,
        desired_address_id: i32,
        edited_address: CreateUserAddress,
    ) -> anyhow::Result<()>;
}

#[async_trait]
impl UserAddressRepo for PgUserAddressRepo {
    /// Get UserAddress record specified by id
    async fn get(&self, desired_address_id: i32) -> anyhow::Result<UserAddress> {
        let query_result: UserAddress = user_address
            .find(desired_address_id)
            .get_result(&self.get_connection().await?)?;

        Ok(query_result)
    }

    /// Create a new UserAddress
    async fn create(&self, new_address: CreateUserAddress) -> anyhow::Result<i32> {
        let id: i32 = insert_into(user_address_table)
            .values(new_address)
            .returning(user_address_id)
            .get_result(&self.get_connection().await?)?;

        Ok(id)
    }

    /// Edit an already existing UserAddress record
    async fn edit(
        &self,
        desired_address_id: i32,
        edited_address: CreateUserAddress,
    ) -> anyhow::Result<()> {
        let _ = update(user_address_table.find(desired_address_id))
            .set(edited_address)
            .execute(&self.get_connection().await?)?;

        Ok(())
    }
}
