use actix_web::{App, HttpResponse, HttpServer};
use actix_web::{get, web};
use askama::Template;
use askama_actix::TemplateToResponse;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate {
    name: String,
}


#[get("/hello/{name}")]
async fn hello(name: web::Path<String>) -> HttpResponse{
    let hello = HelloTemplate {
        name: name.into_inner(),
    };
    hello.to_response()
}