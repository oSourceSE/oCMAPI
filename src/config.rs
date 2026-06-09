// Load crates
use std::{process::exit, sync::OnceLock};

// Define global variables
pub static SERVER_ADDRESS: OnceLock<String> = OnceLock::new();
pub static SERVER_IP: OnceLock<String> = OnceLock::new();
pub static SERVER_PORT: OnceLock<String> = OnceLock::new();
pub static CERT_PATH: OnceLock<String> = OnceLock::new();
pub static CERT_PRIVKEY: OnceLock<String> = OnceLock::new();
pub static USE_SSL: OnceLock<String> = OnceLock::new();
pub static COOKIE_NAME: OnceLock<String> = OnceLock::new();
pub static CRED_SALT: OnceLock<String> = OnceLock::new();
pub static CRED_USER: OnceLock<String> = OnceLock::new();
pub static CRED_PASSWD: OnceLock<String> = OnceLock::new();
pub static LOG_PATH: OnceLock<String> = OnceLock::new();
pub static ENV_PATH: OnceLock<String> = OnceLock::new();
pub static SEC_PATH: OnceLock<String> = OnceLock::new();
pub static COMMON_KEY: OnceLock<String> = OnceLock::new();
pub static CONTAINERS_KEY: OnceLock<String> = OnceLock::new();
pub static PODS_KEY: OnceLock<String> = OnceLock::new();
pub static NETWORKS_KEY: OnceLock<String> = OnceLock::new();

// Read configuration from file.
pub fn configuration(cfg_file: String) {
    // Required crates for reading from .toml
    use config_file::FromConfigFile;
    use serde::Deserialize;

    // Get configuration keys
    #[derive(Deserialize)]
    struct Config {
        server: Server, // Match 'server' key in config file.
        certificate: Certificate, // Match 'certificate' key in config file.
        session: Session, // Match 'session' key in config file.
        auth: Auth, // Match 'auth' key in config file.
        logs: Logs, // Match 'logs' key in config file.
        env: Env, // Match 'env' key in config file.
        secrets: Secrets, // Match 'secrets' key in config file.
        keys: Keys, // Match 'keys' key in config file.
    }

    // Get parameters from [Server] key.
    #[derive(Deserialize)]
    struct Server {
        listen: String,
        port: String,
        ssl: String,
    }

    // Get parameters from [Certificate] key.
    #[derive(Deserialize)]
    struct Certificate {
        certfile: String,
        keyfile: String,
    }

    // Get parameters from [session] key.
    #[derive(Deserialize)]
    struct Session {
        cookiename: String,
    }

    // Get parameters from [auth] key.
    #[derive(Deserialize)]
    struct Auth {
        salt: String,
        username: String,
        password: String,
    }

    // Get parameters from [logs] key.
    #[derive(Deserialize)]
    struct Logs {
        logpath: String,
    }

    // Get parameters from [env] key.
    #[derive(Deserialize)]
    struct Env {
        envpath: String,
    }

    // Get parameters from [secrets] key.
    #[derive(Deserialize)]
    struct Secrets {
        secpath: String,
    }

    // Get parameters deom [keys] key.
    #[derive(Deserialize)]
    struct Keys {
        common_security_key: String,
        containers_security_key: String,
        pods_security_key: String,
        networks_security_key: String,
    }

    // Check if load of config file is Ok and act accordingly.
    let config:Config = if Config::from_config_file(cfg_file.clone()).is_ok() {
        // If loaded Ok fetch the file.
        Config::from_config_file(cfg_file).unwrap()
    }
    else {
        // If it is not Ok, error out and exit.
        println!("Cannot load configuration file, see 'ocmapi --help' for more information.");
        exit(1)
    };

    // Check all required parameters, maybe not the best way...
    let mut cfg_ok: bool = true;
    if config.server.listen.trim().is_empty() { cfg_ok = false; }
    if config.server.port.trim().is_empty() { cfg_ok = false; }
    if config.server.ssl.trim().is_empty() { cfg_ok = false; }
    if config.auth.username.trim().is_empty() { cfg_ok = false; }
    if config.auth.password.trim().is_empty() { cfg_ok = false; }
    if config.auth.salt.trim().is_empty() { cfg_ok = false; }
    if config.session.cookiename.trim().is_empty() { cfg_ok = false; }
    if config.env.envpath.trim().is_empty() { cfg_ok = false; }
    if config.secrets.secpath.trim().is_empty() { cfg_ok = false; }
    if config.keys.common_security_key.trim().is_empty() { cfg_ok = false; }
    if config.keys.containers_security_key.trim().is_empty() { cfg_ok = false; }
    if config.keys.pods_security_key.trim().is_empty() { cfg_ok = false; }
    if config.keys.networks_security_key.trim().is_empty() { cfg_ok = false; }
    // Extra check for SSL
    if config.server.ssl.trim() == "yes" {
        if config.certificate.certfile.trim().is_empty() { cfg_ok = false; }
        if config.certificate.keyfile.trim().is_empty() { cfg_ok = false; }
    }

    // Chack value of cfg_ok.
    if !cfg_ok {
        // Print information to console.
        println!("Some required parameters have no value,\ncheck your configuration file...");
        // Exit
        std::process::exit(1)
    }
    else {
        // Get address and port.
        let connect_string: String = format!("{}:{}",config.server.listen.trim(), config.server.port.trim());
        // Get IP separately for specific use.
        let ip_string: String = config.server.listen.trim().to_string();
        // Get port separately for specific use.
        let port_string: String = config.server.port.trim().to_string();
        // Get SSL settings.
        let ssl_string: String = config.server.ssl.trim().to_string();
        let path_string: String = config.certificate.certfile.trim().to_string();
        let privkey_string: String = config.certificate.keyfile.trim().to_string();
        // Get session settings.
        let cookie_name: String = config.session.cookiename.trim().to_string();
        // Get credentials.
        let cred_salt: String = config.auth.salt.trim().to_string();
        let cred_user: String = config.auth.username.trim().to_string();
        let cred_passwd: String = config.auth.password.trim().to_string();
        // Get log settings.
        let log_path: String = config.logs.logpath.trim().to_string();
        // Get env settings.
        let env_path: String = config.env.envpath.trim().to_string();
        // Get secrets settings.
        let sec_path: String = config.secrets.secpath.trim().to_string();
        // Get keys settings.
        let common_key: String = config.keys.common_security_key.trim().to_string();
        let containers_key: String = config.keys.containers_security_key.trim().to_string();
        let pods_key: String = config.keys.pods_security_key.trim().to_string();
        let networks_key: String = config.keys.networks_security_key.trim().to_string();
        // Spawn a thread and write to `OnceLock`.
        std::thread::spawn(|| {
            let _value = SERVER_ADDRESS.get_or_init(|| connect_string);
            let _value = SERVER_IP.get_or_init(|| ip_string);
            let _value = SERVER_PORT.get_or_init(|| port_string);
            let _value = USE_SSL.get_or_init(|| ssl_string);
            let _value = CERT_PATH.get_or_init(|| path_string);
            let _value = CERT_PRIVKEY.get_or_init(|| privkey_string);
            let _value = COOKIE_NAME.get_or_init(|| cookie_name);
            let _value = CRED_SALT.get_or_init(|| cred_salt);
            let _value = CRED_USER.get_or_init(|| cred_user);
            let _value = CRED_PASSWD.get_or_init(|| cred_passwd);
            let _value = LOG_PATH.get_or_init(|| log_path);
            let _value = ENV_PATH.get_or_init(|| env_path);
            let _value = SEC_PATH.get_or_init(|| sec_path);
            let _value = COMMON_KEY.get_or_init(|| common_key);
            let _value = CONTAINERS_KEY.get_or_init(|| containers_key);
            let _value = PODS_KEY.get_or_init(|| pods_key);
            let _value = NETWORKS_KEY.get_or_init(|| networks_key);
        })
        .join()
        .unwrap();
    }
}