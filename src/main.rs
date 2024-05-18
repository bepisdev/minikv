use actix_web::{App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use clap::Parser;
use log::info;

mod api;
mod app_state;
mod middlewares;

use api::{del, get, get_all, set};
use app_state::AppState;
use middlewares::auth_middleware;

#[derive(Parser, Debug)]
#[command(author = "Josh Burns", version = env!("CARGO_PKG_VERSION"), about = "Mini key-value store over HTTP", long_about = None)]
struct Args {
    #[arg(long, default_value_t = String::from("0.0.0.0"))]
    host: String,

    #[arg(long, short, default_value_t = 8899)]
    port: u16,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    env_logger::init_from_env(env_logger::Env::new().filter_or("MINIKV_LOG_LEVEL", "warn"));

    info!(
        "Starting MiniKV server on {}:{}",
        args.host.as_str(),
        args.port
    );
    let state = AppState::new();

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(HttpAuthentication::basic(auth_middleware))
            .service(get)
            .service(set)
            .service(del)
            .service(get_all)
    })
    .bind((args.host.as_str(), args.port))?
    .run()
    .await
}
