pub fn login(cred: models::Credentials) {
    crate::database::get_user(cred)
}
fn logout() {
    // logs out some user
}

pub mod models;
