use crate::data;
use actix_session::Session;
use actix_web::{get, web, HttpResponse};
use serde_json::json;

#[get("/api/signup/{email}/{username}/{password}")]
pub async fn signup(
    data: web::Path<(String, String, String)>,
    db: web::Data<data::Database>,
) -> HttpResponse {
    let (email, username, password) = data.into_inner();
    match db.signup(email, username, password).await {
        Ok(_) => HttpResponse::Ok().body(""),
        Err(err) => HttpResponse::Forbidden().body(json!({"error" : err.to_string()}).to_string()),
    }
}

#[get("/api/login/{email}/{password}")]
pub async fn login(
    data: web::Path<(String, String)>,
    db: web::Data<data::Database>,
    session: Session,
) -> HttpResponse {
    let (email, password) = data.into_inner();
    match db.login(email, password).await {
        Ok(sid) => {
            if session.insert("sid", sid).is_ok() {
                HttpResponse::Ok().body("session set")
            } else {
                HttpResponse::TemporaryRedirect().body("couldn't set your session id")
            }
        }
        Err(err) => HttpResponse::Forbidden().body(json!({"error" : err.to_string()}).to_string()),
    }
}

#[get("/api/getdata")]
pub async fn get_data(db: web::Data<data::Database>, session: Session) -> HttpResponse {
    match session.get("sid").unwrap_or(None) {
        Some(sid) => match db.get_data(sid).await {
            Ok(v) => HttpResponse::Ok().body(v),
            Err(err) => {
                HttpResponse::Forbidden().body(json!({ "error": err.to_string()}).to_string())
            }
        },
        None => HttpResponse::Forbidden().body(json!({"error" : "no session id"}).to_string()),
    }
}
#[get("/api/message/{reciver}/{text}")]
pub async fn message(
    db: web::Data<data::Database>,
    session: Session,
    data: web::Path<(String, String)>,
) -> HttpResponse {
    let (reciver, text) = data.into_inner();
    match session.get("sid").unwrap_or(None) {
        Some(sid) => {
            let owner = match db.get_id(sid).await {
                Ok(v) => v,
                Err(err) => {
                    return HttpResponse::Forbidden()
                        .body(json!({ "error": err.to_string()}).to_string());
                }
            };
            let chat = match db.get_chat(vec![reciver, owner]).await {
                Ok(v) => HttpResponse::Ok().body(v),
                Err(err) => {
                    HttpResponse::Forbidden().body(json!({ "error": err.to_string()}).to_string())
                }
            };
            chat
        }
        None => HttpResponse::Forbidden().body(json!({"error" : "no session id"}).to_string()),
    }
}

#[get("/api/get_chat/{id}")]
pub async fn get_chat(
    db: web::Data<data::Database>,
    session: Session,
    data: web::Path<String>,
) -> HttpResponse {
    unreachable!()
}
