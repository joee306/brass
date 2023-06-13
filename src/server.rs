use actix::prelude::*;
use std::collections::HashMap;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

#[derive(Message)]
#[rtype(String)]
pub struct Connect {
    pub username: String,
    pub addr: Recipient<Message>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub username: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ListUsers;

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub text: String,
    pub resiver: String,
    pub sender: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ContactAdd {
    pub requester: String,
}

pub struct ChatServer {
    pub session: HashMap<String, Recipient<Message>>,
}

impl ChatServer {
    pub fn new() -> ChatServer {
        ChatServer {
            session: HashMap::new(),
        }
    }
}

impl Actor for ChatServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for ChatServer {
    type Result = String;
    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> String {
        self.session.insert(msg.username.clone(), msg.addr);
        println!("new user : {}", msg.username);
        msg.username
    }
}

impl Handler<Disconnect> for ChatServer {
    type Result = ();
    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        self.session.remove(&msg.username);
    }
}

impl Handler<ClientMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
        if let Some(addr) = self.session.get(&msg.resiver) {
            addr.do_send(Message(msg.text));
        } else {
            println!("no such User as : {}", msg.resiver);
        }
    }
}

impl Handler<ListUsers> for ChatServer {
    type Result = ();

    fn handle(&mut self, _: ListUsers, _: &mut Context<Self>) {
        println!("{:#?}", self.session);
    }
}
