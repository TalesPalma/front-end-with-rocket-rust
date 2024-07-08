use std::collections::HashMap;

use rocket::get;
use rocket_dyn_templates::{context, Template};

#[get("/")]
pub fn index() -> Template {
    let mut context = HashMap::new();
    context.insert("title", "Home page");
    Template::render("home/index", &context)
}

#[get("/hello")]
pub fn hello() -> Template {
    Template::render(
        "layouts/application",
        context! {title_page:"Titulo da pagina"},
    )
}
