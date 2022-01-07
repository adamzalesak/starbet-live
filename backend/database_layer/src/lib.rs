#![allow(dead_code)]

#[macro_use]
extern crate diesel;

// pub mod data_types;
pub mod handlers;
mod models;
mod schema;

async fn hehe() {
    match handlers::database_connection::establish_connection(
        "postgres://postgres:postgres@localhost:5432/postgres",
    )
    .await
    {
        Ok(_) => println!("Correct!"),
        Err(err) => println!("{:?}", err),
    }
}
