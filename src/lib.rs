use actix_web::dev::Server;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

#[derive(serde::Deserialize)]
struct FormData {
    name: String,
    email: String
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", name)
}

async fn health_check(_: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}

async fn subscribe(form: web::Form<FormData>) -> impl Responder {
    println!("{}", form.name);
    println!("{}", form.email);
    HttpResponse::Ok()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .route("/{name}", web::get().to(greet))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
