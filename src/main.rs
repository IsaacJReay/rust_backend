mod funct;

#[macro_use]
extern crate dotenv_codegen;

use actix_web::{web, guard, App, HttpServer};
use jsonrpc_v2::Server;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let loginrpc = Server::new()
        .with_method("message", funct::pam_login)
        .finish();

    let server = HttpServer::new(move || {
        let loginrpc = loginrpc.clone();
        App::new()
            .service(funct::root)
            .route("/private/api", web::get().to(funct::privateapi))
            .service(
                web::service("/private/api/login")
                    .guard(guard::Post())
                    .finish(loginrpc.into_web_service()),
            )
    })
    .bind(&dotenv!("IPADDR"))?
    .run();
    println!("Server running at http://{}",dotenv!("IPADDR"));
    server.await
}
