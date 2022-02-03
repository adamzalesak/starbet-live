#![allow(dead_code, unused_variables, unused_imports)]
// #![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

#[macro_use]
extern crate diesel;

pub mod connection;
pub mod db_access;
pub mod db_models;
pub mod result_types;
mod schema;
pub mod type_storing;
