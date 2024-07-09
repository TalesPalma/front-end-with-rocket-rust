use rocket::FromForm;

#[derive(Debug, FromForm)]
pub struct ClientForm {
    pub name: String,
    pub email: String,
    pub phone: String,
}
