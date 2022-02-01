#![allow(unused_variables, unused_import_braces, unused_imports, dead_code)]

use clap::{App, Arg};

mod game_match;
mod seed;
mod team_in_game;
mod user_repo_test;

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
            Arg::new("user")
                .short('u')
                .long("user")
                .takes_value(true)
                .value_name("user-option")
                .help("Usage: --user add / get / create / edit / get-address / add-address / get-balance / add-balance "),
        )
        // .arg(
        //     Arg::new("add_team_to_game")
        //         .short('1')
        //         .long("team-to-game")
        //         .help("Add a team to the game")
        //         .required(false)
        //         .takes_value(false),
        // )
        // .arg(
        //     Arg::new("remove_team_from_game")
        //         .short('2')
        //         .long("remove-from-game")
        //         .help("Remove team from the game")
        //         .required(false)
        //         .takes_value(false),
        // )
        // .arg(
        //     Arg::new("create_game_match")
        //         .short('3')
        //         .long("create-match")
        //         .help("Create a game match")
        //         .required(false)
        //         .takes_value(false),
        // )
        .get_matches();

    if testing_app.is_present("seed") {
        seed().await?;
    }

    if testing_app.is_present("user") {
        user_repo_test::run(testing_app.value_of("user")).await?;
    }

    println!("App ran successfully");
    Ok(())
}
