mod cryption;
mod data;
mod routes;
mod server;
mod session;
mod table;
mod utils;

use routes::*;

use actix::*;
use actix_files::Files;
use actix_session::{
    config::PersistentSession, storage::CookieSessionStore, Session, SessionMiddleware,
};
use actix_web::{cookie::Key, web, App, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use serde_json::json;

async fn socket(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<server::ChatServer>>,
    session: Session,
    db: web::Data<data::Database>,
) -> HttpResponse {
    if let Some(sid) = session.get::<String>("sid").unwrap_or(None) {
        let username = match db.get_username(sid).await {
            Ok(v) => v,
            Err(err) => {
                return HttpResponse::Forbidden()
                    .body(json!({ "error": err.to_string() }).to_string())
            }
        };
        let ws = ws::start(
            session::SocketSession {
                username,
                addr: srv.get_ref().clone(),
            },
            &req,
            stream,
        );
        match ws {
            Ok(response) => response,
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        }
    } else {
        HttpResponse::Forbidden().body(
            json!({
                "error": "No Session"}
            )
            .to_string(),
        )
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let server = web::Data::new(server::ChatServer::new().start());
    let db: web::Data<data::Database> = web::Data::new(
        data::Database::new("localhost:8000", None, None)
            .await
            .unwrap(),
    );
    HttpServer::new(move || {
        App::new()
            .app_data(server.clone())
            .app_data(db.clone())
            .route("/ws", web::get().to(socket))
            .service(signup)
            .service(login)
            .service(get_data)
            .service(message)
            .service(get_chat)
            .service(
                Files::new("/", "www/dist")
                    .prefer_utf8(true)
                    .index_file("index.html"),
            )
            .service(
                Files::new("/home", "www/dist/home")
                    .prefer_utf8(true)
                    .index_file("index.html"),
            )
            .service(
                Files::new("/auth", "www/dist/auth")
                    .prefer_utf8(true)
                    .index_file("index.html"),
            )
            //            .wrap(Logger::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                    .cookie_secure(false)
                    // customize session and cookie expiration
                    .session_lifecycle(
                        PersistentSession::default(), //.session_ttl(actix_web::cookie::time::Duration::hours(2)),
                    )
                    .build(),
            )
    })
    .workers(4)
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
