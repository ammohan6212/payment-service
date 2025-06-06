use actix_web::{get, App, HttpResponse, HttpServer, Responder};

// Define GET endpoint /payment/message
#[get("/payment/")]
async fn payment_message() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the payment service")
}

// Main function to start the server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Starting Payment Service at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .service(payment_message) // register the service
    })
    .bind(("127.0.0.1", 8081))? // bind to localhost:8080
    .run()
    .await
}
