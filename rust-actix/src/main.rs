use actix_web::{get, App, HttpServer};

const PORT: u8 = 8000;

#[get("/")]
async fn index() -> String {
    "Hello World!".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("0.0.0.0", PORT))?
        .run()
        .await
}
