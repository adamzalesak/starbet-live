#![allow(dead_code)]

#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;
pub mod handlers;

async fn hehe() {
    match handlers::database_connection::establish_connection("postgres://postgres:postgres@localhost:5432/postgres").await {
        Ok(_) => println!("Correct!"),
        Err(err) => println!("{:?}", err),
    }
}