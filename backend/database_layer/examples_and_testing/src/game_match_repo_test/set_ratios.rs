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

pub async fn set_ratios() -> anyhow::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;
    let database_connection_pool = Arc::new(db_connect_create_pool(&database_url).await?);
    let pg_game_match = PgMatchRepo::new(&database_connection_pool);

    println!("Set match ratio:");

    let mut results: Vec<String> = Vec::new();
    let mut input;

    let messages = ["Match ID:", "Ratio of team 1:", "Ratio of team 2:"];

    for message in messages {
        println!("{}", message);
        input = "".to_string();
        io::stdin().read_line(&mut input)?;
        results.push(input.clone());
    }

    let results: Vec<&str> = results.iter().map(|user_input| user_input.trim()).collect();
    match pg_game_match
        .set_ratios(
            results[0].parse()?,
            results[1].parse()?,
            results[2].parse()?,
        )
        .await
    {
        Ok(_) => println!(
            "Success! Ratios for match with id: {} have been set!",
            results[0]
        ),
        Err(error) => println!("ERROR: {}", error),
    }

    Ok(())
}
