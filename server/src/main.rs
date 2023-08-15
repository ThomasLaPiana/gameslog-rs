use actix_web::middleware::Logger;
use actix_web::{delete, get, post, App, HttpResponse, HttpServer, Responder};
use env_logger;

#[get("/")]
async fn view() -> impl Responder {
    HttpResponse::Ok().body("View")
}

#[post("/game/{name}")]
async fn add(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[delete("/game/{name}")]
async fn delete(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(view)
            .service(add)
            .service(delete)
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}
