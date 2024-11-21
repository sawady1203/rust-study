use actix_web::{App, HttpServer};
use actix_web::get;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[get("/hello")]
async fn hello() -> String{
    "Hello, world".to_string()
}