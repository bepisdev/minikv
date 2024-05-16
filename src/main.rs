use clap::Parser;
use std::collections::HashMap;
use std::sync::Mutex;
use actix_web::{web, App, get, post, delete, HttpServer, HttpResponse};
use actix_web::http::header::ContentType;
use log::{info, error};

#[derive(Parser, Debug)]
#[command(author = "Josh Burns", version = "0.0.0", about = "Mini key-value store over HTTP", long_about = None)]
struct Args {
    #[arg(long, default_value_t = String::from("0.0.0.0"))]
    host: String,

    #[arg(long, short, default_value_t = 8899)]
    port: u16,
}

#[derive(Debug)]
struct AppState {
    store: Mutex<HashMap<String, String>>
}

#[get("/keys/{key:.*}")]
async fn get(path: web::Path<String>, data: web::Data<AppState>) -> HttpResponse {
    let key: String = path.into_inner();
    info!("Fetching key: {}", key);

    let store = data.store.lock().unwrap();
    match store.get(&key) {
        Some(value) => {
            info!("Key found: {} -> {}", key, value);
            HttpResponse::Ok()
                .content_type(ContentType::plaintext())
                .body(value.to_string())
        },
        None => {
            info!("Key not found: {}", key);
            HttpResponse::NotFound()
                .content_type(ContentType::plaintext())
                .body("key not found\n")
        },
    }
}

#[post("/keys/{key:.*}")]
async fn set(path: web::Path<String>, post: web::Bytes, data: web::Data<AppState>) -> HttpResponse {
    let key: String = path.into_inner();
    let value = match String::from_utf8(post.to_vec()) {
        Ok(v) => v,
        Err(e) => {
            error!("Invalid UTF-8 sequence: {}", e);
            return HttpResponse::BadRequest()
                .content_type(ContentType::plaintext())
                .body("invalid UTF-8 sequence\n");
        },
    };

    info!("Setting key: {} with value: {}", key, value);

    let mut store = data.store.lock().unwrap();
    match store.insert(key.clone(), value.clone()) {
        None => {
            info!("New key inserted: {} -> {}", key, value);
            HttpResponse::Created()
                .content_type(ContentType::plaintext())
                .body("new value inserted\n")
        },
        Some(_) => {
            info!("Key updated: {} -> {}", key, value);
            HttpResponse::Accepted()
                .content_type(ContentType::plaintext())
                .body("value updated\n")
        },
    }
}

#[delete("/keys/{key:.*}")]
async fn del(path: web::Path<String>, data: web::Data<AppState>) -> HttpResponse {
    let key: String = path.into_inner();
    info!("Deleting key: {}", key);

    let mut store = data.store.lock().unwrap();
    match store.remove(&key) {
        Some(_) => {
            info!("Key removed: {}", key);
            HttpResponse::Ok()
                .content_type(ContentType::plaintext())
                .body("key and value removed\n")
        },
        None => {
            info!("Key not found for deletion: {}", key);
            HttpResponse::NotFound()
                .content_type(ContentType::plaintext())
                .body("key not found\n")
        },
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let state = web::Data::new(AppState {
        store: Mutex::new(HashMap::new())
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(get)
            .service(set)
            .service(del)
    })
    .bind((args.host.as_str(), args.port))?
    .run()
    .await
}
