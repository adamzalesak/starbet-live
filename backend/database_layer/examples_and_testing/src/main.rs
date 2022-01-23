use anyhow::Result;
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
        user::{PgUserRepo, UserRepo}, game::{PgGameRepo, GameRepo}, team::{PgTeamRepo, TeamRepo},
        // fe. Pg_*_Repo = actual structure holding the reference to the db pool,
        // _*_Repo = contains all methods that could be performed with the repo, along w documentation
    },
    // this module ↓ contains all models = structures that interact with the database and
    db_models::{game::CreateGame, user::CreateUser, user_address::CreateUserAddress, team::CreateTeam},
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
                "+421987654",
                None,
            ),
            CreateUserAddress::new(
                "Listova",
                "15/17",
                "Brno",
                Some("Jihomoravsky".into()), // the user has an area available, in this case -> "kraj"
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
                "+421902456789",
                None,
            ),
            CreateUserAddress::new(
                "Banicka",
                "40/b",
                "Tvrdosin",
                None, // the user has NO area available
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
    ];

    // store team id's
    let mut team_ids: Vec<i32> = Vec::new();

    // create teams
    for team in teams{
        let id = pg_team.create(team).await?;
        println!("[server] Team[{}] created", id);

        team_ids.push(id);
    }

    //
    // ==========================
    // NOW let's use the db
    // ==========================
    //

    // lets say we want to get the "Hana" user
    let hana_id = *user_ids.get(1).unwrap();
    let hana = pg_user.get(hana_id).await?;
    println!("User: \n[ name: {} {}\n  email: {}\n  civil id: {}\n  database id: {}\n]", hana.first_name, hana.last_name, hana.email, hana.civil_id_number, hana.id);

    // now lets edit hana's email, shall we?
    // we might create an easier way to edit records as we go... we'll see about that
    let hana_edit_record = CreateUser::new(
        hana.first_name.as_str(),
        hana.last_name.as_str(),
        hana.civil_id_number.as_str(),
        "han.kollarova@gmail.com",
        hana.phone_number.as_str(),
        hana.photo.as_deref(),
    );

    // edit user (returns () on success)
    pg_user.edit(hana.id, hana_edit_record).await?;

    // retrieve hana again, we see that she now has a new email address
    let hana = pg_user.get(hana_id).await?;
    println!("User: \n[ name: {} {}\n  email: {}\n  civil id: {}\n  database id: {}\n]", hana.first_name, hana.last_name, hana.email, hana.civil_id_number, hana.id);

    // lets now get hana's address okay?
    let hana_address = pg_user.get_current_address(hana_id).await?;
    println!("Hana's current address is:\n[ street name and number: {} {}\n  city: {}\n  country: {}\n  postal number: {}\n]", hana_address.street_name, hana_address.street_number, hana_address.city, hana_address.country, hana_address.postal_code);

    // lets change the address completely
    let hana_address_edit_record = CreateUserAddress::new(
        "Botanicka",
        "68a",
        "Brno",
        Some("Jihomoravsky".into()),
        "602 00",
        "Czech republic",
    );

    pg_user.edit_current_address(hana_id, hana_address_edit_record).await?;

    // now we retrieve the current address
    let hana_updated_address = pg_user.get_current_address(hana_id).await?;
    println!("Hana's current address is:\n[ street name and number: {} {}\n  city: {}\n  country: {}\n  postal number: {}\n]", hana_updated_address.street_name, hana_updated_address.street_number, hana_updated_address.city, hana_updated_address.country, hana_updated_address.postal_code);

    // we can even add a new address -> this is useful when you either have multiple billing addresses,
    // or you move, so rather than editing an old address, you just add a new one
    // NOTE -> maybe the only thing we actually want to do is to always just add a new address instead of editing one.
    // we can talk about this and i can erase the option to edit an address

    let hana_new_address = CreateUserAddress::new("Dubcekova", "11", "Ziar nad Hronom", Some("Banskobystricky".into()), "96501", "Slovakian republic");
    let _ = pg_user.add_new_address(hana_id, hana_new_address).await?;

    // now if we check what is hana's new address, it is the new address we have added
    let hana_new_address = pg_user.get_current_address(hana_id).await?;
    println!("Hana's new (current) address is:\n[ street name and number: {} {}\n  city: {}\n  country: {}\n  postal number: {}\n]", hana_new_address.street_name, hana_new_address.street_number, hana_new_address.city, hana_new_address.country, hana_new_address.postal_code);


    // now onto games and teams

    Ok(())
}
