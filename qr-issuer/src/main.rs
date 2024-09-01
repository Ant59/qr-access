mod qr;
mod token;

use std::time::Duration;

use actix_web::{get, App, HttpResponse, HttpServer};

#[get("/")]
async fn issue_token() -> actix_web::Result<HttpResponse> {
    let token = token::new("user", Duration::from_secs(60));

    let image_html = qr::create_qr_image_html(&token);
    let escaped_token = token.replace("<", "&lt;").replace(">", "&gt;");
    let html = format!("<p><pre>{}</pre></p>{}", escaped_token, image_html);

    Ok(HttpResponse::Ok().body(html))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(issue_token))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
