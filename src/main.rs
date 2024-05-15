use clap::Parser;
use std::collections::HashMap;
use std::sync::Mutex;
use actix_web::{web};

#[derive(Parser, Debug)]
#[command(author = "Josh Burns", version = "0.0.0", about = "Mini key-value store", long_about = None)]
struct Args {
    #[arg(long, default_value_t = String::from("0.0.0.0"))]
    host: String,

    #[arg(long, short, default_value_t = 8899)]
    port: u16,
}

struct AppState {
    store: Mutex<HashMap<String, String>>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let state = web::Data::new(AppState {
	store: Mutex::new(HashMap::new())
    });

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    Ok(())
}
