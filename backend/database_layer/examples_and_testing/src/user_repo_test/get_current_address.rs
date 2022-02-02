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

use crate::team_plays_game_test::team_in_game::add_team_to_the_game;

pub async fn get_current_address() -> anyhow::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;
    let database_connection_pool: Arc<PgPool> =
        Arc::new(db_connect_create_pool(&database_url).await?);
    let pg_user: PgUserRepo = PgUserRepo::new(&database_connection_pool);

    println!("Get a desired user's (at the moment) current address:\nUser id:");

    let mut input = "".to_string();
    io::stdin().read_line(&mut input)?;
    let user_id: i32 = input.trim().parse()?;

    let user = pg_user.get_current_address(user_id).await;

    match user {
        Ok(address) => println!(
            "[\n  Street name: {}\n  Street number: {}\n  City: {}\n  Area: {}\n  Postal code: {}\n Country: {}\n]\n",
            address.street_name, address.street_number, address.city, address.area.unwrap_or("No area specified".into()), address.postal_code, address.country
        ),
        Err(error) => println!("ERROR: {}", error),
    }

    Ok(())
}
