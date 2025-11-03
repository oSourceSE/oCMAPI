// Load crates
use actix_web::{Error, dev::ServiceRequest, http::header::USER_AGENT};
use actix_web_httpauth::{extractors::AuthenticationError};
use actix_web_httpauth::{extractors::basic::BasicAuth, headers::www_authenticate};
// Load my own config crate
use crate::config;
use crate::logs;

// Authentication validator.
pub async fn validator(
    req: ServiceRequest,
    credentials: BasicAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    if secure_compare(
        credentials.user_id(),
        credentials.password().unwrap_or_default(),
    ) {

        // Get USER-AGENT from request header, ugly but works.
        let mut ua_string = String::new();
        for v in req.headers().get_all(USER_AGENT) {
            ua_string = format!("{:?}",v);
        };
        // Vec for ServiceRequest data to log.
        // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
        let vlogdata = vec![
            req.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
            req.connection_info().scheme().to_string(),
            req.path().to_string(),
            req.connection_info().host().to_string(),
            ua_string
        ];
        
        // Get log function and put requierd data into it.
        let vlog: Vec<String> = logs::log_data(200,"Login Succesfully",0,"POST",vlogdata);
        // Send information to log.
        let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
        let _ = logs::send_logs(logdata);

        // Login successful.
        Ok(req)
    } else {

        // Get USER-AGENT from request header, ugly but works.
        let mut ua_string = String::new();
        for v in req.headers().get_all(USER_AGENT) {
            ua_string = format!("{:?}",v);
        };
        // Vec for ServiceRequest data to log.
        // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
        let vlogdata = vec![
            req.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
            req.connection_info().scheme().to_string(),
            req.path().to_string(),
            req.connection_info().host().to_string(),
            ua_string
        ];

        // Get log function and put requierd data into it.
        let vlog: Vec<String> = logs::log_data(401,"Unauthorized",10,"POST",vlogdata);
        // Send information to log.
        let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={} oCMAPIUserLogin={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9],credentials.user_id());
        let _ = logs::send_logs(logdata);

        let challenge = www_authenticate::basic::Basic::new();
        Err((AuthenticationError::new(challenge).into(), req))
    }
}

// Use constant time comparison to prevent timing attacks
use constant_time_eq::constant_time_eq;

// Statics created at runtime for authentication.
lazy_static::lazy_static! {
    static ref SALT: String = String::from(config::CRED_SALT.get().unwrap());
    static ref API_USER: String = String::from(config::CRED_USER.get().unwrap());
    static ref API_PASSWORD: [u8; 32] = hash_string(config::CRED_PASSWD.get().unwrap().as_str());
}

// Compare entered login with stored login.
fn secure_compare(username: &str, password: &str) -> bool {
    if username != API_USER.as_str() {
        return false;
    }
    if password.is_empty() {
        return false;
    }
    let input_password = hash_string(password);
    constant_time_eq(&input_password, &API_PASSWORD[..])
}

// Use salted hashes of a constant length so the constant_time_eq works optimally
use bcrypt_pbkdf::bcrypt_pbkdf;

fn hash_string(string: &str) -> [u8; 32] {
    let mut output = [0u8; 32];
    bcrypt_pbkdf(string.as_bytes(), SALT.as_bytes(), 10, &mut output)
        .expect("bcrypt_pbkdf failed on password");
    output
}