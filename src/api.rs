use actix_web::http::header::ContentType;
use actix_web::{delete, get, post, web, HttpResponse};
use log::{error, info};

use crate::app_state::AppState;

// Keys API

#[get("/keys/{key:.*}")]
pub async fn get(path: web::Path<String>, data: web::Data<AppState>) -> HttpResponse {
    let key: String = path.into_inner();
    info!("Fetching key: {}", key);

    let store = data.store.lock().unwrap();
    match store.get(&key) {
        Some(value) => {
            info!("Key found: {} -> {}", key, value);
            HttpResponse::Ok()
                .content_type(ContentType::plaintext())
                .body(value.to_string())
        }
        None => {
            info!("Key not found: {}", key);
            HttpResponse::NotFound()
                .content_type(ContentType::plaintext())
                .body("key not found\n")
        }
    }
}

#[post("/keys/{key:.*}")]
pub async fn set(
    path: web::Path<String>,
    post: web::Bytes,
    data: web::Data<AppState>,
) -> HttpResponse {
    let key: String = path.into_inner();
    let value = match String::from_utf8(post.to_vec()) {
        Ok(v) => v,
        Err(e) => {
            error!("Invalid UTF-8 sequence: {}", e);
            return HttpResponse::BadRequest()
                .content_type(ContentType::plaintext())
                .body("invalid UTF-8 sequence\n");
        }
    };

    info!("Setting key: {} with value: {}", key, value);

    let mut store = data.store.lock().unwrap();
    match store.insert(key.clone(), value.clone()) {
        None => {
            info!("New key inserted: {} -> {}", key, value);
            HttpResponse::Created()
                .content_type(ContentType::plaintext())
                .body("new value inserted\n")
        }
        Some(_) => {
            info!("Key updated: {} -> {}", key, value);
            HttpResponse::Accepted()
                .content_type(ContentType::plaintext())
                .body("value updated\n")
        }
    }
}

#[delete("/keys/{key:.*}")]
pub async fn del(path: web::Path<String>, data: web::Data<AppState>) -> HttpResponse {
    let key: String = path.into_inner();
    info!("Deleting key: {}", key);

    let mut store = data.store.lock().unwrap();
    match store.remove(&key) {
        Some(_) => {
            info!("Key removed: {}", key);
            HttpResponse::Ok()
                .content_type(ContentType::plaintext())
                .body("key and value removed\n")
        }
        None => {
            info!("Key not found for deletion: {}", key);
            HttpResponse::NotFound()
                .content_type(ContentType::plaintext())
                .body("key not found\n")
        }
    }
}

// View All Keys
#[get("/all")]
pub async fn get_all(data: web::Data<AppState>) -> HttpResponse {
    info!("Dumping all keys to requester");

    let store = data.store.lock().unwrap();
    let mut res_str: String = String::new();

    for (key, val) in store.iter() {
        res_str.push_str(format!("{} => {}\n", key, val).as_str());
    }

    return HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body(res_str.to_string());
}
