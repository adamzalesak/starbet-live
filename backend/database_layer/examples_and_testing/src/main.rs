#![allow(unused_variables, unused_import_braces, unused_imports, dead_code)]

use clap::{App, Arg};

mod game_match;
mod seed;
mod team_in_game;

use game_match::create_game_match;
use seed::seed;
use team_in_game::{add_team_to_the_game, remove_team_from_game};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // argument parsing
    let testing_app = App::new("Database layer testing")
        .version("1.1")
        .author("Tomas Sedlacek")
        .about("Allows me to test my contribution to the project")
        .arg(
            Arg::new("seed")
                .short('s')
                .long("seed")
                .takes_value(false)
                .help("Run the seeding of the database")
                .required(false)
                .takes_value(false),
        )
        .arg(
            Arg::new("add_team_to_game")
                .short('1')
                .long("team-to-game")
                .help("Add a team to the game")
                .required(false)
                .takes_value(false),
        )
        .arg(
            Arg::new("remove_team_from_game")
                .short('2')
                .long("remove-from-game")
                .help("Remove team from the game")
                .required(false)
                .takes_value(false),
        )
        .arg(
            Arg::new("create_game_match")
                .short('3')
                .long("create-match")
                .help("Create a game match")
                .required(false)
                .takes_value(false),
        )
        .get_matches();

    if testing_app.is_present("seed") {
        seed().await?;
    }

    if testing_app.is_present("add_team_to_game") {
        add_team_to_the_game().await?;
    } else if testing_app.is_present("remove_team_from_game") {
        remove_team_from_game().await?;
    } else if testing_app.is_present("create_game_match") {
        create_game_match().await?;
    }

    println!("App ran successfully");
    Ok(())
}
