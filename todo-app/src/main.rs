use actix_web::{App, HttpServer};
use actix_web::{get, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[get("/hello/{name}")]
async fn hello(name: web::Path<String>) -> String{
    format!("Hello, {name}!")
}