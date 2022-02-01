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

    println!("Create a game match:\nGame id:");

    let mut input = "".to_string();
    io::stdin().read_line(&mut input)?;
    let game_id: i32 = input.trim().parse()?;

    println!("Team 1 ID:");

    input = "".to_string();
    io::stdin().read_line(&mut input)?;
    let team_one_id: i32 = input.trim().parse()?;

    println!("Team 2 ID:");

    input = "".to_string();
    io::stdin().read_line(&mut input)?;
    let team_two_id: i32 = input.trim().parse()?;

    println!("Team 1 ratio:");

    input = "".to_string();
    io::stdin().read_line(&mut input)?;
    let team_one_ratio = input.clone();

    println!("Team 2 ratio:");

    input = "".to_string();
    io::stdin().read_line(&mut input)?;
    let team_two_ratio = input.clone();

    let supposed_start_at = Utc::now() + Duration::minutes(5);

    println!("Display string:");

    input = "".to_string();
    io::stdin().read_line(&mut input)?;
    let state = input.clone();

    match pg_game_match
        .create(CreateGameMatch::new(
            game_id,
            team_one_id,
            team_two_id,
            &team_one_ratio,
            &team_two_ratio,
            supposed_start_at,
            &state,
        ))
        .await
    {
        Ok(id) => println!("Success! Match with id: {} has been created!", id),
        Err(error) => println!("ERROR: {}", error),
    }

    Ok(())
}
