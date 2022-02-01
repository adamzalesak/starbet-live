use dotenv::dotenv;
use std::env;
use std::io;
use std::sync::Arc;

use database_layer::{
    connection::*,
    db_access::{
        repo::Repo,
        team::{PgTeamRepo, TeamRepo},
    },
};

async fn add_or_remove(op: bool) -> anyhow::Result<()> {
    // for development purposes only, using dotenv to retrieve the connection string
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;

    // create one pool of database connections, the reference will be stored
    // in every repo and the connections will be used up when necessarry
    let database_connection_pool = Arc::new(db_connect_create_pool(&database_url).await?);

    // creating a team repository
    let pg_team = PgTeamRepo::new(&database_connection_pool);

    let action = if op { "ADD" } else { "REMOVE" };

    println!("Specify which team to {}:", action);

    // get line as an int
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let team_id: i32 = input.trim().parse()?;

    println!("Specify which game:");

    input = "".to_string();
    io::stdin().read_line(&mut input)?;
    let game_id: i32 = input.trim().parse()?;

    match op {
        true => match pg_team.add_to_game(team_id, game_id).await {
            Ok(_) => {
                println!("Success!");
            }
            Err(error) => {
                println!("ERROR: {}", error);
            }
        },
        false => match pg_team.remove_from_game(team_id, game_id).await {
            Ok(_) => {
                println!("Success!");
            }
            Err(error) => {
                println!("ERROR: {}", error);
            }
        },
    }

    Ok(())
}

pub async fn add_team_to_the_game() -> anyhow::Result<()> {
    add_or_remove(true).await
}

pub async fn remove_team_from_game() -> anyhow::Result<()> {
    add_or_remove(false).await
}
