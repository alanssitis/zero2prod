use crate::routes;
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::web;
use actix_web::{App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    // Wrap connection in a smart pointer
    let connection = web::Data::new(db_pool);
    // Capture 'connection' from surrounding environment
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(routes::health_check))
            .route("/subscriptions", web::post().to(routes::subscribe))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
