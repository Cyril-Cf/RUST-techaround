use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/test")]
async fn get_test() -> impl Responder {
    HttpResponse::Ok().body("Test OK 2")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_test))
        .bind("0.0.0.0:3000")?
        .run()
        .await
}