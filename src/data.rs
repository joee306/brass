use super::utils::jsontostring;
use crate::table;
use actix_session::Session;
use anyhow::anyhow;
use bson::Bson;
use mongodb::{
    bson::{doc, Document},
    Client,
};
use serde_json::{json, Value};
use std::result::Result::Ok;
use table::*;
use uuid::Uuid;

const db_name: &'static str = "line-db";

pub struct DataHandler(pub Client, pub Uuid);
#[derive(serde::Deserialize)]
struct Message {
    sender: String,
    text: String,
}
#[derive(serde::Deserialize)]
struct ChatDoc {
    id: String,
    history: Vec<Message>,
}

impl Into<Bson> for Message {
    fn into(self) -> Bson {
        doc! {
            "sender" : self.sender,
            "text" : self.text,
        }
        .into()
    }
}

impl DataHandler {
    pub async fn add_user(&self, username: String, password: String) -> anyhow::Result<String> {
        let collection = self.0.database(db_name).collection::<Document>("users");
        let filter = doc! { "public.username": username.clone() };
        let result = collection.find_one(filter, None).await?;
        match result {
            Some(_) => {
                return Err(anyhow!("Account already exists"));
            }
            None => (),
        }
        let ac = table::Account::new(username, password, "".to_string(), "".to_string())?;
        collection.insert_one(ac.to_doc(), None).await?;
        Ok(ac.private_key)
    }
    pub async fn check_user(
        &self,
        username: String,
        password: String,
        session: Session,
    ) -> anyhow::Result<String> {
        let collection = self.0.database(db_name).collection::<Document>("users");
        let filter = doc! { "public.username": username.clone() };
        let result = collection.find_one(filter, None).await?;
        let ac: Account = match result {
            Some(doc) => match bson::from_bson::<Account>(Bson::Document(doc)) {
                Ok(ac) => ac,
                Err(err) => return Err(anyhow!("login error : {}", err.to_string())),
            },
            None => return Err(anyhow!("Userdata wrong")),
        };
        match PasswordHandler::verify(password.as_bytes(), ac.get_passhash()) {
            Ok(b) => {
                if b {
                    match session.insert("sessionid", ac.private_key) {
                        Ok(_) => Ok("Session set".to_string()),
                        Err(err) => Err(anyhow!("error : {}", err.to_string())),
                    }
                } else {
                    Err(anyhow!("error : {}", "Userdata wrong"))
                }
            }
            Err(err) => Err(anyhow!("error : {}", err.to_string())),
        }
    }
    pub async fn valid(&self, id: String) -> bool {
        let collection = self.0.database(db_name).collection::<Document>("users");
        let filter = doc! { "private_key" : id };
        if let Ok(result) = collection.find_one(filter, None).await {
            match result {
                Some(_) => true,
                None => false,
            }
        } else {
            false
        }
    }
    pub async fn get_username_private(&self, id: String) -> anyhow::Result<String> {
        let collection = self.0.database(db_name).collection::<Document>("users");
        let filter = doc! { "private_key" : id };
        let result = collection.find_one(filter, None).await?;
        match result {
            Some(doc) => match bson::from_bson::<Account>(Bson::Document(doc)) {
                Ok(ac) => return anyhow::Ok(ac.public_key),
                Err(err) => return Err(anyhow!(err.to_string())),
            },
            None => {
                return Err(anyhow!(jsontostring(
                    &json!({"error" : "Wrong Session ID"})
                )))
            }
        }
    }
    pub async fn get_username_public(&self, id: String) -> anyhow::Result<String> {
        let collection = self.0.database(db_name).collection::<Document>("users");
        let filter = doc! { "public_key" : id };
        let result = collection.find_one(filter, None).await?;
        match result {
            Some(doc) => match bson::from_bson::<Account>(Bson::Document(doc)) {
                Ok(ac) => return anyhow::Ok(ac.public.username),
                Err(err) => return Err(anyhow!(err.to_string())),
            },
            None => {
                return Err(anyhow!(jsontostring(
                    &json!({"error" : "Wrong Session ID"})
                )))
            }
        }
    }

    pub async fn get_publickey(&self, username: String) -> anyhow::Result<Option<String>> {
        let collection = self.0.database(db_name).collection::<Document>("users");
        let filter = doc! { "public.username" : username };
        match collection.find_one(filter, None).await {
            Ok(Some(doc)) => match bson::from_bson::<Account>(Bson::Document(doc)) {
                Ok(ac) => Ok(Some(ac.public_key)),
                Err(err) => Err(anyhow!(err.to_string())),
            },
            Ok(None) => Ok(None),
            Err(err) => Err(anyhow!(err.to_string())),
        }
    }

    pub async fn add_contact(&self, username: String, private_key: String) -> anyhow::Result<()> {
        let collection = self.0.database(db_name).collection::<Document>("users");
        let filter = doc! { "private_key" : private_key};
        let mut contacts: Vec<String> = vec![];
        if let Some(doc) = collection.find_one(filter.clone(), None).await? {
            match bson::from_bson::<Account>(Bson::Document(doc)) {
                Ok(ac) => contacts = ac.public.contacts,
                Err(err) => return Err(anyhow!(err.to_string())),
            };
            match self.get_publickey(username).await {
                Ok(Some(public_key)) => {
                    if contacts.contains(&public_key) {
                        return Err(anyhow!(jsontostring(&json!({"error" : "Already added"}))));
                    }
                    contacts.push(public_key);
                }
                Ok(None) => {
                    return Err(anyhow!(jsontostring(&json!({"error" : "No such Account"}))))
                }
                Err(err) => return Err(err),
            }
            let change = doc! {"$set" : {"public.contacts" : contacts}};
            match collection.update_one(filter, change, None).await {
                Ok(_) => Ok(()),
                Err(err) => Err(anyhow!(err.to_string())),
            }
        } else {
            Err(anyhow!(jsontostring(
                &json!({"error" : "Wrong Session ID"})
            )))
        }
    }
    pub async fn get_userdata(&self, id: String) -> anyhow::Result<String> {
        let collection = self.0.database(db_name).collection("users");
        let filter = doc! { "private_key" : id };
        let result = collection.find_one(filter, None).await?;
        match result {
            Some(doc) => {
                match bson::from_bson::<Account>(Bson::Document(doc)) {
                    Ok(ac) => {
                        let mut vec = vec![];
                        for key in ac.public.contacts.iter() {
                            vec.push(json!({
                                "username" : self.get_username_public(key.clone()).await?,
                                "public_key" : key,
                            }))
                        }
                        return anyhow::Ok(jsontostring(&json!(
                            {
                                "username" : ac.public.username,
                                "profile_picture" : ac.public.picture,
                                "contacts" : vec,
                            }
                        )));
                    }
                    Err(err) => return Err(anyhow!(err.to_string())),
                };
            }
            None => {
                return Err(anyhow!(jsontostring(
                    &json!({"error" : "Wrong Session ID"})
                )))
            }
        }
    }
    async fn new_document(
        &self,
        private_key: String,
        public_key: String,
    ) -> anyhow::Result<String> {
        let collection = self.0.database(db_name).collection("messages");
        let uuid = Uuid::new_v4().to_string();
        let vec: Vec<Message> = vec![];
        collection
            .insert_one(doc! { "id" : &uuid , "history" : vec}, None)
            .await?;
        anyhow::Ok(uuid)
    }
    pub async fn add_message(&self, id: String, message: String) -> anyhow::Result<()> {
        let collection = self.0.database(db_name).collection::<ChatDoc>("messages");

        /*let result = collection.find_one(doc! {"id" : &id}, None).await?;
        let doc = match result {
            Some(doc) => {
                match bson::from_bson::<ChatDoc>(Bson::Document(doc)) {
                    Ok(v) => v,
                    Err(err) => return Err(anyhow!(err.to_string())),
                };
            }
            None => {
                return Err(anyhow!(jsontostring(
                    &json!({"error" : "No such ChatCollection"})
                )))
            }
        };*/
        let filter = doc! { "id" : id};
        let change = doc! { "$push" : {"history" : message}};
        match collection.update_one(filter, change, None).await {
            Ok(v) => {
                println!("{v:?}");
                Ok(())
            }
            Err(err) => Err(anyhow!(err.to_string())),
        };
        anyhow::Ok(())
    }
}
