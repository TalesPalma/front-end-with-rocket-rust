use rocket::get;
use rocket_dyn_templates::{context, Template};

use crate::services::client_service::get_clients;

#[get("/clients")]
pub fn clients() -> Template {
    let clients = get_clients();
    Template::render("clients/index", context! {clients_list: &clients})
}

#[get("/clientes/new")]
pub fn new() -> Template {
    Template::render("clients/new", context! {})
}
