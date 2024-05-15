use clap::Parser;
use std::collections::HashMap;
use std::sync::Mutex;
use actix_web::{web, App, get, post, delete, HttpServer, HttpResponse};
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

#[derive(Debug)]
struct AppState {
    store: Mutex<HashMap<String, String>>
}

#[get("/{key:.*}")]
async fn get(path: web::Path<String>, data: web::Data<AppState>) -> HttpResponse {
    let key: String = path.into_inner();

    info!("Fetching {key}");

    match data.store.lock().unwrap().get(&key) {
	Some(value) => HttpResponse::Ok()
			.content_type(ContentType::plaintext())
			.body(value.to_string()),
	_ => HttpResponse::NotFound()
		.content_type(ContentType::plaintext())
		.body("key not found\n")
    }

}

#[post("/{key:.*}")]
async fn set(path: web::Path<String>, post: web::Bytes, data: web::Data<AppState>) -> HttpResponse {
    let key: String = path.into_inner();
    let value: String = String::from_utf8(post.to_vec()).unwrap();

    info!("Setting {key} as {value}");

    match data.store.lock().unwrap().insert(key, value) {
	None => HttpResponse::Created()
	    .content_type(ContentType::plaintext())
	    .body("new value inserted\n"),
	Some(_) => HttpResponse::Accepted()
	    .content_type(ContentType::plaintext())
	    .body("value updated\n"),
    }
}

#[delete("/{key:.*}")]
async fn del(path: web::Path<String>, data: web::Data<AppState>) -> HttpResponse {
    let key: String = path.into_inner();

    match data.store.lock().unwrap().remove(&key) {
	Some(_) => HttpResponse::Ok()
	    .content_type(ContentType::plaintext())
	    .body("key and value removed\n"),
	_ => HttpResponse::NotFound()
	    .content_type(ContentType::plaintext())
	    .body("key not found\n"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let _ = HttpServer::new(move || {
	let state = web::Data::new(AppState {
	    store: Mutex::new(HashMap::new())
	});

        App::new()
            .app_data(state)
            .service(get)
            .service(set)
            .service(del)
    })
    .bind((args.host, args.port))?
    .run()
    .await;

    Ok(())
}
