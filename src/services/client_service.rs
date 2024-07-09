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

pub fn insert_client(clinent: Client) -> Client {
    Client {
        id: 4,
        name: clinent.name,
        email: clinent.email,
    }
}
