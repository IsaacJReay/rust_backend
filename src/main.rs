use actix_web::http::{StatusCode};
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};


#[macro_use]
extern crate dotenv_codegen;


#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../sites/404.html"))
}


async fn privateapi() -> impl Responder {
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../sites/GraphiQL.html"))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let server = HttpServer::new(|| {
        App::new()
            .service(root)
            .route("/private/api", web::get().to(privateapi))
    })
    .bind(&dotenv!("IPADDR"))?
    .run();
    println!("Server running at http://{}",dotenv!("IPADDR"));
    server.await
}
