use actix_web::{get, web, App, HttpServer, Responder};

#[get("/{name}")]
async fn hello(web::Path(name): web::Path<String>) -> impl Responder {
    format!("Hello, {}!", name)
}

#[get("/")]
async fn index() -> impl Responder {
    "Hello, world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(hello))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
