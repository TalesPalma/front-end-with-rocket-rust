use rocket::get;
use rocket_dyn_templates::{context, Template};

#[get("/servicos")]
pub fn servicos() -> Template {
    Template::render("servicos/index", context! {})
}
