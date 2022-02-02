use dotenv::dotenv;
use std::env;
use std::io;
use std::sync::Arc;

use chrono::{TimeZone, Utc};
use database_layer::{
    connection::*,
    db_access::{
        repo::Repo,
        user::{PgUserRepo, UserRepo},
    },
    db_models::{
        user::{CreateUser, User},
        user_address::{CreateUserAddress, UserAddress},
    },
};

pub async fn create_user() -> anyhow::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;
    let database_connection_pool: Arc<PgPool> =
        Arc::new(db_connect_create_pool(&database_url).await?);
    let pg_user: PgUserRepo = PgUserRepo::new(&database_connection_pool);

    let messages = [
        "User's first name:",
        "User's last name:",
        "User's civil ID number:",
        "User's email:",
        "User's phone number:",
        "User's address - street name:",
        "User's address - street number:",
        "User's address - city:",
        "User's address - area:",
        "User's address - postal code:",
        "User's address - country:",
    ];

    let mut input;
    let mut results: Vec<String> = Vec::new();

    for message in messages {
        println!("{}", message);
        input = "".to_string();
        io::stdin().read_line(&mut input)?;
        results.push(input.clone());
    }

    let results: Vec<&str> = results.iter().map(|user_input| user_input.trim()).collect();

    let area = if results[8] == "" {
        None
    } else {
        Some(results[8])
    };

    let (user_id, address_id) = pg_user
        .create(
            CreateUser::new(
                results[0],
                results[1],
                "",
                results[2],
                results[3],
                &Utc::now().to_string(),
                results[4],
                None,
            ),
            CreateUserAddress::new(
                results[5],
                results[6],
                results[7],
                area,
                results[9],
                results[10],
            ),
        )
        .await?;

    println!(
        "User [{}] with address [{}] has been created",
        user_id, address_id
    );

    Ok(())
}
