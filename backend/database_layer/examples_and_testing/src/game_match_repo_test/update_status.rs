use chrono::Duration;
use chrono::Utc;
use dotenv::dotenv;
use std::env;
use std::io;
use std::sync::Arc;

use database_layer::{
    connection::*,
    db_access::{
        game_match::{MatchRepo, PgMatchRepo},
        repo::Repo,
    },
    db_models::game_match::CreateGameMatch,
};

pub async fn update_status() -> anyhow::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;
    let database_connection_pool = Arc::new(db_connect_create_pool(&database_url).await?);
    let pg_game_match = PgMatchRepo::new(&database_connection_pool);

    println!("Update status of a match, specified by ID:\nMatch ID:");

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let game_match_id: i32 = input.trim().parse()?;

    println!("New status:");

    input = "".into();
    io::stdin().read_line(&mut input)?;
    let new_status = input.trim();

    match pg_game_match.update_status(game_match_id, new_status).await {
        Ok(_) => println!(
            "Success! Status of the match with ID {} has successfully been updated to: '{}'",
            game_match_id, new_status
        ),
        Err(error) => println!("ERROR: {}", error),
    }

    Ok(())
}
