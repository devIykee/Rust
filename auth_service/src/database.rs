pub enum Status {
    Connected,
    Disconnected,
}

pub fn connect_to_database() -> Status {
    Status::Connected
}

pub fn get_user(cred: crate::auth_utils::models::Credentials) {
    // returns user data
}
