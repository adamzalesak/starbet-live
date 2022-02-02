use dotenv::dotenv;
use std::env;
use std::io;
use std::sync::Arc;

use chrono::{TimeZone, Utc};
use database_layer::{
    connection::*,
    db_access::{
        repo::Repo,
        user::{PgUserRepo, UserRepo},
    },
    db_models::{
        user::{CreateUser, User},
        user_address::{CreateUserAddress, UserAddress},
    },
};

pub async fn add_balance() -> anyhow::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;
    let database_connection_pool: Arc<PgPool> =
        Arc::new(db_connect_create_pool(&database_url).await?);
    let pg_user: PgUserRepo = PgUserRepo::new(&database_connection_pool);

    println!("Add user's balance!\nUser ID:");

    let mut input = "".to_string();
    io::stdin().read_line(&mut input)?;
    let user_id: i32 = input.trim().parse()?;

    println!("Balance:");

    input = "".to_string();
    io::stdin().read_line(&mut input)?;
    let balance: f64 = input.trim().parse()?;

    match pg_user.add_balance(user_id, balance).await {
        Ok(_) => println!("Success: Balance added!"),
        Err(error) => println!("ERROR: {}", error),
    }

    Ok(())
}
