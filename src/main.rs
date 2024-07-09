#[macro_use]
mod controllers {
    pub mod about_controller;
    pub mod clients_controller;
    pub mod home_controller;
    pub mod services_controller;
} //Another approach to declaring modules

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
                about_controller::about,
                services_controller::services,
                clients_controller::clients,
                clients_controller::new,
            ],
        )
        .mount("/static", FileServer::from(relative!("static")))
        .attach(Template::fairing())
}
