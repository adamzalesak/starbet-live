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
            "https://seeklogo.com/images/C/csgo-logo-CAA0A4D48A-seeklogo.com.png",
        ),
        CreateGame::new(
            "Dota 2",
            "3rd person MOBA game, based on Warcraft 2 mod",
            "https://i.pinimg.com/originals/8a/8b/50/8a8b50da2bc4afa933718061fe291520.jpg",
        ),
        CreateGame::new(
            "League of Legends",
            "3rd person MOBA, competitor of Dota 2 and one of the most played games in the world",
            "https://pentagram-production.imgix.net/cc7fa9e7-bf44-4438-a132-6df2b9664660/EMO_LOL_02.jpg?rect=0%2C0%2C1440%2C1512&w=640&crop=1&fm=jpg&q=70&auto=format&fit=crop&h=672",
        ),
        CreateGame::new(
            "Valorant",
            "A CSGO & Overwatch crossover!",
            "https://upload.wikimedia.org/wikipedia/commons/f/fc/Valorant_logo_-_pink_color_version.svg"
        ),
        CreateGame::new(
            "Overwatch",
            "Fun FPS game", 
            "https://upload.wikimedia.org/wikipedia/commons/thumb/5/55/Overwatch_circle_logo.svg/1024px-Overwatch_circle_logo.svg.png"),
        CreateGame::new(
            "Rocket League",
            "One of the most famous 'football' games",
            "https://www.kindpng.com/picc/m/467-4672294_rocket-league-logo-rocket-league-hd-png-download.png",
        ),
    ]
}

pub fn teams() -> Vec<CreateTeam> {
    vec![
        // 1
        // csgo, league
        CreateTeam::new(
            "Astralis",
            "Denmark based esports team",
            "https://upload.wikimedia.org/wikipedia/commons/7/7d/Astralis_logo.svg",
        ),
        // 2
        // csgo, league, valorant, dota
        CreateTeam::new(
            "Fnatic",
            "UK based esports team",
            "https://upload.wikimedia.org/wikipedia/en/4/43/Esports_organization_Fnatic_logo.svg",
        ),
        // csgo, rocket league
        CreateTeam::new(
            "Natus Vincere",
            "Ukraine based esports team",
            "https://upload.wikimedia.org/wikipedia/en/a/ac/NaVi_logo.svg",
        ),
        // valorant, league,
        CreateTeam::new(
            "Cloud9",
            "US based esports team",
            "https://liquipedia.net/commons/images/0/01/Cloud9_full_lightmode.png",
        ),
        // dota 2. rocket league, csgo, valorant
        CreateTeam::new(
            "Team Liquid",
            "Netherlands / US based esports team",
            "https://upload.wikimedia.org/wikipedia/en/f/f1/Team_Liquid_logo.svg",
        ),
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

    let mut game_ids = Vec::new();

    for game in games() {
        println!("[structure] Game '{}' being created:", &game.name);
        let gid = pg_game.create(game).await?;
        game_ids.push(gid);
        println!("[server] Game[{}] created!\n", &gid);
    }

    let mut team_ids = Vec::new();

    for team in teams() {
        println!("[structure] Team '{}' being created:", &team.name);
        let tid = pg_team.create(team).await?;
        team_ids.push(tid);
        println!("[server] Team[{}] created!\n", &tid);
    }

    let game_team_bindings = [
        // team astralis
        (team_ids[0], game_ids[0]),
        (team_ids[0], game_ids[2]),
        //
        // team fnatic
        (team_ids[1], game_ids[0]),
        (team_ids[1], game_ids[1]),
        (team_ids[1], game_ids[2]),
        (team_ids[1], game_ids[3]),
        //
        // team navi
        (team_ids[2], game_ids[0]),
        (team_ids[2], game_ids[4]),
        //
        // team cloud9
        (team_ids[3], game_ids[2]),
        (team_ids[3], game_ids[3]),
        //
        // team liquid
        (team_ids[4], game_ids[0]),
        (team_ids[4], game_ids[2]),
        (team_ids[4], game_ids[3]),
        (team_ids[4], game_ids[4]),
    ];

    // add teams to games
    for (team, game) in game_team_bindings {
        pg_team.add_to_game(team, game).await?;
        println!("[server] Team [{}] bound to Game [{}]", team, game);
    }

    Ok(())
}
