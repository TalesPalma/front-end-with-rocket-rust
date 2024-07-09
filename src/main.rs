#[macro_use]
mod controllers;
mod models;
mod services;

extern crate rocket;
use controllers::*;
use rocket::{
    fs::{relative, FileServer},
    launch, routes,
};
use rocket_dyn_templates::Template;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                home_controller::index,
                home_controller::hello,
                sobre_controller::sobre,
                services_controller::servicos,
                clients_controller::clientes,
                clients_controller::new,
            ],
        )
        .mount("/static", FileServer::from(relative!("static")))
        .attach(Template::fairing())
}
