use crate::server;
use actix::{
    fut, Actor, ActorContext, ActorFutureExt, Addr, AsyncContext, ContextFutureSpawner, Handler,
    StreamHandler, WrapFuture,
};
use actix_web_actors::ws;

pub struct SocketSession {
    pub username: String,
    pub addr: Addr<server::ChatServer>,
}

impl Actor for SocketSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        self.addr
            .send(server::Connect {
                username: self.username.clone(),
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.username = res,
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }
}

impl Handler<server::Message> for SocketSession {
    type Result = ();

    fn handle(&mut self, msg: server::Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for SocketSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        match msg {
            ws::Message::Text(t) => {
                println!("{t}");
                let text = t.trim();
                let parts: Vec<&str> = text.splitn(2, '/').collect();
                if parts[0] == "LU" {
                    self.addr.do_send(server::ListUsers);
                } else {
                    if parts.len() > 1 {
                        self.addr.do_send(server::ClientMessage {
                            resiver: parts[0].to_string(),
                            text: parts[1].to_string(),
                            sender: self.username.clone(),
                        });
                    }
                }
            }
            ws::Message::Binary(_) => println!("Unexpected binary"),
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            ws::Message::Continuation(_) => {
                ctx.stop();
            }
            _ => (),
        }
    }
}
