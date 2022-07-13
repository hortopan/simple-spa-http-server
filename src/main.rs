#[macro_use]
extern crate lazy_static;
extern crate serde_json;

use actix_files as fs;
use actix_web::{guard, http, middleware, web, App, HttpRequest, HttpResponse, HttpServer, Result};

mod config;
use config::CONFIG;

lazy_static! {
    #[derive(Debug)]
    pub static ref HTML: String = {
        std::fs::read_to_string(&format!("{}/index.html", CONFIG.serve)).unwrap()
    };
}

async fn index_serve(_req: HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .set_header(http::header::CONTENT_TYPE, "text/html")
        .set_header(http::header::CACHE_CONTROL, &*CONFIG.cache_control_index)
        .body(HTML.as_str()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!(
        "Starting server and listing at {} , serving from {} , with Cache-Control: {}",
        CONFIG.bind, CONFIG.serve, CONFIG.cache_control
    );

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::DefaultHeaders::new().header("Cache-Control", &CONFIG.cache_control))
            .wrap(middleware::Compress::default())
            .route("/", web::get().to(|req| index_serve(req)))
            .service(fs::Files::new("", &CONFIG.serve))
            .default_service(
                web::resource("")
                    .route(web::get().to(|req| index_serve(req)))
                    .route(
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
