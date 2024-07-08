#[macro_use]
mod controllers;
mod models;
mod services;

extern crate rocket;
use controllers::*;
use rocket::{launch, routes};
use rocket_dyn_templates::Template;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                home_controller::index,
                clients_contoller::clientes,
                home_controller::hello
            ],
        )
        .attach(Template::fairing())
}
