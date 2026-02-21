#![allow(unused)]

struct Credentials {
    username: String,
    password: String,
}

enum Status {
    Connected,
    Disconnected,
}

fn connect_to_database() -> Status {
    Status::Connected
}

fn login(cred: Credentials) {
    get_user()
}
fn logout() {
    // logs out some user
}

fn get_user() {
    // returns user data
}
fn authenticate(cred: Credentials) {
    if let Status::Connected = connect_to_database() {
        login(cred)
    }
}
fn main() {
    println!("Hello, world!");
}
