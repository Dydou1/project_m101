mod db;
mod graph;
mod macros;
mod mqtt;

use std::io;

use actix_web::{App, HttpServer};
use env_logger::Env;
use tokio::task;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().expect("env file should exist");

    let env = Env::new().filter_or("LOG_LEVEL", "info");
    env_logger::builder()
        .parse_env(env)
        .format_timestamp(None)
        .init();

    task::spawn(mqtt::connect());

    HttpServer::new(App::new)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
