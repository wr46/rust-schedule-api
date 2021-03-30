#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;

mod routes;
mod api;

fn server() -> rocket::Rocket {
    rocket::ignite()
        .register(routes::api_catchers())
        .mount(routes::ROOT_PATH, routes::api_routes())
}

fn main() {
    server().launch();
}
