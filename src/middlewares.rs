use actix_web::{dev::ServiceRequest, error::ErrorUnauthorized, Error as ActixError};
use actix_web_httpauth::extractors::basic::BasicAuth;

pub async fn auth_middleware(req: ServiceRequest, creds: BasicAuth) -> Result<ServiceRequest, (ActixError, ServiceRequest)> {
    if creds.user_id() == "admin" && creds.password() == Some("admin") {
	Ok(req)
    } else {
	Err((ErrorUnauthorized("Invalid Credentials"), req))
    }
}
