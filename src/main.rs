mod data;
mod server;
mod session;
mod table;
mod utils;

use actix::*;
use actix_files::Files;
use actix_session::{
    config::PersistentSession, storage::CookieSessionStore, Session, SessionMiddleware,
};
use actix_web::{
    cookie::Key, get, middleware::Logger, web, App, HttpRequest, HttpResponse, HttpServer,
};
use actix_web_actors::ws;
use mongodb::{options::ClientOptions, Client};
use serde_json::json;
use utils::jsontostring;
use uuid::Uuid;

async fn socket(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<server::ChatServer>>,
    client: web::Data<data::DataHandler>,
    session: Session,
) -> HttpResponse {
    if let Some(sid) = session.get::<String>("sessionid").unwrap_or(None) {
        let username = match client.get_username_private(sid).await {
            Ok(username) => username,
            Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
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
        HttpResponse::Forbidden().body(jsontostring(&serde_json::json!({
            "error": "No Session"}
        )))
    }
}

#[get("/api/signup/{username}/{password}")]
async fn signup(
    data: web::Path<(String, String)>,
    client: web::Data<data::DataHandler>,
) -> HttpResponse {
    let (username, password) = data.into_inner();
    match client.add_user(username, password).await {
        Ok(v) => HttpResponse::Ok().body(v),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/api/login/{username}/{password}")]
async fn login(
    data: web::Path<(String, String)>,
    client: web::Data<data::DataHandler>,
    session: Session,
) -> HttpResponse {
    let (username, password) = data.into_inner();
    match client.check_user(username, password, session).await {
        Ok(v) => HttpResponse::Ok().body(v),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/api/add_contact/{username}")]
async fn add_contact(
    data: web::Path<String>,
    client: web::Data<data::DataHandler>,
    session: Session,
) -> HttpResponse {
    if let Some(id) = session.get::<String>("sessionid").unwrap_or(None) {
        let username = data.into_inner();
        match client.add_contact(username, id).await {
            Ok(_) => HttpResponse::Ok().body("Contact added"),
            Err(err) => HttpResponse::Forbidden().body(err.to_string()),
        }
    } else {
        HttpResponse::Forbidden().body(jsontostring(&serde_json::json!({
            "error": "No Session"}
        )))
    }
}
#[get("/api/getdata")]
async fn getdata(client: web::Data<data::DataHandler>, session: Session) -> HttpResponse {
    if let Some(id) = session.get::<String>("sessionid").unwrap_or(None) {
        match client.get_userdata(id).await {
            Ok(arr) => {
                println!("{arr:?}");
                HttpResponse::Ok().body(arr)
            }
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        }
    } else {
        HttpResponse::InternalServerError().body(jsontostring(&serde_json::json!({
            "error": "No Session"}
        )))
    }
}

#[get("/api/valid")]
async fn valid(session: Session, client: web::Data<data::DataHandler>) -> HttpResponse {
    if let Some(id) = session.get::<String>("sessionid").unwrap_or(None) {
        if client.valid(id).await {
            HttpResponse::Ok().body("true".to_string())
        } else {
            HttpResponse::Ok().body("false".to_string())
        }
    } else {
        HttpResponse::Ok().body("false".to_string())
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let server = web::Data::new(server::ChatServer::new().start());
    let db_uri = "mongodb://127.0.0.1:27017";
    let client_options = ClientOptions::parse(db_uri).await.unwrap();
    let mongodb = web::Data::new(data::DataHandler(
        Client::with_options(client_options).unwrap(),
        uuid::Uuid::new_v4(),
    ));
    HttpServer::new(move || {
        App::new()
            .app_data(mongodb.clone())
            .app_data(server.clone())
            ////////////////////////////////
            //.service(socket)
            .route("/ws", web::get().to(socket))
            .service(signup)
            .service(login)
            .service(getdata)
            .service(add_contact)
            .service(valid)
            .service(
                Files::new("/", "www/dist") // File server needed for Svelte GUI, has to be created as last
                    .prefer_utf8(true)
                    .index_file("index.html"),
            )
            .service(
                Files::new("/home", "www/dist/home") // File server needed for Svelte GUI, has to be created as last
                    .prefer_utf8(true)
                    .index_file("index.html"),
            )
            .service(
                Files::new("/auth", "www/dist/auth") // File server needed for Svelte GUI, has to be created as last
                    .prefer_utf8(true)
                    .index_file("index.html"),
            )
            ////////////////////////////////
            //.wrap(Logger::default())
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
    .workers(2)
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
