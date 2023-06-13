use std::collections::HashMap;

use anyhow::anyhow;
use bson::Document;
use mongodb::bson::doc;
use pbkdf2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Pbkdf2,
};
use rand::Rng;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Account {
    pub public_key: String,
    pub private_key: String,
    pub public: Public,
    private: Private,
}

#[derive(Deserialize, Debug)]
pub struct Private {
    email: String,
    passhash: String,
    chat_map: HashMap<String, String>,
}

#[derive(Deserialize, Debug)]
pub struct Public {
    pub username: String,
    pub picture: String,
    pub contacts: Vec<String>,
    pub whitelist: Vec<String>,
    pub blacklist: Vec<String>,
}

impl Account {
    pub fn new(
        username: String,
        password: String,
        email: String,
        picture: String,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            public_key: get_id(15),
            private_key: get_id(47),
            public: Self::new_public(username, picture),
            private: Self::new_private(email, password)?,
        })
    }
    pub fn to_doc(&self) -> Document {
        doc! {
            "public_key" : self.public_key.clone(),
            "private_key" : self.private_key.clone(),
            "public" : self.public.to_doc(),
            "private"  : self.private.to_doc(),
        }
    }
    pub fn get_passhash(&self) -> String {
        self.private.passhash.clone()
    }
    fn new_private(email: String, password: String) -> anyhow::Result<Private, anyhow::Error> {
        let passhash = PasswordHandler::new(password.as_bytes())?;
        Ok(Private {
            email,
            passhash,
            chat_map: HashMap::new(),
        })
    }
    fn new_public(username: String, picture: String) -> Public {
        Public {
            username,
            picture,
            contacts: vec![],
            whitelist: vec![],
            blacklist: vec![],
        }
    }
}

impl Private {
    pub fn to_doc(&self) -> Document {
        doc! {
            "email": self.email.clone(),
            "passhash": self.passhash.clone(),
        }
    }
}

impl Public {
    pub fn to_doc(&self) -> Document {
        doc! {
            "username": self.username.clone(),
            "picture": self.picture.clone(),
            "contacts": self.contacts.clone(),
            "whitelist": self.whitelist.clone(),
            "blacklist": self.whitelist.clone(),
        }
    }
}

pub struct PasswordHandler;

impl PasswordHandler {
    pub fn new(input: &[u8]) -> anyhow::Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        match Pbkdf2.hash_password(input, &salt) {
            Ok(str) => Ok(str.to_string()),
            Err(err) => Err(anyhow!("{:?}", err)),
        }
    }
    pub fn verify(input: &[u8], hash: String) -> anyhow::Result<bool> {
        let parsed_hash = match PasswordHash::new(&hash) {
            Ok(v) => v,
            Err(err) => return Err(anyhow!("{:?}", err)),
        };
        match Pbkdf2.verify_password(input, &parsed_hash) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}

pub fn get_id(index: usize) -> String {
    let mut s = "".to_string();
    for _ in 0..index {
        s.push(rand::thread_rng().gen_range(33..126) as u8 as char);
    }
    s
}
