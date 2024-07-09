use rocket::{form::Form, get, post};
use rocket_dyn_templates::{context, Template};

use crate::models::client_form::ClientForm;
use crate::{models::client::Client, services::client_service::get_clients};
#[get("/clients")]
pub fn clients() -> Template {
    let clients = get_clients();
    Template::render("clients/index", context! {clients_list: &clients})
}

#[get("/clients/new")]
pub fn new() -> Template {
    Template::render("clients/new", context! {})
}

#[post("/clients/created", data = "<client_form>")]
pub fn created(client_form: Form<ClientForm>) -> Template {
    let new_client = client_form.into_inner();

    let mut clients = get_clients();
    clients.push(Client {
        id: 0,
        name: new_client.name.clone(),
        email: new_client.email.clone(),
    });

    Template::render("clients/index", context! {clients_list: &clients})
}

#[get("/clients/<id>/edit")]
pub fn edit(id: i32) -> Template {
    if let Some(client) = get_clients().iter().find(|c| c.id == id) {
        Template::render("clients/edit", context! { client: &client})
    } else {
        Template::render("clients/index", context! {clients_list: &get_clients()})
    }
}
