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

pub async fn new_address() -> anyhow::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;
    let database_connection_pool: Arc<PgPool> =
        Arc::new(db_connect_create_pool(&database_url).await?);
    let pg_user: PgUserRepo = PgUserRepo::new(&database_connection_pool);

    println!("Add a new address to the user:");

    let messages = [
        "User's ID:",
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

    let area = if results[4] == "" {
        None
    } else {
        Some(results[4])
    };

    let user_id = results[0].parse()?;

    match pg_user
        .add_new_address(
            user_id,
            CreateUserAddress::new(
                results[1], results[2], results[3], area, results[5], results[6],
            ),
        )
        .await
    {
        Ok(id) => {
            println!("User [{}] with address [{}] has been created", user_id, id);
        }
        Err(error) => println!("ERROR: {}", error),
    }

    Ok(())
}
