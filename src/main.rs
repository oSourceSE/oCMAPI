// Load crates
use actix_web::{App, HttpServer, guard, web };
use actix_web::middleware::NormalizePath;
use actix_session::{ SessionMiddleware };
use actix_session::config::{ BrowserSession, CookieContentSecurity };
use actix_session::storage::{ CookieSessionStore };
use actix_web_httpauth::middleware::HttpAuthentication;
use actix_web::cookie::{ Key, SameSite };
use std::io::BufReader;
use std::fs::{File};
use argh::FromArgs;
// Load local modules
mod config;
mod models;
mod handlers;
mod auth;
mod logs;

/// API for managing pods and containers.
#[derive(FromArgs)]
struct Args {
    /// path to configuration file including file name, if left out it will check in the same directory as the binary
    /// for a file named ocmapi.toml.
    #[argh(option, long = "config")]
    config: Option<String>,
    /// show version for oCMAPI.
    #[argh(switch, short = 'v')]
    version: bool,
}

// Function to show version.
fn show_version() {
    // Get version from cargo.toml during build time.
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    // Show version if -v and then exit.
    let args: Args = argh::from_env();
    if args.version {
        println!("oCMAPI v{}", VERSION);
        std::process::exit(0)
    }
}

// Start information
fn start_info() {
    println!("oCMAPI - Container Management API");
    println!("Listening on {}",config::SERVER_ADDRESS.get().unwrap());
    println!("Running server...");
}

// Session Middleware
fn session_middleware() -> SessionMiddleware<CookieSessionStore> {
    SessionMiddleware::builder(
        CookieSessionStore::default(), Key::from(&[0; 64])
    )
    // Get cookie name from config.
    .cookie_name(String::from(config::COOKIE_NAME.get().unwrap()))
    // https only
    .cookie_secure(true)
    // expire at end of session
	.session_lifecycle(BrowserSession::default()) 
	.cookie_same_site(SameSite::Strict) 
    // encrypt
	.cookie_content_security(CookieContentSecurity::Private)
    // disallow scripts from reading
    .cookie_http_only(true)
	.build()
}

// HTTP Server
#[actix_web::main]
async fn http_server() -> std::io::Result<()> {
    // Run the server.
    HttpServer::new(|| {
        App::new()
            .wrap(HttpAuthentication::basic(auth::validator))
            .wrap(session_middleware())
            .wrap(NormalizePath::default())
            // Redirect certain paths to root path.
            .service(web::redirect("/v1/common", "/"))
            .service(web::redirect("/v1/containers", "/"))
            .service(web::redirect("/v1/pods", "/"))
            .service(web::redirect("/v1/networks", "/"))
            .service(
                web::scope("")
                    // API info web.
                    .route("/", web::get().guard(guard::Get()).to(handlers::api_info_web))
                    // Common API
                    .route("/v1/common/getStats", web::get().guard(guard::Get()).to(handlers::get_common_stats))
                    .route("/v1/common/getVersion", web::get().guard(guard::Get()).to(handlers::get_common_version))
                    .route("/v1/common/getInfo", web::get().guard(guard::Get()).to(handlers::get_common_info))
                    .route("/v1/common/postCreateEnvFile", web::post().guard(guard::Post()).to(handlers::post_common_envfile))
                    // Containers API
                    .route("/v1/containers/getStatus", web::get().guard(guard::Get()).to(handlers::get_containers_status))
                    .route("/v1/containers/getStatus/{id}", web::get().guard(guard::Get()).to(handlers::get_containers_status_query))
                    .route("/v1/containers/postContainerCreate", web::post().guard(guard::Post()).to(handlers::post_containers_create))
                    .route("/v1/containers/postSetState", web::post().guard(guard::Post()).to(handlers::post_containers_setstate))
                    // Pods API
                    .route("/v1/pods/getStatus", web::get().guard(guard::Get()).to(handlers::get_pods_status))
                    .route("/v1/pods/getStatus/{id}", web::get().guard(guard::Get()).to(handlers::get_pods_status_query))
                    .route("/v1/pods/postPodCreate", web::post().guard(guard::Post()).to(handlers::post_pods_create))
                    .route("/v1/pods/postSetState", web::post().guard(guard::Post()).to(handlers::post_pods_setstate))
                    // Networks API
                    .route("/v1/networks/getInfo", web::get().guard(guard::Get()).to(handlers::get_networks_info))
                    .route("/v1/networks/getInfo/{id}", web::get().guard(guard::Get()).to(handlers::get_networks_info_single))
                    .route("/v1/networks/postCreateNetwork", web::post().guard(guard::Post()).to(handlers::post_networks_create))
            )
        })
        .bind(config::SERVER_ADDRESS.get().unwrap())?
        .run()
        .await
}

// HTTPS Server
#[actix_web::main]
async fn https_server() -> std::io::Result<()> {

    // Load crypto provider.
    rustls::crypto::aws_lc_rs::default_provider().install_default().unwrap();

    // Load certificate information via config file.
    let mut certs_file = BufReader::new(File::open(config::CERT_PATH.get().unwrap().trim()).unwrap());
    let mut key_file = BufReader::new(File::open(config::CERT_PRIVKEY.get().unwrap().trim()).unwrap());

    // load TLS certs and key
    let tls_certs = rustls_pemfile::certs(&mut certs_file).collect::<Result<Vec<_>, _>>().unwrap();
    let tls_key = rustls_pemfile::pkcs8_private_keys(&mut key_file).next().unwrap().unwrap();

    // Build ServerConfig with certificate data.
    let tls_config = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(tls_certs,rustls::pki_types::PrivateKeyDer::Pkcs8(tls_key))
        .unwrap();
    
    // Run the server.
    HttpServer::new(|| {
        App::new()
            .wrap(HttpAuthentication::basic(auth::validator))
            .wrap(session_middleware())
            .wrap(NormalizePath::default())
            // Redirect certain paths to root path.
            .service(web::redirect("/v1/common", "/"))
            .service(web::redirect("/v1/containers", "/"))
            .service(web::redirect("/v1/pods", "/"))
            .service(web::redirect("/v1/networks", "/"))
            .service(
                web::scope("")
                    // API info web.
                    .route("/", web::get().guard(guard::Get()).to(handlers::api_info_web))
                    // Common API
                    .route("/v1/common/getStats", web::get().guard(guard::Get()).to(handlers::get_common_stats))
                    .route("/v1/common/getVersion", web::get().guard(guard::Get()).to(handlers::get_common_version))
                    .route("/v1/common/getInfo", web::get().guard(guard::Get()).to(handlers::get_common_info))
                    .route("/v1/common/postCreateEnvFile", web::post().guard(guard::Post()).to(handlers::post_common_envfile))
                    // Containers API
                    .route("/v1/containers/getStatus", web::get().guard(guard::Get()).to(handlers::get_containers_status))
                    .route("/v1/containers/getStatus/{id}", web::get().guard(guard::Get()).to(handlers::get_containers_status_query))
                    .route("/v1/containers/postContainerCreate", web::post().guard(guard::Post()).to(handlers::post_containers_create))
                    .route("/v1/containers/postSetState", web::post().guard(guard::Post()).to(handlers::post_containers_setstate))
                    // Pods API
                    .route("/v1/pods/getStatus", web::get().guard(guard::Get()).to(handlers::get_pods_status))
                    .route("/v1/pods/getStatus/{id}", web::get().guard(guard::Get()).to(handlers::get_pods_status_query))
                    .route("/v1/pods/postPodCreate", web::post().guard(guard::Post()).to(handlers::post_pods_create))
                    .route("/v1/pods/postSetState", web::post().guard(guard::Post()).to(handlers::post_pods_setstate))
                    // Networks API
                    .route("/v1/networks/getInfo", web::get().guard(guard::Get()).to(handlers::get_networks_info))
                    .route("/v1/networks/getInfo/{id}", web::get().guard(guard::Get()).to(handlers::get_networks_info_single))
                    .route("/v1/networks/postCreateNetwork", web::post().guard(guard::Post()).to(handlers::post_networks_create))
            )
        })
        .bind_rustls_0_23(config::SERVER_ADDRESS.get().unwrap(), tls_config)?
        .run()
        .await
}

// API Main listener function.
fn main() {
    // Show version if -v
    show_version();

    // Check if path to config is given else assume it is in same directory.
    let args: Args = argh::from_env();
    let cfg_file = args.config.unwrap_or("./ocmapi.toml".to_string());
    config::configuration(cfg_file);

    // Show start info
    start_info();

    // Check if SSL is set in config and run correct server.
    let check_ssl: String = config::USE_SSL.get().unwrap().to_string();
    if check_ssl == "yes" {
        // Send server start information to log.
        let logdata = format!("CEF:1|oCMAPI|API|1.0|100|Server Started|0|startTime=<starttime> src=127.0.0.1 dpt={} proto=TCP app=HTTPS msg=Server started and is listening for incoming traffic",config::SERVER_PORT.get().unwrap());
        let _ = logs::send_logs(logdata);
        // Run HTTPS Server.
        let _ = https_server();
        // Send server stop information to log.
        let logdata = format!("CEF:1|oCMAPI|API|1.0|100|Server Stopped|0|startTime=<starttime> src=127.0.0.1 dpt={} proto=TCP app=HTTPS msg=Server stopped and is no longer listening for incoming traffic",config::SERVER_PORT.get().unwrap());
        let _ = logs::send_logs(logdata);
    }
    else {
        // Send server start information to log.
        let logdata = format!("CEF:1|oCMAPI|API|1.0|100|Server Started|0|startTime=<starttime> src=127.0.0.1 dpt={} proto=TCP app=HTTP msg=Server started and is listening for incoming traffic",config::SERVER_PORT.get().unwrap());
        let _ = logs::send_logs(logdata);
         // Run HTTP Server.
        let _ = http_server();
        // Send server stop information to log.
        let logdata = format!("CEF:1|oCMAPI|API|1.0|100|Server Stopped|0|startTime=<starttime> src=127.0.0.1 dpt={} proto=TCP app=HTTP msg=Server stopped and is no longer listening for incoming traffic",config::SERVER_PORT.get().unwrap());
        let _ = logs::send_logs(logdata);
    }     
}