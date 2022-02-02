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

pub async fn edit_user() -> anyhow::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;
    let database_connection_pool: Arc<PgPool> =
        Arc::new(db_connect_create_pool(&database_url).await?);
    let pg_user: PgUserRepo = PgUserRepo::new(&database_connection_pool);

    println!("EDIT a user");

    let messages = [
        "User's ID:",
        "Edit User's first name:",
        "Edit User's last name:",
        "Edit User's civil ID number:",
        "Edit User's email:",
        "Edit User's phone number:",
    ];

    let mut input;
    let mut results: Vec<String> = Vec::new();

    for message in messages {
        println!("{}", message);
        input = "".to_string();
        io::stdin().read_line(&mut input)?;
        results.push(input.clone());
    }

    let results: Vec<Option<&str>> = results
        .iter()
        .map(|user_input| user_input.trim())
        .map(|string| if string == "" { None } else { Some(string) })
        .collect();

    let user = pg_user.get(results[0].unwrap_or("0").parse()?).await?;

    match pg_user
        .edit(
            user.id,
            user.edit_user(
                results[1], results[2], None, results[3], None, results[4], results[5], None,
            ),
        )
        .await
    {
        Ok(_) => println!("User {} updated", user.id),
        Err(error) => println!("ERROR: {}", error),
    }

    Ok(())
}
