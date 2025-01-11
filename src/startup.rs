use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpServer};

use crate::routes::{healthcheck, subscription};

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/healthcheck", web::get().to(healthcheck))
            .route("/subscription", web::post().to(subscription))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
