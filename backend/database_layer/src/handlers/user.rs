use crate::schema::user::dsl::*;
use crate::models::user::User;
use anyhow::Result;
use super::database_connection::PgPooledConnection;
use diesel::prelude::*;



pub async fn find_user_and_his_addresses(desired_user_id: i32, connection: PgPooledConnection) -> Result<User> {

    let user_result: User = user.find(desired_user_id).first(&connection)?;

    println!("{} {}", user_result.first_name, user_result.last_name);

    Ok(user_result)
}