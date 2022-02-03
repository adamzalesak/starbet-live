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
    db_models::{game_match::CreateGameMatch, game_match_event::GameMatchEventFilter},
};

pub async fn get_all() -> anyhow::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;
    let database_connection_pool = Arc::new(db_connect_create_pool(&database_url).await?);
    let pg_game_match = PgMatchRepo::new(&database_connection_pool);

    println!("Get all game matches:");

    println!("Event type (leave empty for none):");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let event_type = GameMatchEventFilter::from_input(input.trim()).ok();

    let matches = pg_game_match.get_all_show_info(event_type, None);

    match matches.await {
        Ok(game_matches) => {
            println!("SUCCESS!\n");
            for (game_match, game_event) in game_matches {
                println!(
                    "[\n  ID: {},\n  GAME: {},\n  TEAM 1: {} (ratio: {}),\n  TEAM 2: {} (ratio: {}),\n  To start at: {},\n  Current display string: {}\n  Current game event: {}\n]\n",
                    game_match.id,
                    game_match.game_name,
                    game_match.team_one_name,
                    game_match.team_one_ratio,
                    game_match.team_two_name,
                    game_match.team_two_ratio,
                    game_match.supposed_start_at,
                    game_match.state,
                    game_event.event_type
                );
            }
        }
        Err(error) => println!("ERROR: {}", error),
    }

    Ok(())
}
