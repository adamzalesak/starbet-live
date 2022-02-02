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

pub async fn create_game_match() -> anyhow::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;
    let database_connection_pool = Arc::new(db_connect_create_pool(&database_url).await?);
    let pg_game_match = PgMatchRepo::new(&database_connection_pool);

    println!("Create a game match:");

    let mut results: Vec<String> = Vec::new();
    let mut input;

    let messages = [
        "Game ID:",
        "Team 1 ID:",
        "Team 2 ID:",
        "Team 1 Ratio:",
        "Team 2 Ratio:",
        "Display string:",
    ];

    for message in messages {
        println!("{}", message);
        input = "".to_string();
        io::stdin().read_line(&mut input)?;
        results.push(input.clone());
    }

    let results: Vec<&str> = results.iter().map(|user_input| user_input.trim()).collect();
    let supposed_start_at = Utc::now() + Duration::hours(1);

    match pg_game_match
        .create(CreateGameMatch::new(
            results[0].parse()?,
            results[1].parse()?,
            results[2].parse()?,
            results[3].parse::<f64>()?.to_string().as_str(),
            results[4].parse::<f64>()?.to_string().as_str(),
            supposed_start_at,
            results[5],
        ))
        .await
    {
        Ok(id) => println!("Success! Match with id: {} has been created!", id),
        Err(error) => println!("ERROR: {}", error),
    }

    Ok(())
}
