#![allow(unused)]
mod auth_utils;
mod database;

use crate::auth_utils::login;
use crate::auth_utils::models::Credentials;
use crate::database::Status;
use crate::database::connect_to_database;

pub fn authenticate(cred: Credentials) {
    if let Status::Connected = connect_to_database() {
        login(cred)
    }
}
fn main() {
    println!("Hello, world!");
}
