mod qr;
mod token;

use actix_web::{post, web::Json, App, HttpResponse, HttpServer};
use serde::Deserialize;

#[derive(Deserialize)]
struct Token {
    user: String,
    expiry: u64,
}

#[post("/")]
async fn issue_token(form: Json<Token>) -> actix_web::Result<HttpResponse> {
    let token = token::new(&form.user, &form.expiry);

    let png = qr::create_qr_png(&token);

    Ok(HttpResponse::Ok().body(png))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(issue_token))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
