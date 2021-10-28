#[macro_use]
extern crate lazy_static;
extern crate serde_json;

use actix_files as fs;
use actix_web::{guard, middleware, web, App, HttpResponse, HttpServer};

mod config;
use config::CONFIG;

async fn index() -> Result<actix_files::NamedFile, std::io::Error> {
    Ok(actix_files::NamedFile::open("./static/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::DefaultHeaders::new().header("Cache-control", &CONFIG.cache_control))
            .wrap(middleware::Compress::default())
            .route("/", web::get().to(index))
            .service(fs::Files::new("", &CONFIG.serve))
            .default_service(
                web::resource("").route(web::get().to(index)).route(
                    web::route()
                        .guard(guard::Not(guard::Get()))
                        .to(|| HttpResponse::MethodNotAllowed()),
                ),
            )
    })
    .bind(&CONFIG.bind)?
    .run()
    .await
}
