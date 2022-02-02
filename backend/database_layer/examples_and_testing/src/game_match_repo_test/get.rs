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

pub async fn get() -> anyhow::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;
    let database_connection_pool = Arc::new(db_connect_create_pool(&database_url).await?);
    let pg_game_match = PgMatchRepo::new(&database_connection_pool);

    println!("Get a game match, specified by ID:\nMatch ID:");

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let game_match_id: i32 = input.trim().parse()?;

    match pg_game_match.get(game_match_id).await {
        Ok(game_match) => println!(
            "Success! Retrieved match:\n[\n  ID: {},\n  GAME: {},\n  TEAM 1: {} (ratio: {}),\n  TEAM 2: {} (ratio: {}),\n  To start at: {},\n  Current display string: {}\n]\n",
            game_match.id,
            game_match.game_name,
            game_match.team_one_name,
            game_match.team_one_ratio,
            game_match.team_two_name,
            game_match.team_two_ratio,
            game_match.supposed_start_at,
            game_match.state,
        ),
        Err(error) => println!("ERROR: {}", error),
    }

    Ok(())
}
