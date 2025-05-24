use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/payment")]
async fn welcome() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the cart service")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Cart service running on http://0.0.0.0:8080");

    HttpServer::new(|| App::new().service(welcome))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
