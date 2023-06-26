use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Account {
    pub username: String,
    pub passhash: String,
    pub session: String,
    pub picture: String,
    pub email: String,
    pub chats: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Chat {
    pub members: Vec<String>,
    pub messages: Vec<Message>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Message {
    pub date: String,
    pub text: String,
    pub owner: String,
}
