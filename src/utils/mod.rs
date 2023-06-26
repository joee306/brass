use actix_web::HttpRequest;

#[allow(dead_code)]
pub fn pip(req: HttpRequest) {
    println!("{}", {
        if let Some(v) = req.peer_addr() {
            v.ip().to_string()
        } else {
            "No IP ???".to_string()
        }
    });
}
