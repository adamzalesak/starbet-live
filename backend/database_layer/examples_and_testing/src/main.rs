#![allow(unused_variables, unused_import_braces, unused_imports, dead_code)]

use clap::{App, Arg};

mod game_match_repo_test;
mod seed;
mod team_plays_game_test;
mod user_repo_test;

use seed::seed;

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
                .value_name("user-value")
                .help("Usage: --user add / get / create / edit / get-address / add-address / get-balance / add-balance "),
        ).arg(
            Arg::new("team_plays_game")
                .short('1')
                .long("team-plays-game")
                .takes_value(true)
                .value_name("team_plays_game_value")
                .help("Usage: --team-plays-game add / remove"),
        ).arg(
            Arg::new("game_match")
                .short('2')
                .long("game-match")
                .takes_value(true)
                .value_name("game_match_value")
                .help("Usage: --game_match create / get / get all"),
        )
        .get_matches();

    if testing_app.is_present("seed") {
        seed().await?;
    }

    if testing_app.is_present("user") {
        user_repo_test::run(testing_app.value_of("user")).await?;
    } else if testing_app.is_present("team_plays_game") {
        team_plays_game_test::run(testing_app.value_of("team_plays_game")).await?;
    } else if testing_app.is_present("game_match") {
        game_match_repo_test::run(testing_app.value_of("game_match")).await?;
    }

    println!("\n\n=================\nApp ran successfully\n=================");
    Ok(())
}
