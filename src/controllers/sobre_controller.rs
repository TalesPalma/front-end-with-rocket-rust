use rocket::get;
use rocket_dyn_templates::{context, Template};

#[get("/sobre")]
pub fn sobre() -> Template {
    Template::render("sobre/index", context! {})
}
