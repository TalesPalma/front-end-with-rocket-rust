use rocket::http::ext::IntoCollection;

use crate::models::client::Client;

pub fn get_clients() -> Vec<Client> {
    vec![
        Client {
            id: 1,
            name: "Tales".to_string(),
            email: "pUqgU@example.com".to_string(),
        },
        Client {
            id: 2,
            name: "Tales segundo".to_string(),
            email: "pUqgU@example.com".to_string(),
        },
        Client {
            id: 3,
            name: "Tales Terceiro".to_string(),
            email: "pUqgU@example.com".to_string(),
        },
    ]
}

pub fn get_clients_by_id(id: i32) -> Option<Client> {
    let list = vec![
        Client {
            id: 1,
            name: "Tales".to_string(),
            email: "pUqgU@example.com".to_string(),
        },
        Client {
            id: 2,
            name: "Tales segundo".to_string(),
            email: "pUqgU@example.com".to_string(),
        },
        Client {
            id: 3,
            name: "Tales Terceiro".to_string(),
            email: "pUqgU@example.com".to_string(),
        },
    ];
    list.iter().find(|c| c.id == id).cloned()
}

pub fn delete_client(id: i32) -> Vec<Client> {
    let mut list = vec![
        Client {
            id: 1,
            name: "Tales".to_string(),
            email: "pUqgU@example.com".to_string(),
        },
        Client {
            id: 2,
            name: "Tales segundo".to_string(),
            email: "pUqgU@example.com".to_string(),
        },
        Client {
            id: 3,
            name: "Tales Terceiro".to_string(),
            email: "pUqgU@example.com".to_string(),
        },
    ];
    list.retain(|c| c.id != id);
    list
}

//pub fn insert_client(clinent: Client) -> Client {
//    Client {
//        id: 4,
//        name: clinent.name,
//        email: clinent.email,
//    }
//}
