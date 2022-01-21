use anyhow::Result;
use dotenv::dotenv;
use std::env;
use std::sync::Arc;

use database_layer::{
    connection::db_connect_create_pool,
    db_access::{
        repo::Repo,
        user::{PgUserRepo, UserRepo},
    },
    db_models::user::CreateUser,
};

#[tokio::main]
async fn main() -> Result<()> {
    // for development purposes only, using dotenv to store the connection string
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;

    // create one pool of database connections, the reference will be stored
    // in every repo and the connections will be used up when necessarry
    let database_connection_pool = Arc::new(db_connect_create_pool(&database_url).await?);

    // creating a user repository
    let user_repository = PgUserRepo::new(&database_connection_pool);

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

    let new_user = CreateUser::new(
        "Janko",
        "Matuska",
        "ER125432",
        "janko@matuska.sk",
        "+421987654",
        None,
    );

    //
    // ==========================
    // NOW let's use the db
    // ==========================
    //

    let id_new = user_repository.create(new_user).await?;

    println!("User with ID {} created!", &id_new);

    let user_from_db = user_repository.get(id_new).await?;

    println!(
        "USER {} {} (id: {}) from main",
        &user_from_db.first_name, &user_from_db.last_name, &user_from_db.id
    );

    Ok(())
}
