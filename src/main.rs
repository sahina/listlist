use actix_web::{App, HttpServer, web};
use slog::{Drain, info, Logger, o};
use slog_term;
use tokio_postgres::NoTls;

use dotenv::dotenv;

use crate::handlers::*;

mod config;
mod db;
mod errors;
mod handlers;
mod models;

fn config_log() -> Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let console_drain = slog_term::FullFormat::new(decorator).build().fuse();
    let console_drain = slog_async::Async::new(console_drain).build().fuse();

    slog::Logger::root(console_drain, o!("v" => env!("CARGO_PKG_VERSION")))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // load env vars
    dotenv().ok();

    let config = config::Config::from_env().unwrap();
    let pool = config.pg.create_pool(NoTls).unwrap();
    let log = config_log();

    println!("Started at {}:{}", config.server.host, config.server.port);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/", web::get().to(index))
            .route("/lists{_:/?}", web::post().to(create_list))
            .route("/lists{_:/?}", web::get().to(get_lists))
            .route(
                "/lists/{list_id}/items{_:/?}",
                web::get().to(get_list_items),
            )
    })
        .bind(format!("{}:{}", config.server.host, config.server.port))?
        .run()
        .await
}
