use dotenv::dotenv;
use std::env;
use std::sync::Arc;

use chrono::{TimeZone, Utc};
use database_layer::{
    connection::*,
    db_access::{
        game::{GameRepo, PgGameRepo},
        repo::Repo,
        team::{PgTeamRepo, TeamRepo},
        user::{PgUserRepo, UserRepo},
    },
    db_models::{
        game::CreateGame, team::CreateTeam, user::CreateUser, user_address::CreateUserAddress,
    },
};

pub fn users_and_addresses() -> Vec<(CreateUser, CreateUserAddress)> {
    vec![
        (
            CreateUser::new(
                "Tomas",
                "Sedlacek",
                "-",
                "ER748503",
                &Utc.ymd(2000, 2, 7).to_string(),
                "sedlacekt@gmail.com",
                "0912345698",
                None,
            ),
            CreateUserAddress::new(
                "Andreja",
                "125",
                "Ziar nad Hronom",
                Some("Banskobystricky"),
                "96501",
                "Slovakia",
            ),
        ),
        (
            CreateUser::new(
                "Janko",
                "Mrkvicka",
                "-",
                "ER741254",
                &Utc.ymd(1995, 12, 4).to_string(),
                "mrkvickajesef@gmail.com",
                "0914789541",
                None,
            ),
            CreateUserAddress::new(
                "Gagarinova",
                "13",
                "Trebisov",
                Some("Presovsky"),
                "12345",
                "Slovakia",
            ),
        ),
        (
            CreateUser::new(
                "Alojz",
                "Hlina",
                "-",
                "ER548503",
                &Utc.ymd(1960, 6, 12).to_string(),
                "hlina@gmail.com",
                "0915987654",
                None,
            ),
            CreateUserAddress::new(
                "Hlavna",
                "13",
                "Bratislava",
                Some("Bratislavsky"),
                "00000",
                "Slovakia",
            ),
        ),
        (
            CreateUser::new(
                "Sandricka",
                "Toth",
                "-",
                "ER113226",
                &Utc.ymd(2000, 5, 15).to_string(),
                "tothovasandra@gmail.com",
                "0912345658",
                None,
            ),
            CreateUserAddress::new(
                "SNP",
                "155",
                "Ziar nad Hronom",
                Some("Banskobystricky"),
                "96501",
                "Slovakia",
            ),
        ),
        (
            CreateUser::new(
                "Janko",
                "Matuska",
                "-",
                "ER125432",
                &Utc::now().to_string(),
                "janko@matuska.sk",
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
                "-",
                "EB123597",
                &Utc::now().to_string(),
                "kollarovahan@gmail.com",
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
                "-",
                "EB458796",
                &Utc::now().to_string(),
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
    ]
}

pub fn games() -> Vec<CreateGame> {
    vec![
        CreateGame::new(
            "Counter Strike: Global Offensive",
            "One of the best known online FPS games",
            "",
        ),
        CreateGame::new(
            "Dota 2",
            "3rd person MOBA game, based on Warcraft 2 mod",
            "",
        ),
        CreateGame::new("Valorant", "A CSGO & Overwatch crossover!", ""),
        CreateGame::new("Overwatch", "Fun FPS", ""),
    ]
}

pub fn teams() -> Vec<CreateTeam> {
    vec![
        CreateTeam::new("Fnatic", "UK based esports team", ""),
        CreateTeam::new("Natus Vincere", "Ukraine based esports team", ""),
        CreateTeam::new("Cloud9", "US based esports team", ""),
    ]
}

pub async fn seed() -> anyhow::Result<()> {
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

    for (user, address) in users_and_addresses() {
        println!(
            "[structure] User '{} {}' being created:",
            &user.first_name, &user.last_name
        );
        let (uid, aid) = pg_user.create(user, address).await?;
        println!("[server] User[{}] with Address[{}] created!\n", &uid, &aid);
    }

    for game in games() {
        println!("[structure] Game '{}' being created:", &game.name);
        let gid = pg_game.create(game).await?;
        println!("[server] Game[{}] created!\n", &gid);
    }

    for team in teams() {
        println!("[structure] Team '{}' being created:", &team.name);
        let tid = pg_team.create(team).await?;
        println!("[server] Team[{}] created!\n", &tid);
    }

    Ok(())
}
