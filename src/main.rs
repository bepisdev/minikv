use clap::Parser;
use std::collections::HashMap;
use std::sync::Mutex;
use actix_web::{web, App, get, post, delete, HttpServer, HttpRequest, HttpResponse};
use actix_web::http::header::ContentType;
use log::info;

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

#[get("/{key:.*}")]
async fn get(request: HttpRequest, data: web::Data<AppState>) -> HttpResponse {
    let key: String = request.uri().to_string();

    match data.store.lock().unwrap().get(&key) {
	Some(value) => HttpResponse::Ok()
	    .content_type(ContentType::plaintext())
	    .body(value.to_string()),
	_ => HttpResponse::NotFound()
	    .content_type(ContentType::plaintext())
	    .body("key not found"),
    }
}

#[post("/{key:.*}")]
async fn set(request: HttpRequest, post: web::Bytes, data: web::Data<AppState>) -> HttpResponse {
    let key: String = request.uri().to_string();
    let value: String = String::from_utf8(post.to_vec()).unwrap();

    match data.store.lock().unwrap().insert(key, value) {
	None => HttpResponse::Created()
	    .content_type(ContentType::plaintext())
	    .body("new value inserted"),
	Some(_) => HttpResponse::Accepted()
	    .content_type(ContentType::plaintext())
	    .body("value updated"),
    }
}

#[delete("/{key:.*}")]
async fn del(request: HttpRequest, data: web::Data<AppState>) -> HttpResponse {
    let key: String = request.uri().to_string();

    match data.store.lock().unwrap().remove(&key) {
	Some(_) => HttpResponse::Ok()
	    .content_type(ContentType::plaintext())
	    .body("key and value removed"),
	_ => HttpResponse::NotFound()
	    .content_type(ContentType::plaintext())
	    .body("key not found"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let state = web::Data::new(AppState {
	store: Mutex::new(HashMap::new())
    });

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(get)
            .service(set)
            .service(del)
            .wrap(info!("%a %{User-Agent}i"))
    })
    .bind((args.host, args.port))?
    .run()
    .await;

    Ok(())
}
