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

pub async fn delete_game_match() -> anyhow::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;
    let database_connection_pool = Arc::new(db_connect_create_pool(&database_url).await?);
    let pg_game_match = PgMatchRepo::new(&database_connection_pool);

    println!("Delete a game match:");

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let game_match_id: i32 = input.trim().parse()?;

    match pg_game_match.delete(game_match_id).await {
        Ok(_) => println!(
            "Success! Match with id: {} has been deleted!",
            game_match_id
        ),
        Err(error) => println!("ERROR: {}", error),
    }

    Ok(())
}
