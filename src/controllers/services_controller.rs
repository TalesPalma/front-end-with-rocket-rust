use rocket::get;
use rocket_dyn_templates::{context, Template};

#[get("/services")]
pub fn services() -> Template {
    Template::render("services/index", context! {})
}
