use actix_web::{dev::ServiceRequest, error::ErrorUnauthorized, Error as ActixError};
use actix_web_httpauth::extractors::basic::BasicAuth;
use log::warn;
use std::env;

pub async fn auth_middleware(
    req: ServiceRequest,
    creds: BasicAuth,
) -> Result<ServiceRequest, (ActixError, ServiceRequest)> {
    // Fetch the valid credentials from environment variables
    let valid_user_id = env::var("MINIKV_USERNAME").unwrap_or_else(|_| "admin".to_string());
    let valid_password = env::var("MINIKV_PASSWORD").unwrap_or_else(|_| "admin".to_string());

    if creds.user_id() == valid_user_id && creds.password() == Some(valid_password.as_str()) {
        Ok(req)
    } else {
        warn!(
            "Unauthorized access attempt with user ID: {}",
            creds.user_id()
        );
        Err((ErrorUnauthorized("Invalid Credentials"), req))
    }
}
