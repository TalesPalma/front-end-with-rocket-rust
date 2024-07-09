use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Client {
    pub id: i32,
    pub name: String,
    pub email: String,
}
