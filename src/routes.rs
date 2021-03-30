use rocket::Route;
use rocket::Catcher;
use crate::api::controllers::{root, users};

pub const ROOT_PATH: &str = "/api";

pub fn api_catchers() -> Vec<Catcher> {
    catchers![root::not_found]
}

pub fn api_routes() -> Vec<Route> {
    routes![
        root::get_api,
        users::get_users
    ]
}
