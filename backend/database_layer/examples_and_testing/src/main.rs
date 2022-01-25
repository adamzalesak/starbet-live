use anyhow::Result;
use chrono::Utc;
use dotenv::dotenv;
use std::env;
use std::sync::Arc;

use database_layer::{
    // this module ↓ contains function to connect to the db and create the connection pool
    connection::db_connect_create_pool,
    // this module ↓ contains all repositiories necessary to work with the database
    db_access::{
        repo::Repo, // contains basic implementation for the repo (for us, the interest is in the 'new' method)
        // all tables / records have their own module
        user::{PgUserRepo, UserRepo}, game::{PgGameRepo, GameRepo}, team::{PgTeamRepo, TeamRepo}, game_match::{PgMatchRepo, MatchRepo},
        // fe. Pg_*_Repo = actual structure holding the reference to the db pool,
        // _*_Repo = contains all methods that could be performed with the repo, along w documentation
    },
    // this module ↓ contains all models = structures that interact with the database
    db_models::{game::CreateGame, user::CreateUser, user_address::CreateUserAddress, team::CreateTeam, game_match_event::GameMatchEventType},
    result_types::{GameInfo, TeamInfo},
};

#[tokio::main]
async fn main() -> Result<()> {
    // for development purposes only, using dotenv to retrieve the connection string
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;

    // create one pool of database connections, the reference will be stored
    // in every repo and the connections will be used up when necessarry
    let database_connection_pool = Arc::new(db_connect_create_pool(&database_url).await?);

    // creating a user repository
    let pg_user = PgUserRepo::new(&database_connection_pool);

    // creating a game repository
    let pg_game = PgGameRepo::new(&database_connection_pool);

    // creating a team repository 
    let pg_team = PgTeamRepo::new(&database_connection_pool); 

    // creating a game_match repository
    let pg_game_match = PgMatchRepo::new(&database_connection_pool);

    //
    // ==============================================================================================
    //                                           THE EXAMPLES
    // ==============================================================================================
    //

    //
    // ==========================
    // SEEDING THE DATABASE FIRST
    // ==========================
    //

    // Now, let's create a few users and their addresses.
    //
    //
    // User structure: first name, last name, civil id number, email, phone number, optional: image path
    // UserAddress structure: street name, street number, city, optional: area, postal number, country
    let users_and_addresses: Vec<(CreateUser, CreateUserAddress)> = vec![
        (
            CreateUser::new(
                "Janko",
                "Matuska",
                "ER125432",
                "janko@matuska.sk",
                Utc::now(),
                "+421987654",
                Some("/photos/user_profile/user_id.png"), // hypotetically let's say that the user also has a profile photo
            ),
            CreateUserAddress::new(
                "Listova",
                "15/17",
                "Brno",
                Some("Jihomoravsky"), // the user has an area available, in this case -> "kraj"
                "602 00",
                "Czech republic",
            ),
        ),
        (
            CreateUser::new(
                "Hana",
                "Kollarova",
                "EB123597",
                "kollarovahan@gmail.com",
                Utc::now(),
                "+421902456789",
                None,
            ),
            CreateUserAddress::new(
                "Banicka",
                "40/b",
                "Tvrdosin",
                Some("Zilinsky kraj"),
                "120 58",
                "Slovak republic",
            ),
        ),
        (
            CreateUser::new(
                "Annamaria",
                "Hronska",
                "EB458796",
                "annamariahronska@gmail.com",
                Utc::now(),
                "+421958012345",
                None,
            ),
            CreateUserAddress::new(
                "Krajni dolina",
                "1/14",
                "Modrice",
                None, // the user has NO area available
                "145 25",
                "Czech republic",
            ),
        ),
    ];

    let mut user_ids: Vec<i32> = Vec::new();

    // Add all of our users and their addresses into the db
    for (user, address) in users_and_addresses {
        let (user_id, address_id) = pg_user.create(user, address).await?;
        println!(
            "[server] User[{}] with Address[{}] created!",
            user_id, address_id
        );
        // store the user ids -> we'll need them later
        user_ids.push(user_id);
    }

    // Now is the good time to create some games!
    // game structure: game name, game description, game logo url
    let games: Vec<CreateGame> = vec![
        CreateGame::new(
        "Counter Strike: Global Offensive",
        "Fast, competetive online FPS game, which captivated many players.",
        "", // we don't have an image storing capabilities yet
        ),
        CreateGame::new(
        "League of Legends",
        "Online 3rd person team battle arena game",
        ""
        ), 
        CreateGame::new(
         "Valorant",
         "Competetive online FPS game, which honestly is just another rip off of CSGO and Overwatch",
         ""
        ),
        CreateGame::new(
            "Overwatch",
            "Basically the OG Valorant",
            ""
        )
    ];

    // to store created game ID's
    let mut game_ids: Vec<i32> = Vec::new();

    // create new games in the database
    for game in games {
        let id = pg_game.create(game).await?;
        println!("[server] Game[{}] created", id);

        game_ids.push(id);
    }

    // For the teams, we will add a few of those as well
    // team structure: team name, team description, team logo url

    let teams: Vec<CreateTeam> = vec![
        CreateTeam::new("Cloud9", "American esports team", "" // we don't have a way to store images yet, so this is intentionally blank
        ),
        CreateTeam::new("Fnatic", "UK esports team", ""),
        CreateTeam::new("Natus Vincere", "Ukrainian esports team", ""), // Na'Vi plays a lot of games but i'll intentionally only let it play CSGO
        CreateTeam::new("Dallas Fuel", "US esports team", ""),
    ];

    // store team id's
    let mut team_ids: Vec<i32> = Vec::new();

    // create teams
    for team in teams{
        let id = pg_team.create(team).await?;
        println!("[server] Team[{}] created", id);

        team_ids.push(id);
    }

    // assign teams to the game
    let csgo_id = *game_ids.get(0).unwrap();
    let league_id = *game_ids.get(1).unwrap();
    let valorant_id = *game_ids.get(2).unwrap();
    let overwatch_id = *game_ids.get(3).unwrap();

    let cloud9_id = *team_ids.get(0).unwrap();
    let fnatic_id = *team_ids.get(1).unwrap();
    let navi_id = *team_ids.get(2).unwrap();
    let dallas_fuel_id = *team_ids.get(3).unwrap();
    
    // this is just to demonstrate
    pg_team.add_to_game(cloud9_id, csgo_id).await?;
    pg_team.add_to_game(fnatic_id, csgo_id).await?;
    pg_team.add_to_game(navi_id, csgo_id).await?;

    pg_team.add_to_game(cloud9_id, league_id).await?;
    pg_team.add_to_game(fnatic_id, league_id).await?;

    pg_team.add_to_game(cloud9_id, valorant_id).await?;
    pg_team.add_to_game(fnatic_id, valorant_id).await?;

    pg_team.add_to_game(dallas_fuel_id, overwatch_id).await?;

    // pg_team.add_to_game(desired_team_id, desired_game_id)

    //
    // ==========================
    // NOW let's use the db
    // ==========================
    //
    println!("-----USAGE-----");

    // lets say we want to get the "Hana" user
    let hana_id = *user_ids.get(1).unwrap();
    let hana = pg_user.get(hana_id).await?;
    println!("User: \n[\n  name: {} {}\n  email: {}\n  civil id: {}\n  database id: {}\n]", hana.first_name, hana.last_name, hana.email, hana.civil_id_number, hana.id);

    println!("-----");

    // now lets edit hana's email, shall we?
    // edit user (returns () on success)
    pg_user.edit(hana.id, hana.edit_user(
        None,
        None,
        None,
        Some("han.kollarova@gmail.com"),
        None,
        None,
        None
    )).await?;

    // retrieve hana again, we see that she now has a new email address
    let hana = pg_user.get(hana_id).await?;
    println!("User: \n[\n  name: {} {}\n  email: {}\n  civil id: {}\n  database id: {}\n]", hana.first_name, hana.last_name, hana.email, hana.civil_id_number, hana.id);

    // lets now get hana's address okay?
    let hana_address = pg_user.get_current_address(hana_id).await?;
    println!("Hana's current address is:\n[\n  street name and number: {} {}\n  city: {}\n  area: {}\n  postal number: {}\n  country: {}\n]", hana_address.street_name, hana_address.street_number, hana_address.city, hana_address.area.clone().unwrap_or_else(|| "none".into()), hana_address.postal_code, hana_address.country);

    println!("-----");

    // we can add a new address, or just change the original one -> this is useful when you either have multiple billing addresses,
    // or you move (possibly only to a different street). This can also help when there is an error with the originally created address
    // -> just creates a new one

    // let hana_new_address = CreateUserAddress::new("Dubcekova", "11", "Ziar nad Hronom", Some("Banskobystricky".into()), "96501", "Slovak republic");
    let _ = pg_user.add_new_address(hana_id, hana_address.edit_address(
        Some("Hlinkova"),
        Some("12"),
        None,
        Some(None), // we have just removed the area from the address -> note: if NONE was put here, the original area would remain
        None,
        None,
    )).await?;

    // now if we check what is hana's new address, it is the new address we have added
    let hana_address = pg_user.get_current_address(hana_id).await?;
    println!("Hana's current address is:\n[\n  street name and number: {} {}\n  city: {}\n  area: {}\n  postal number: {}\n  country: {}\n]", hana_address.street_name, hana_address.street_number, hana_address.city, hana_address.area.clone().unwrap_or_else(|| "none".into()), hana_address.postal_code, hana_address.country);

    println!("-----");

    // now onto games and teams
    // we want to focus on csgo
    println!("RETRIEVE ONE GAME RECORD");
    let csgo = pg_game.get(csgo_id).await?;

    println!("The game is {} and its description is: {}", csgo.name, csgo.description);

    // say we want to edit the game?
    let csgo_edit_record = CreateGame::new(
        csgo.name.as_str(),
        "Totally the least toxic game without russians, trust me bro", csgo.logo_url.as_str()
    );

    println!("-----");
    println!("EDIT AND RETRIEVE ONE GAME RECORD");
    // edit the csgo game description
    pg_game.edit(csgo.id, csgo_edit_record).await?;

    // get the edited game!
    let csgo_edited = pg_game.get(csgo_id).await?;
    println!("The game is {} and its description is: {}", csgo_edited.name, csgo_edited.description);

    println!("-----");
    println!("RETRIEVE ALL GAMES (automatically ordered by name, as overwatch is before valorant)");

    // so now, lets get all the games that are currently in the system!
    let games_in_db: Vec<GameInfo> = pg_game.get_all().await?;

    // this is actually an info type -> can possibly change it to be it's own structure to just generally
    // not use this as a tuple
    for game in games_in_db {
        println!("Game[{}] {}", game.id, game.name);
    }

    println!("-----");
    println!("RETRIEVE ALL TEAMS (again, ordered by name by default)");

    // now get all the teams on this website
    let all_teams: Vec<TeamInfo> = pg_team.get_all(None).await?;
    for team in all_teams {
        println!("Team [{}]: {}", team.id, team.name);
    }

    println!("-----");
    println!("TEAMS THAT PLAY LEAGUE OF LEGENDS");
    // now let's focus on the teams that actually play league
    let play_league: Vec<TeamInfo> = pg_team.get_all(Some(league_id)).await?;
    for team in play_league {
        println!("Team [{}]: {}", team.id, team.name);
    }

    // if we remove fnatic from the game, only the cloud9 team will remain
    pg_team.remove_from_game(fnatic_id, league_id).await?;

    println!("-----");
    println!("AFTER REMOVAL OF FNATIC FROM LEAGUE");

    let play_league: Vec<TeamInfo> = pg_team.get_all(Some(league_id)).await?;
    for team in play_league {
        println!("Team [{}]: {}", team.id, team.name);
    }

    println!("-----");

    // now let's try to remove fnatic from the game again!
    let cant_do_that = pg_team.remove_from_game(fnatic_id, league_id).await;
    
    println!("Can we remove fnatic from league again? {}", cant_do_that.is_ok());

    // now let's check which games fnatic actually plays
    let fnatic_games: Vec<GameInfo> = pg_team.games_played(fnatic_id).await?;

    println!("-----");

    println!("Fnatic plays these games:");
    for game in fnatic_games {
        println!("Game [{}]: {}", game.id, game.name);
    }

    println!("-----");

    // now let's try to remove fnatic from the game again?
    let cant_do_that = pg_team.add_to_game(fnatic_id, csgo_id).await;

    println!("Can we add fnatic to csgo again? {}", cant_do_that.is_ok());

    println!("-----");

    println!("{}", csgo_id);
    let all_matches = pg_game_match.get_all_show(Some(GameMatchEventType::Upcoming), Some(1)).await?;

    println!("{:?}", all_matches);

    Ok(())
}
