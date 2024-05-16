use clap::Parser;
use actix_web::{App, HttpServer};
use log::info;

mod app_state;
mod api;

use app_state::AppState;
use api::{get, set, del, get_all};

#[derive(Parser, Debug)]
#[command(author = "Josh Burns", version = "0.0.0", about = "Mini key-value store over HTTP", long_about = None)]
struct Args {
    #[arg(long, default_value_t = String::from("0.0.0.0"))]
    host: String,

    #[arg(long, short, default_value_t = 8899)]
    port: u16,
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!("Starting MiniKV server on {}:{}", args.host.as_str(), args.port);
    let state = AppState::new();

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(get)
            .service(set)
            .service(del)
            .service(get_all)
    })
    .bind((args.host.as_str(), args.port))?
    .run()
    .await
}
