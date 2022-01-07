use super::database_connection::PgPooledConnection;
use crate::models::ticket::Ticket;
use crate::models::user::User;
use crate::{
    models::user_address::UserAddress,
    schema::{
        user::dsl::{id as user_id, user},
        user_address::dsl::{user_address, valid_from},
    },
};
use anyhow::Result;
use diesel::prelude::*;

pub async fn get_user(desired_user_id: i32, connection: PgPooledConnection) -> Result<User> {
    let result = user.find(desired_user_id).first(&connection)?;
    Ok(result)
}

pub async fn get_user_current_address(
    desired_user_id: i32,
    connection: PgPooledConnection,
) -> Result<(User, UserAddress)> {
    let result: (User, UserAddress) = user
        .inner_join(user_address)
        .filter(user_id.eq(desired_user_id))
        .order(valid_from.desc())
        .first(&connection)?;

    Ok(result)
}

pub async fn get_user_current_ticket(
    desired_user_id: i32,
    connection: PgPooledConnection,
) -> Result<(User, Ticket)> {
    todo!()
}
