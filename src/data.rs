use crate::table::{Account, Message};
use crate::{cryption, table::Chat};
use anyhow::anyhow;
use serde::Deserialize;
use serde_json::json;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    sql::Thing,
    Surreal,
};

pub struct Database {
    pub con: Surreal<Client>,
}

#[derive(Deserialize, Debug)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

impl Database {
    pub async fn new(
        uri: &'static str,
        username: Option<&'static str>,
        password: Option<&'static str>,
    ) -> anyhow::Result<Database> {
        let con = Surreal::new::<Ws>(uri).await?;
        con.signin(Root {
            username: username.unwrap_or("root"),
            password: password.unwrap_or("root"),
        });
        con.use_ns("namespace").use_db("database").await?;
        anyhow::Ok(Database { con })
    }
    pub async fn signup(
        &self,
        email: String,
        username: String,
        password: String,
    ) -> anyhow::Result<()> {
        self.con
            .signin(Root {
                username: "root",
                password: "root",
            })
            .await?;
        self.con.use_ns("joe").use_db("database").await?;
        let mut result = self
            .con
            .query(r#"SELECT session FROM user WHERE email = $email"#)
            .bind(("email", email.clone()))
            .await?;
        let obj: Option<String> = result.take((0, "session"))?;
        if obj.is_some() {
            return Err(anyhow!("Account already exsists".to_string()));
        }
        let session = cryption::get_id(64).iter().collect();
        let _created: Account = self
            .con
            .create("user")
            .content(Account {
                username,
                passhash: cryption::new(password.as_bytes())?,
                session,
                chats: vec![],
                email,
                picture: "binary".into(),
            })
            .await?;
        Ok(())
    }
    pub async fn login(&self, email: String, password: String) -> anyhow::Result<String> {
        self.con
            .signin(Root {
                username: "root",
                password: "root",
            })
            .await?;
        self.con.use_ns("joe").use_db("database").await?;
        let mut result = self
            .con
            .query("select passhash,session from user where (email = $email)")
            .bind(("email", email))
            .await?;
        let account: Option<String> = result.take((0, "passhash"))?;
        if let Some(passhash) = account {
            if cryption::verify(password.as_bytes(), passhash)? {
                let session: Option<String> = result.take((0, "session"))?;
                match session {
                    Some(v) => Ok(v),
                    None => Err(anyhow!("wrong userdata structure".to_string())),
                }
            } else {
                Err(anyhow!("wrong userdata".to_string()))
            }
        } else {
            Err(anyhow!("no such account"))
        }
    }
    pub async fn get_data(&self, sid: String) -> anyhow::Result<String> {
        let mut result = self
            .con
            .query("SELECT username,picture,chats,id FROM user WHERE (session = $sid)")
            .bind(("sid", sid))
            .await?;
        let account: Option<String> = result.take((0, "username"))?;
        match account {
            Some(username) => {
                let picture: Option<String> = result.take("picture")?;
                let chats: Option<Vec<String>> = result.take("chats")?;
                let id_unfor: Option<Thing> = result.take("id")?;
                let id: String = match id_unfor {
                    Some(v) => v.id.to_raw(),
                    None => {
                        return Err(anyhow!("no such account".to_string()));
                    }
                };
                Ok(json!({
                    "username" : username,
                    "picture" : picture,
                    "chats" : chats,
                    "id" : id
                })
                .to_string())
            }
            None => Err(anyhow!("no such account".to_string())),
        }
    }
    pub async fn get_chat(&self, users: Vec<String>) -> anyhow::Result<String> {
        let mut chat: Option<Thing> = None;
        // remove duplicates of users
        if false {
            Err(anyhow!("impossible"))
        } else {
            let mut users_thing: Vec<Thing> = Vec::new();
            for user_u_p in users.iter() {
                let user = string_into_thing(user_u_p)?;
                if !self.exsists(&user).await? {
                    return Err(anyhow!("input data is inccorect"));
                }
                users_thing.push(user);
            }
            chat = Some(self.create_chat().await?);

            if chat.is_none() {
                return Err(anyhow!("chat couldn't be initiated"));
            }
            self.chat_init(users_thing, &chat.clone().unwrap()).await?;
            Ok(chat.unwrap().id.to_string())
        }
    }
    pub async fn get_username(&self, sid: String) -> anyhow::Result<String> {
        let mut result = self
            .con
            .query("SELECT username FROM user WHERE (session = $sid)")
            .bind(("sid", sid))
            .await?;
        let r: Option<String> = result.take((0, "username"))?;
        match r {
            Some(v) => Ok(v),
            None => Err(anyhow!("No session in this account")),
        }
    }
    pub async fn insert_to_chat(&self, chat_id: String, msg: Message) -> anyhow::Result<String> {
        let mut _insertion = self
            .con
            .query("update $chat_id set messages += $msg")
            .bind(("chat_id", chat_id))
            .bind(("msg", msg))
            .await?;
        Ok("".to_string())
    }
    pub async fn get_id(&self, sid: String) -> anyhow::Result<String> {
        self.con
            .signin(Root {
                username: "root",
                password: "root",
            })
            .await?;
        self.con.use_ns("joe").use_db("database").await?;
        let mut result = self
            .con
            .query("SELECT id FROM user WHERE (session = $sid)")
            .bind(("sid", sid))
            .await?;
        let r: Option<Thing> = result.take((0, "id"))?;
        match r {
            Some(v) => Ok(format!("{v}")),
            None => Err(anyhow!("no id in this account")),
        }
    }

    async fn chat_init(&self, users: Vec<Thing>, chat: &Thing) -> anyhow::Result<()> {
        self.con
            .signin(Root {
                username: "root",
                password: "root",
            })
            .await?;
        self.con.use_ns("joe").use_db("database").await?;
        /*let mut _user_insertion = self
        .con
        .query("update username")
        .bind(("username", username.clone()))
        .await?;*/
        for id in users.iter() {
            let mut _chat_insertion = self
                .con
                .query("update $chat set members += $id")
                .bind(("chat", chat.clone()))
                .bind(("id", id.clone()))
                .await?;
            let mut _user_insertion = self
                .con
                .query("update $id set chats += $chat")
                .bind(("id", id.clone()))
                .bind(("chat", chat.clone()))
                .await?;
        }
        Ok(())
    }
    async fn create_chat(&self) -> anyhow::Result<Thing> {
        let created: Record = self
            .con
            .create("chat")
            .content(Chat {
                members: vec![],
                messages: vec![],
            })
            .await?;
        Ok(created.id)
    }
    async fn exsists(&self, id: &Thing) -> anyhow::Result<bool> {
        self.con
            .signin(Root {
                username: "root",
                password: "root",
            })
            .await?;
        self.con.use_ns("joe").use_db("database").await?;
        let mut result = self
            .con
            .query("select id from user where id = $id")
            .bind(("id", id))
            .await?;
        let user: Option<Thing> = result.take("id")?;
        Ok(user.is_some())
    }
}
fn string_into_thing(s: &String) -> anyhow::Result<Thing> {
    match s.as_str().split_once(":") {
        Some(r) => Ok(Thing::from(r)),
        None => Err(anyhow!("couldn't convert string into record")),
    }
}
