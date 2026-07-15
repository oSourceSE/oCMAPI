// Load crates
use actix_web::{HttpRequest, HttpResponse, http::{StatusCode, header::{ContentType, USER_AGENT}}, web::{self}};
use std::{ fs::{self, File, remove_file}, path::Path, process::Command };
use serde_json::{ json };
use std::io::{ self, Write };
use include_dir::{include_dir, Dir};
use regex::Regex;
use rand::{RngExt};
// Load local modules
use crate::{config::{COMMON_KEY, CONTAINERS_KEY, PODS_KEY, NETWORKS_KEY, ENV_PATH, SEC_PATH}};
use crate::{models::{*}};
use crate::logs;

// Collection of common headers for API responses.
pub fn api_headers() -> Vec<String> {
    // Vec for headers.
    let vheaders: Vec<String> = vec![
        // API Version.
        "1".to_string(),
        // ContentType JSON, application/json.
        ContentType::json().to_string(),
        // ContentType HTML, text/html; charset=utf-8.
        ContentType::html().to_string(),
    ];
    // Return headers.
    vheaders
}

// Random number generator used where needed.
pub fn rand_number() -> String {
    let mut rng = rand::rng();
    let random_number: u32 = rng.random();
    random_number.to_string()
}

/* --- Main API Information Web --- */
pub async fn api_info_web(reqdata: HttpRequest) -> io::Result<HttpResponse> {
    // Include html file inside binary.
    static HTML_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/html");
    let api_html = HTML_DIR.get_file("api.html").unwrap();
    let html_body = api_html.contents_utf8().unwrap();

    // Get USER-AGENT from request header, ugly but works.
    let mut ua_string = String::new();
    for v in reqdata.headers().get_all(USER_AGENT) {
        ua_string = format!("{:?}",v);
    };

    // Vec for HttpRequest data to log.
    // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
    let vlogdata = vec![
        reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
        reqdata.connection_info().scheme().to_string(),
        reqdata.path().to_string(),
        reqdata.connection_info().host().to_string(),
        ua_string
    ];

    // Get log function and put requierd data into it.
    let vlog: Vec<String> = logs::log_data(200,"Request OK",0,"GET",vlogdata);
    // Send information to log.
    let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
    let _ = logs::send_logs(logdata);

    // Fetch headers.
    let vheaders = api_headers();

    // Send response.
    Ok( HttpResponse::build(StatusCode::OK)
        .append_header(("api-version",vheaders[0].clone()))
        .content_type(vheaders[2].clone())
        .body(html_body) )
}

/* --- Common API --- */

// Get podman stats.
pub async fn get_common_stats(reqdata: HttpRequest) -> io::Result<HttpResponse> {
    // The command.
    let cmd = Command::new("podman")
        .arg("stats")
        .arg("--no-stream")
        .arg("--format=json")
        .output();

    // Check if command went ok or not.
    match cmd {
        // When Ok build response.
        Ok(cmd_ok) => {
            // Check length of stdout, returns empty response when no containers are running.
            let count = cmd_ok.stdout.len();
            if count > 4 {
                // Clean data from unneeded characters.
                let data = format!("{}", String::from_utf8_lossy(&cmd_ok.stdout))
                    .trim_start()
                    .trim_end()
                    .to_string();

                 // Get USER-AGENT from request header, ugly but works.
                let mut ua_string = String::new();
                for v in reqdata.headers().get_all(USER_AGENT) {
                    ua_string = format!("{:?}",v);
                };

                // Vec for HttpRequest data to log.
                // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
                let vlogdata = vec![
                    reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                    reqdata.connection_info().scheme().to_string(),
                    reqdata.path().to_string(),
                    reqdata.connection_info().host().to_string(),
                    ua_string
                ];

                // Get log function and put requierd data into it.
                let vlog: Vec<String> = logs::log_data(200,"Request OK",0,"GET",vlogdata);
                // Send information to log.
                let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
                let _ = logs::send_logs(logdata);

                // Fetch headers.
                let vheaders = api_headers();

                // Return answer.
                Ok( HttpResponse::Ok()
                    .append_header(("api-version",vheaders[0].clone()))
                    .content_type(vheaders[1].clone())
                    .body(data) )
            }
            // Return error since no containers are running.
            else {
                // Get USER-AGENT from request header, ugly but works.
                let mut ua_string = String::new();
                for v in reqdata.headers().get_all(USER_AGENT) {
                    ua_string = format!("{:?}",v);
                };

                // Vec for HttpRequest data to log.
                // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
                let vlogdata = vec![
                    reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                    reqdata.connection_info().scheme().to_string(),
                    reqdata.path().to_string(),
                    reqdata.connection_info().host().to_string(),
                    ua_string
                ];

                // Get log function and put requierd data into it.
                let vlog: Vec<String> = logs::log_data(404,"Not Found",4,"GET",vlogdata);
                // Send information to log.
                let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
                let _ = logs::send_logs(logdata);

                let data = json!(
                    {
                        "Code": 404,
                        "Message": "No stats available, No containers running?",
                        "Error": "Empty Respons"
                    }
                );

                // Fetch headers.
                let vheaders = api_headers();

                // Return answer.
                Ok( HttpResponse::NotFound()
                    .append_header(("api-version",vheaders[0].clone()))
                    .content_type(vheaders[1].clone())
                    .json(data) )
            }
        },
        // When error build response.
        Err(cmd_err) => {
            // Construct JSON object
            let data = json!(
                {
                    "Code": 500,
                    "Info": format!("{}", cmd_err),
                    "Status": "Internal Server Error"
                }
            );

            // Get USER-AGENT from request header, ugly but works.
            let mut ua_string = String::new();
            for v in reqdata.headers().get_all(USER_AGENT) {
                ua_string = format!("{:?}",v);
            };

            // Vec for HttpRequest data to log.
            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
            let vlogdata = vec![
                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                reqdata.connection_info().scheme().to_string(),
                reqdata.path().to_string(),
                reqdata.connection_info().host().to_string(),
                ua_string
            ];

            // Get log function and put requierd data into it.
            let vlog: Vec<String> = logs::log_data(500,"Internal Server Error",6,"GET",vlogdata);
            // Send information to log.
            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
            let _ = logs::send_logs(logdata);

            // Fetch headers.
            let vheaders = api_headers();

            // Return answer.
            Ok( HttpResponse::InternalServerError()
                .append_header(("api-version",vheaders[0].clone()))
                .content_type(vheaders[1].clone())
                .json(data) )
        }
    }
}

// Get podman version
pub async fn get_common_version(reqdata: HttpRequest) -> io::Result<HttpResponse> {
    // The command.
    let cmd = Command::new("podman")
        .arg("version")
        .arg("--format=json")
        .output();

    // Check if command went ok or not.
    match cmd {
        // When Ok build response.
        Ok(cmd_ok) => {
            // Clean data from unneeded characters.
            let data = format!("{}", String::from_utf8_lossy(&cmd_ok.stdout));

            // Get USER-AGENT from request header, ugly but works.
            let mut ua_string = String::new();
            for v in reqdata.headers().get_all(USER_AGENT) {
                ua_string = format!("{:?}",v);
            };

            // Vec for HttpRequest data to log.
            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
            let vlogdata = vec![
                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                reqdata.connection_info().scheme().to_string(),
                reqdata.path().to_string(),
                reqdata.connection_info().host().to_string(),
                ua_string
            ];

            // Get log function and put requierd data into it.
            let vlog: Vec<String> = logs::log_data(200,"Request OK",0,"GET",vlogdata);
            // Send information to log.
            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
            let _ = logs::send_logs(logdata);

            // Fetch headers.
            let vheaders = api_headers();

            // Return answer.
            Ok( HttpResponse::Ok()
                .append_header(("api-version",vheaders[0].clone()))
                .content_type(vheaders[1].clone())
                .body(data) )
        },
        // When error build response.
        Err(cmd_err) => {
            // Construct JSON object
            let data = json!(
                {
                    "Code": 400,
                    "Info": format!("{}", cmd_err),
                    "Status": "Bad Request"
                }
            );

            // Get USER-AGENT from request header, ugly but works.
            let mut ua_string = String::new();
            for v in reqdata.headers().get_all(USER_AGENT) {
                ua_string = format!("{:?}",v);
            };

            // Vec for HttpRequest data to log.
            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
            let vlogdata = vec![
                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                reqdata.connection_info().scheme().to_string(),
                reqdata.path().to_string(),
                reqdata.connection_info().host().to_string(),
                ua_string
            ];

            // Get log function and put requierd data into it.
            let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"GET",vlogdata);
            // Send information to log.
            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
            let _ = logs::send_logs(logdata);

            // Fetch headers.
            let vheaders = api_headers();

            // Return answer.
            Ok( HttpResponse::BadRequest()
                .append_header(("api-version",vheaders[0].clone()))
                .content_type(vheaders[1].clone())
                .json(data) )
        }
    }
}

// Get podman information.
pub async fn get_common_info(reqdata: HttpRequest) -> io::Result<HttpResponse> {
    // The command.
    let cmd = Command::new("podman")
        .arg("info")
        .arg("--format=json")
        .output();

    // Check if command went ok or not.
    match cmd {
        // When Ok build response.
        Ok(cmd_ok) => {
            // Clean data from unneeded characters.
            let data = format!("{}", String::from_utf8_lossy(&cmd_ok.stdout));

            // Get USER-AGENT from request header, ugly but works.
            let mut ua_string = String::new();
            for v in reqdata.headers().get_all(USER_AGENT) {
                ua_string = format!("{:?}",v);
            };

            // Vec for HttpRequest data to log.
            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
            let vlogdata = vec![
                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                reqdata.connection_info().scheme().to_string(),
                reqdata.path().to_string(),
                reqdata.connection_info().host().to_string(),
                ua_string
            ];

            // Get log function and put requierd data into it.
            let vlog: Vec<String> = logs::log_data(200,"Request OK",0,"GET",vlogdata);
            // Send information to log.
            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
            let _ = logs::send_logs(logdata);

            // Fetch headers.
            let vheaders = api_headers();

            // Return answer.
            Ok( HttpResponse::Ok()
                .append_header(("api-version",vheaders[0].clone()))
                .content_type(vheaders[1].clone())
                .body(data) )
        },
        // When error build response.
        Err(cmd_err) => {
            // Construct JSON object
            let data = json!(
                {
                    "Code": 400,
                    "Info": format!("{}", cmd_err),
                    "Status": "Bad Request"
                }
            );

            // Get USER-AGENT from request header, ugly but works.
            let mut ua_string = String::new();
            for v in reqdata.headers().get_all(USER_AGENT) {
                ua_string = format!("{:?}",v);
            };

            // Vec for HttpRequest data to log.
            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
            let vlogdata = vec![
                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                reqdata.connection_info().scheme().to_string(),
                reqdata.path().to_string(),
                reqdata.connection_info().host().to_string(),
                ua_string
            ];

            // Get log function and put requierd data into it.
            let vlog: Vec<String> = logs::log_data(400,"Internal Server Error",4,"GET",vlogdata);
            // Send information to log.
            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
            let _ = logs::send_logs(logdata);

            // Fetch headers.
            let vheaders = api_headers();

            // Return answer.
            Ok( HttpResponse::BadRequest()
                .append_header(("api-version",vheaders[0].clone()))
                .content_type(vheaders[1].clone())
                .json(data) )
        }
    }
}

// Get minimal image list.
pub async fn get_common_image_list(reqdata: HttpRequest) -> io::Result<HttpResponse> {
    // The command.
    let cmd = Command::new("podman")
        .arg("image")
        .arg("ls")
        .arg(
            "--format=
            { \"Names\": {{json .Names}},
            \"ImageSize\": {{json .VirtualSize}},
            \"Created\": {{json .CreatedAt}} },"
        )
        .output();

    // Check if command went ok or not.
    match cmd {
        // When Ok build response.
        Ok(cmd_ok) => {
            // Clean data from unneeded characters.
            let data = format!("[{}]", String::from_utf8_lossy(&cmd_ok.stdout))
            .replace(",\n]", "]");

            // Get USER-AGENT from request header, ugly but works.
            let mut ua_string = String::new();
            for v in reqdata.headers().get_all(USER_AGENT) {
                ua_string = format!("{:?}",v);
            };

            // Vec for HttpRequest data to log.
            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
            let vlogdata = vec![
                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                reqdata.connection_info().scheme().to_string(),
                reqdata.path().to_string(),
                reqdata.connection_info().host().to_string(),
                ua_string
            ];

            // Get log function and put requierd data into it.
            let vlog: Vec<String> = logs::log_data(200,"Request OK",0,"GET",vlogdata);
            // Send information to log.
            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
            let _ = logs::send_logs(logdata);

            // Fetch headers.
            let vheaders = api_headers();

            // Return answer.
            Ok( HttpResponse::Ok()
                .append_header(("api-version",vheaders[0].clone()))
                .content_type(vheaders[1].clone())
                .body(data) )
        },
        // When error build response.
        Err(cmd_err) => {
            // Construct JSON object
            let data = json!(
                {
                    "Code": 400,
                    "Info": format!("{}", cmd_err),
                    "Status": "Bad Request"
                }
            );

            // Get USER-AGENT from request header, ugly but works.
            let mut ua_string = String::new();
            for v in reqdata.headers().get_all(USER_AGENT) {
                ua_string = format!("{:?}",v);
            };

            // Vec for HttpRequest data to log.
            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
            let vlogdata = vec![
                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                reqdata.connection_info().scheme().to_string(),
                reqdata.path().to_string(),
                reqdata.connection_info().host().to_string(),
                ua_string
            ];

            // Get log function and put requierd data into it.
            let vlog: Vec<String> = logs::log_data(400,"Internal Server Error",4,"GET",vlogdata);
            // Send information to log.
            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
            let _ = logs::send_logs(logdata);

            // Fetch headers.
            let vheaders = api_headers();

            // Return answer.
            Ok( HttpResponse::BadRequest()
                .append_header(("api-version",vheaders[0].clone()))
                .content_type(vheaders[1].clone())
                .json(data) )
        }
    }
}

// Get .env files.
pub async fn get_common_env_files(reqdata: HttpRequest) -> io::Result<HttpResponse> {
    // Command
    let cmd = Command::new("ls")
        .arg("-1")
        .arg(format!("{}/",ENV_PATH.get().unwrap()))
        .output();

    // Check if command executed OK.
    match cmd {
        Ok(cmd_ok) => {
            // Create empty JSON object.
            let mut data = json!({});
            // Check length of response.
            let file_count = cmd_ok.stdout.len();
            if file_count == 0 {
                // Construct JSON object.
                data = json!(
                    {
                        "Code": 200,
                        "Info": "No .env files found",
                        "State": 0,
                        "Status": "OK"
                    }
                );
            }
            else if file_count > 4 {
                // Construct JSON object.
                data = json!(
                    {
                        "Code": 200,
                        "Info": "Found .env files",
                        "Files": format!("{}", String::from_utf8_lossy(&cmd_ok.stdout).replace("\n", ",")),
                        "State": 1,
                        "Status": "OK"
                    }
                );
            }

             // Get USER-AGENT from request header, ugly but works.
            let mut ua_string = String::new();
            for v in reqdata.headers().get_all(USER_AGENT) {
                ua_string = format!("{:?}",v);
            };

            // Vec for HttpRequest data to log.
            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
            let vlogdata = vec![
                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                reqdata.connection_info().scheme().to_string(),
                reqdata.path().to_string(),
                reqdata.connection_info().host().to_string(),
                ua_string
            ];

            // Get log function and put requierd data into it.
            let vlog: Vec<String> = logs::log_data(200,"Request OK",0,"GET",vlogdata);
            // Send information to log.
            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
            let _ = logs::send_logs(logdata);

            // Fetch headers.
            let vheaders = api_headers();
            // Return answer.
            return Ok( HttpResponse::Ok()
                .append_header(("api-version",vheaders[0].clone()))
                .content_type(vheaders[1].clone())
                .json(data) );
        }
        Err(cmd_err) => {
            // Construct JSON object
            let data = json!(
                {
                    "Code": 400,
                    "Info": format!("{}", cmd_err),
                    "Status": "Bad Request"
                }
            );

            // Get USER-AGENT from request header, ugly but works.
            let mut ua_string = String::new();
            for v in reqdata.headers().get_all(USER_AGENT) {
                ua_string = format!("{:?}",v);
            };

            // Vec for HttpRequest data to log.
            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
            let vlogdata = vec![
                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                reqdata.connection_info().scheme().to_string(),
                reqdata.path().to_string(),
                reqdata.connection_info().host().to_string(),
                ua_string
            ];

            // Get log function and put requierd data into it.
            let vlog: Vec<String> = logs::log_data(400,"Internal Server Error",4,"GET",vlogdata);
            // Send information to log.
            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
            let _ = logs::send_logs(logdata);

            // Fetch headers.
            let vheaders = api_headers();

            // Return answer.
            Ok( HttpResponse::BadRequest()
                .append_header(("api-version",vheaders[0].clone()))
                .content_type(vheaders[1].clone())
                .json(data) )
        }
    }
}

// Get secret files.
pub async fn get_common_secret_files(reqdata: HttpRequest) -> io::Result<HttpResponse> {
    // Command
    let cmd = Command::new("ls")
        .arg("-1")
        .arg(format!("{}/",SEC_PATH.get().unwrap()))
        .output();

    // Check if command executed OK.
    match cmd {
        Ok(cmd_ok) => {
            // Create empty JSON object.
            let mut data = json!({});
            // Check length of response.
            let file_count = cmd_ok.stdout.len();
            if file_count == 0 {
                // Construct JSON object.
                data = json!(
                    {
                        "Code": 200,
                        "Info": "No secrets files found",
                        "State": 0,
                        "Status": "OK"
                    }
                );
            }
            else if file_count > 4 {
                // Construct JSON object.
                data = json!(
                    {
                        "Code": 200,
                        "Info": "Found secret files",
                        "Files": format!("{}", String::from_utf8_lossy(&cmd_ok.stdout).replace("\n", ",")),
                        "State": 1,
                        "Status": "OK"
                    }
                );
            }

             // Get USER-AGENT from request header, ugly but works.
            let mut ua_string = String::new();
            for v in reqdata.headers().get_all(USER_AGENT) {
                ua_string = format!("{:?}",v);
            };

            // Vec for HttpRequest data to log.
            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
            let vlogdata = vec![
                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                reqdata.connection_info().scheme().to_string(),
                reqdata.path().to_string(),
                reqdata.connection_info().host().to_string(),
                ua_string
            ];

            // Get log function and put requierd data into it.
            let vlog: Vec<String> = logs::log_data(200,"Request OK",0,"GET",vlogdata);
            // Send information to log.
            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
            let _ = logs::send_logs(logdata);

            // Fetch headers.
            let vheaders = api_headers();
            // Return answer.
            return Ok( HttpResponse::Ok()
                .append_header(("api-version",vheaders[0].clone()))
                .content_type(vheaders[1].clone())
                .json(data) );
        }
        Err(cmd_err) => {
            // Construct JSON object
            let data = json!(
                {
                    "Code": 400,
                    "Info": format!("{}", cmd_err),
                    "Status": "Bad Request"
                }
            );

            // Get USER-AGENT from request header, ugly but works.
            let mut ua_string = String::new();
            for v in reqdata.headers().get_all(USER_AGENT) {
                ua_string = format!("{:?}",v);
            };

            // Vec for HttpRequest data to log.
            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
            let vlogdata = vec![
                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                reqdata.connection_info().scheme().to_string(),
                reqdata.path().to_string(),
                reqdata.connection_info().host().to_string(),
                ua_string
            ];

            // Get log function and put requierd data into it.
            let vlog: Vec<String> = logs::log_data(400,"Internal Server Error",4,"GET",vlogdata);
            // Send information to log.
            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
            let _ = logs::send_logs(logdata);

            // Fetch headers.
            let vheaders = api_headers();

            // Return answer.
            Ok( HttpResponse::BadRequest()
                .append_header(("api-version",vheaders[0].clone()))
                .content_type(vheaders[1].clone())
                .json(data) )
        }
    }
}

// Get detailed image list.
pub async fn post_common_image_list_single(sdata: web::Json<GetImageInfo>,reqdata: HttpRequest) -> io::Result<HttpResponse> {
    // Get JSON data from post.
    let jdata = sdata.into_inner();

    // Regex for non allowed characters.
    let rx_repository = Regex::new(r"([^A-Za-z0-9./:_-])").unwrap();

    // Check if regex matches anything in name.
    if jdata.name.is_empty() || rx_repository.find(&&jdata.name.as_str()).is_some() {
        // Construct JSON object
        let data = json!(
            {
                "Code": 400,
                "Info": "Information provided is not in correct format",
                "Status": "Bad Request"
            }
        );

        // Get USER-AGENT from request header, ugly but works.
        let mut ua_string = String::new();
        for v in reqdata.headers().get_all(USER_AGENT) {
            ua_string = format!("{:?}",v);
        };

        // Vec for HttpRequest data to log.
        // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
        let vlogdata = vec![
            reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
            reqdata.connection_info().scheme().to_string(),
            reqdata.path().to_string(),
            reqdata.connection_info().host().to_string(),
            ua_string
        ];

        // Get log function and put requierd data into it.
        let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"POST",vlogdata);
        // Send information to log.
        let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
        let _ = logs::send_logs(logdata);

        // Fetch headers.
        let vheaders = api_headers();

        // Return answer.
        return Ok( HttpResponse::BadRequest()
            .append_header(("api-version",vheaders[0].clone()))
            .content_type(vheaders[1].clone())
            .json(data) );
    }

    // The command.
    let cmd = Command::new("podman")
        .arg("image")
        .arg("inspect")
        .arg(jdata.name)
        .arg("--format=json")
        .output();

    // Check if command went ok or not.
    match cmd {
        // When Ok build response.
        Ok(cmd_ok) => {
            // Clean data from unneeded characters.
            let data = format!("{}", String::from_utf8_lossy(&cmd_ok.stdout));

            // Get USER-AGENT from request header, ugly but works.
            let mut ua_string = String::new();
            for v in reqdata.headers().get_all(USER_AGENT) {
                ua_string = format!("{:?}",v);
            };

            // Vec for HttpRequest data to log.
            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
            let vlogdata = vec![
                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                reqdata.connection_info().scheme().to_string(),
                reqdata.path().to_string(),
                reqdata.connection_info().host().to_string(),
                ua_string
            ];

            // Get log function and put requierd data into it.
            let vlog: Vec<String> = logs::log_data(200,"Request OK",0,"GET",vlogdata);
            // Send information to log.
            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
            let _ = logs::send_logs(logdata);

            // Fetch headers.
            let vheaders = api_headers();

            // Return answer.
            Ok( HttpResponse::Ok()
                .append_header(("api-version",vheaders[0].clone()))
                .content_type(vheaders[1].clone())
                .body(data) )
        },
        // When error build response.
        Err(cmd_err) => {
            // Construct JSON object
            let data = json!(
                {
                    "Code": 400,
                    "Info": format!("{}", cmd_err),
                    "Status": "Bad Request"
                }
            );

            // Get USER-AGENT from request header, ugly but works.
            let mut ua_string = String::new();
            for v in reqdata.headers().get_all(USER_AGENT) {
                ua_string = format!("{:?}",v);
            };

            // Vec for HttpRequest data to log.
            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
            let vlogdata = vec![
                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                reqdata.connection_info().scheme().to_string(),
                reqdata.path().to_string(),
                reqdata.connection_info().host().to_string(),
                ua_string
            ];

            // Get log function and put requierd data into it.
            let vlog: Vec<String> = logs::log_data(400,"Internal Server Error",4,"GET",vlogdata);
            // Send information to log.
            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
            let _ = logs::send_logs(logdata);

            // Fetch headers.
            let vheaders = api_headers();

            // Return answer.
            Ok( HttpResponse::BadRequest()
                .append_header(("api-version",vheaders[0].clone()))
                .content_type(vheaders[1].clone())
                .json(data) )
        }
    }
}

// Post env file
pub async fn post_common_envfile(sdata: web::Json<CreateEnvFile>,reqdata: HttpRequest) -> io::Result<HttpResponse> {
    // Get JSON data from post.
    let jdata = sdata.into_inner();

    // Regex for non allowed characters.
    let rx_name = Regex::new(r"([^A-Za-z0-9_-])").unwrap();
    
    // Check if regex matches anything in name.
    if rx_name.find(jdata.name.as_str()).is_some() {
        // Construct JSON object
        let data = json!(
            {
                "Code": 400,
                "Info": "Name provided is not in correct format",
                "Status": "Bad Request"
            }
        );

        // Get USER-AGENT from request header, ugly but works.
        let mut ua_string = String::new();
        for v in reqdata.headers().get_all(USER_AGENT) {
            ua_string = format!("{:?}",v);
        };

        // Vec for HttpRequest data to log.
        // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
        let vlogdata = vec![
            reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
            reqdata.connection_info().scheme().to_string(),
            reqdata.path().to_string(),
            reqdata.connection_info().host().to_string(),
            ua_string
        ];

        // Get log function and put requierd data into it.
        let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"POST",vlogdata);
        // Send information to log.
        let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
        let _ = logs::send_logs(logdata);

        // Fetch headers.
        let vheaders = api_headers();

        // Return answer.
        return Ok( HttpResponse::BadRequest()
            .append_header(("api-version",vheaders[0].clone()))
            .content_type(vheaders[1].clone())
            .json(data) );
    }

    // Build complete path to file.
    let full_path = format!("{}/{}.env", ENV_PATH.get().unwrap(), jdata.name);

    // Get file exist status.
    let file_exist = Path::new(full_path.as_str()).exists();

    // Check if file exists and if "replace" is set to "no" or other wierd value then error out.
    if jdata.replace != "no" && jdata.replace != "yes" {
        // Construct JSON object
        let data = json!(
            {
                "Code": 400,
                "Info": "Replace tag must contain only yes or no",
                "Status": "Bad Request"
            }
        );

        // Get USER-AGENT from request header, ugly but works.
        let mut ua_string = String::new();
        for v in reqdata.headers().get_all(USER_AGENT) {
            ua_string = format!("{:?}",v);
        };

        // Vec for HttpRequest data to log.
        // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
        let vlogdata = vec![
            reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
            reqdata.connection_info().scheme().to_string(),
            reqdata.path().to_string(),
            reqdata.connection_info().host().to_string(),
            ua_string
        ];

        // Get log function and put requierd data into it.
        let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"POST",vlogdata);
        // Send information to log.
        let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
        let _ = logs::send_logs(logdata);

        // Fetch headers.
        let vheaders = api_headers();

        // Return answer.
        return Ok( HttpResponse::BadRequest()
            .append_header(("api-version",vheaders[0].clone()))
            .content_type(vheaders[1].clone())
            .json(data) );
    }

    // If replace is set to yes, go ahead and write a new file or owerwrite old file.
    if file_exist && jdata.replace == "yes" || !file_exist && jdata.replace == "no" {
        // Create file.
        let mut file = File::create(full_path.clone()).expect("Could not open file");

        // Write to file.
        let content = format!("{}\n",jdata.content.to_string().replace("|", "\n").trim_end());
        file.write_all(content.as_bytes()).expect("ERROR");

        // Build answer.
        let data = json!(
            {
                "Code": "201",
                "Info": "Successfully created env file",
                "File": format!("{}", full_path),
                "Status": "Created"
            }
        );

        // Get USER-AGENT from request header, ugly but works.
        let mut ua_string = String::new();
        for v in reqdata.headers().get_all(USER_AGENT) {
            ua_string = format!("{:?}",v);
        };

        // Vec for HttpRequest data to log.
        // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
        let vlogdata = vec![
            reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
            reqdata.connection_info().scheme().to_string(),
            reqdata.path().to_string(),
            reqdata.connection_info().host().to_string(),
            ua_string
        ];

        // Get log function and put requierd data into it.
        let vlog: Vec<String> = logs::log_data(201,"Created",0,"POST",vlogdata);
        // Send information to log.
        let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={} oCMAPIEnvFile={} oCMAPICreateType=EnvFile",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9],full_path);
        let _ = logs::send_logs(logdata);

        // Fetch headers.
        let vheaders = api_headers();

        // Return answer.
        Ok( HttpResponse::Created()
            .append_header(("api-version",vheaders[0].clone()))
            .content_type(vheaders[1].clone())
            .json(data) )
    }
    else {
        // Construct JSON object
        let data = json!(
            {
                "Code": 400,
                "Info": "File already exist, set replace to yes or change the name",
                "Status": "Bad Request"
            }
        );

        // Get USER-AGENT from request header, ugly but works.
        let mut ua_string = String::new();
        for v in reqdata.headers().get_all(USER_AGENT) {
            ua_string = format!("{:?}",v);
        };

        // Vec for HttpRequest data to log.
        // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
        let vlogdata = vec![
            reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
            reqdata.connection_info().scheme().to_string(),
            reqdata.path().to_string(),
            reqdata.connection_info().host().to_string(),
            ua_string
        ];

        // Get log function and put requierd data into it.
        let vlog: Vec<String> = logs::log_data(400,"Bad Request",6,"POST",vlogdata);
        // Send information to log.
        let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
        let _ = logs::send_logs(logdata);

        // Fetch headers.
        let vheaders = api_headers();

        // Return answer.
        Ok( HttpResponse::BadRequest()
            .append_header(("api-version",vheaders[0].clone()))
            .content_type(vheaders[1].clone())
            .json(data) )
    }
}

// Post secret information
pub async fn post_common_secret(sdata: web::Json<CreateSecret>,reqdata: HttpRequest) -> io::Result<HttpResponse> {
    // Get JSON data from post.
    let jdata = sdata.into_inner();

    // Regex for non allowed characters in name.
    let rx_name = Regex::new(r"([^A-Za-z0-9_-])").unwrap();

    // Regex for non allowed characters in labels.
    let rx_labels = Regex::new(r"([^A-Za-z0-9_\-\=\,])").unwrap();

    // Check if regex matches anything in name.
    if rx_name.find(jdata.name.as_str()).is_some() {
        // Construct JSON object
        let data = json!(
            {
                "Code": 400,
                "Info": "Name provided is not in correct format",
                "Status": "Bad Request"
            }
        );

        // Get USER-AGENT from request header, ugly but works.
        let mut ua_string = String::new();
        for v in reqdata.headers().get_all(USER_AGENT) {
            ua_string = format!("{:?}",v);
        };

        // Vec for HttpRequest data to log.
        // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
        let vlogdata = vec![
            reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
            reqdata.connection_info().scheme().to_string(),
            reqdata.path().to_string(),
            reqdata.connection_info().host().to_string(),
            ua_string
        ];

        // Get log function and put requierd data into it.
        let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"POST",vlogdata);
        // Send information to log.
        let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
        let _ = logs::send_logs(logdata);

        // Fetch headers.
        let vheaders = api_headers();

        // Return answer.
        return Ok( HttpResponse::BadRequest()
            .append_header(("api-version",vheaders[0].clone()))
            .content_type(vheaders[1].clone())
            .json(data) );
    }

    // Check if regex matches anything in labels.
    if rx_labels.find(jdata.labels.as_str()).is_some() {
        // Construct JSON object
        let data = json!(
            {
                "Code": 400,
                "Info": "Labels provided is not in correct format",
                "Status": "Bad Request"
            }
        );

        // Get USER-AGENT from request header, ugly but works.
        let mut ua_string = String::new();
        for v in reqdata.headers().get_all(USER_AGENT) {
            ua_string = format!("{:?}",v);
        };

        // Vec for HttpRequest data to log.
        // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
        let vlogdata = vec![
            reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
            reqdata.connection_info().scheme().to_string(),
            reqdata.path().to_string(),
            reqdata.connection_info().host().to_string(),
            ua_string
        ];

        // Get log function and put requierd data into it.
        let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"POST",vlogdata);
        // Send information to log.
        let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
        let _ = logs::send_logs(logdata);

        // Fetch headers.
        let vheaders = api_headers();

        // Return answer.
        return Ok( HttpResponse::BadRequest()
            .append_header(("api-version",vheaders[0].clone()))
            .content_type(vheaders[1].clone())
            .json(data) );
    }

    // Check if path exist, error out if not.
    if !Path::new(SEC_PATH.get().unwrap()).exists() {
        // Construct JSON object
        let data = json!(
            {
                "Code": 400,
                "Info": "Path do not exist, check configuration",
                "Status": "Bad Request"
            }
        );

        // Get USER-AGENT from request header, ugly but works.
        let mut ua_string = String::new();
        for v in reqdata.headers().get_all(USER_AGENT) {
            ua_string = format!("{:?}",v);
        };

        // Vec for HttpRequest data to log.
        // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
        let vlogdata = vec![
            reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
            reqdata.connection_info().scheme().to_string(),
            reqdata.path().to_string(),
            reqdata.connection_info().host().to_string(),
            ua_string
        ];

        // Get log function and put requierd data into it.
        let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"POST",vlogdata);
        // Send information to log.
        let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
        let _ = logs::send_logs(logdata);

        // Fetch headers.
        let vheaders = api_headers();

        // Return answer.
        return Ok( HttpResponse::BadRequest()
            .append_header(("api-version",vheaders[0].clone()))
            .content_type(vheaders[1].clone())
            .json(data) );
    }

     // Build complete path to file.
    let mut full_path = format!("{}/{}_{}.tmp", SEC_PATH.get().unwrap(), jdata.name, rand_number());

    // Get file exists status.
    let file_exist = Path::new(full_path.as_str()).exists();

    // if for some wierd reason a file already exist with same filename
    // generate a new one, but folder should always be empty.
    if file_exist {
        full_path = format!("{}/{}_{}.tmp", SEC_PATH.get().unwrap(), jdata.name, rand_number());
    }
    
    // Create empty vec for cmd args.
    let mut argdata: Vec<String> = Vec::new();

    // Fill vec based on input.
    argdata.push("secret".to_string());
    argdata.push("create".to_string());
    if jdata.labels.trim().len() > 1 {
        // iterate through lables tag and split back to vec.
        for v in jdata.labels.split(",").map(String::from) {
            argdata.push(format!("--label={}",v.trim()).to_string());
        }
    }
    if jdata.replace.trim() == "yes" {
        argdata.push("--replace=true".to_string());
    }
    argdata.push(jdata.name.clone());
    argdata.push(full_path.clone());

    // Command for checking if secret exists.
    let cmd_exist = Command::new("podman")
    .arg("secret")
    .arg("exists")
    .arg(jdata.name)
    .status()
    .expect("Expect Nothing...");

    // Check exit code to see if secret already exists or not.
    // Exit codes: 0 = do not exist, 1 = exists.
    match cmd_exist.code() {
        // Secret do not exist, time to create it.
        Some(1) => {
            // Create file.
            let mut file = File::create(full_path.clone()).expect("Could not open file");
            // Write to file.
            let content = format!("{}\n",jdata.data.to_string().replace("|", "\n").trim_end());
            file.write_all(content.as_bytes()).expect("ERROR");

            // Create command.
            let mut cmd_create = Command::new("podman");
            for v in argdata.iter() {
                cmd_create.arg(v);
            }

            // Run command and get response.
            match cmd_create.output() {
                Ok(cmd_create_ok) => {
                    // Get exit code.
                    let exit_code = cmd_create_ok.status.code();

                    // Remove temporary file.
                    let _ = fs::remove_file(full_path);

                    match exit_code {
                        Some(0) => {
                            // Create response.
                            let data = json!(
                                {
                                    "Code": "201",
                                    "Info": "Successfully created secret",
                                    "ID": format!("{}", String::from_utf8_lossy(&cmd_create_ok.stdout).trim_end()),
                                    "Status": "Created"
                                }
                            );

                            // Get USER-AGENT from request header, ugly but works.
                            let mut ua_string = String::new();
                            for v in reqdata.headers().get_all(USER_AGENT) {
                                ua_string = format!("{:?}",v);
                            };

                            // Vec for HttpRequest data to log.
                            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
                            let vlogdata = vec![
                                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                                reqdata.connection_info().scheme().to_string(),
                                reqdata.path().to_string(),
                                reqdata.connection_info().host().to_string(),
                                ua_string
                            ];

                            // Get log function and put requierd data into it.
                            let vlog: Vec<String> = logs::log_data(201,"Created",0,"POST",vlogdata);
                            // Extra create data for log.
                            let cdata = format!("{:?}",String::from_utf8_lossy(&cmd_create_ok.stdout).trim_end());
                            // Send information to log.
                            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={} oCMAPICreateID={} oCMAPICreateType=Secret",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9],cdata);
                            let _ = logs::send_logs(logdata);

                            // Fetch headers.
                            let vheaders = api_headers();

                            // Return answer.
                            Ok( HttpResponse::Created()
                                .append_header(("api-version",vheaders[0].clone()))
                                .content_type(vheaders[1].clone())
                                .json(data) )
                        }
                        _ => {
                            // Construct JSON object
                            let data = json!(
                                {
                                    "Code": 400,
                                    "Info": format!("{}", format!("{}",&cmd_create_ok.status).trim_end().replace("\n", ", ")),
                                    "Status": "Bad Request"
                                }
                            );

                            // Get USER-AGENT from request header, ugly but works.
                            let mut ua_string = String::new();
                            for v in reqdata.headers().get_all(USER_AGENT) {
                                ua_string = format!("{:?}",v);
                            };

                            // Vec for HttpRequest data to log.
                            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
                            let vlogdata = vec![
                                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                                reqdata.connection_info().scheme().to_string(),
                                reqdata.path().to_string(),
                                reqdata.connection_info().host().to_string(),
                                ua_string
                            ];

                            // Get log function and put requierd data into it.
                            let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"POST",vlogdata);
                            // Send information to log.
                            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
                            let _ = logs::send_logs(logdata);

                            // Fetch headers.
                            let vheaders = api_headers();

                            // Return answer.
                            Ok( HttpResponse::BadRequest()
                                .append_header(("api-version",vheaders[0].clone()))
                                .content_type(vheaders[1].clone())
                                .json(data) )
                        }
                    }
                }
                // This match path will never trigger due to underlying program behaviour at the moment...
                Err(cmd_create_err) => {
                    // Remove temporary file.
                    let _ = fs::remove_file(full_path);

                    // Construct JSON object
                    let data = json!(
                        {
                            "Code": 400,
                            "Info": format!("{}", format!("{}",&cmd_create_err).trim_end().replace("\n", ", ")),
                            "Status": "Bad Request"
                        }
                    );

                    // Get USER-AGENT from request header, ugly but works.
                    let mut ua_string = String::new();
                    for v in reqdata.headers().get_all(USER_AGENT) {
                        ua_string = format!("{:?}",v);
                    };

                    // Vec for HttpRequest data to log.
                    // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
                    let vlogdata = vec![
                        reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                        reqdata.connection_info().scheme().to_string(),
                        reqdata.path().to_string(),
                        reqdata.connection_info().host().to_string(),
                        ua_string
                    ];

                    // Get log function and put requierd data into it.
                    let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"POST",vlogdata);
                    // Send information to log.
                    let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
                    let _ = logs::send_logs(logdata);

                    // Fetch headers.
                    let vheaders = api_headers();

                    // Return answer.
                    Ok( HttpResponse::BadRequest()
                        .append_header(("api-version",vheaders[0].clone()))
                        .content_type(vheaders[1].clone())
                        .json(data) )
                }
            }
        },
        // Secret already exist.
        Some(0) => {
            // Check if to overwrite secret.
            if jdata.replace.trim() == "yes" {
                // Create file.
                let mut file = File::create(full_path.clone()).expect("Could not open file");
                // Write to file.
                let content = format!("{}\n",jdata.data.to_string().replace("|", "\n").trim_end());
                file.write_all(content.as_bytes()).expect("ERROR");

                // Secret exists, time to replace it.
                let mut cmd_create = Command::new("podman");
                for v in argdata.iter() {
                    cmd_create.arg(v);
                }

                // Run command and get response.
                match cmd_create.output() {
                    Ok(cmd_create_ok) => {
                        // Get exit code.
                        let exit_code = cmd_create_ok.status.code();

                        // Remove temporary file.
                        let _ = fs::remove_file(full_path);

                        match exit_code {
                            Some(0) => {
                                // Create response.
                                let data = json!(
                                    {
                                        "Code": "201",
                                        "Info": "Successfully created secret",
                                        "ID": format!("{}", String::from_utf8_lossy(&cmd_create_ok.stdout).trim_end()),
                                        "Status": "Created"
                                    }
                                );
                                // Get USER-AGENT from request header, ugly but works.
                                let mut ua_string = String::new();
                                for v in reqdata.headers().get_all(USER_AGENT) {
                                    ua_string = format!("{:?}",v);
                                };

                                // Vec for HttpRequest data to log.
                                // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
                                let vlogdata = vec![
                                    reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                                    reqdata.connection_info().scheme().to_string(),
                                    reqdata.path().to_string(),
                                    reqdata.connection_info().host().to_string(),
                                    ua_string
                                ];

                                // Get log function and put requierd data into it.
                                let vlog: Vec<String> = logs::log_data(201,"Created",0,"POST",vlogdata);
                                // Extra create data for log.
                                let cdata = format!("{:?}",String::from_utf8_lossy(&cmd_create_ok.stdout).trim_end());
                                // Send information to log.
                                let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={} oCMAPICreateID={} oCMAPICreateType=Secret",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9],cdata);
                                let _ = logs::send_logs(logdata);
                                // Fetch headers.
                                let vheaders = api_headers();
                                // Return answer.
                                Ok( HttpResponse::Created()
                                    .append_header(("api-version",vheaders[0].clone()))
                                    .content_type(vheaders[1].clone())
                                    .json(data) )
                            },
                            _ => {
                                // Construct JSON object
                                let data = json!(
                                    {
                                        "Code": 400,
                                        "Info": format!("{}", format!("{}",&cmd_create_ok.status).trim_end().replace("\n", ", ")),
                                        "Status": "Bad Request"
                                    }
                                );

                                // Get USER-AGENT from request header, ugly but works.
                                let mut ua_string = String::new();
                                for v in reqdata.headers().get_all(USER_AGENT) {
                                    ua_string = format!("{:?}",v);
                                };

                                // Vec for HttpRequest data to log.
                                // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
                                let vlogdata = vec![
                                    reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                                    reqdata.connection_info().scheme().to_string(),
                                    reqdata.path().to_string(),
                                    reqdata.connection_info().host().to_string(),
                                    ua_string
                                ];

                                // Get log function and put requierd data into it.
                                let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"POST",vlogdata);
                                // Send information to log.
                                let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
                                let _ = logs::send_logs(logdata);

                                // Fetch headers.
                                let vheaders = api_headers();

                                // Return answer.
                                Ok( HttpResponse::BadRequest()
                                    .append_header(("api-version",vheaders[0].clone()))
                                    .content_type(vheaders[1].clone())
                                    .json(data) )
                            }
                        }
                    }
                    // This match path will never trigger due to underlying program behaviour at the moment...
                    Err(cmd_create_err) => {
                        // Remove temporary file.
                        let _ = fs::remove_file(full_path);

                        // Construct JSON object
                        let data = json!(
                            {
                                "Code": 400,
                                "Info": format!("{}", format!("{}",cmd_create_err).trim_end().replace("\n", ", ")),
                                "Status": "Bad Request"
                            }
                        );

                        // Get USER-AGENT from request header, ugly but works.
                        let mut ua_string = String::new();
                        for v in reqdata.headers().get_all(USER_AGENT) {
                            ua_string = format!("{:?}",v);
                        };
                        // Vec for HttpRequest data to log.
                        // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
                        let vlogdata = vec![
                            reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                            reqdata.connection_info().scheme().to_string(),
                            reqdata.path().to_string(),
                            reqdata.connection_info().host().to_string(),
                            ua_string
                        ];

                        // Get log function and put requierd data into it.
                        let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"POST",vlogdata);
                        // Send information to log.
                        let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
                        let _ = logs::send_logs(logdata);

                        // Fetch headers.
                        let vheaders = api_headers();

                        // Return answer.
                        Ok( HttpResponse::BadRequest()
                            .append_header(("api-version",vheaders[0].clone()))
                            .content_type(vheaders[1].clone())
                            .json(data) )
                    }
                }
            }
            else {
                // Construct JSON object
                let data = json!(
                    {
                        "Code": 400,
                        "Info": "Either the secret already exists or invalid input was given",
                        "Status": "Bad Request"
                    }
                );

                // Get USER-AGENT from request header, ugly but works.
                let mut ua_string = String::new();
                for v in reqdata.headers().get_all(USER_AGENT) {
                    ua_string = format!("{:?}",v);
                };

                // Vec for HttpRequest data to log.
                // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
                let vlogdata = vec![
                    reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                    reqdata.connection_info().scheme().to_string(),
                    reqdata.path().to_string(),
                    reqdata.connection_info().host().to_string(),
                    ua_string
                ];

                // Get log function and put requierd data into it.
                let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"POST",vlogdata);
                // Send information to log.
                let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
                let _ = logs::send_logs(logdata);

                // Fetch headers.
                let vheaders = api_headers();

                // Return answer.
                Ok( HttpResponse::BadRequest()
                    .append_header(("api-version",vheaders[0].clone()))
                    .content_type(vheaders[1].clone())
                    .json(data) )
            }
        },
        _ => {
            // When none of the exit codes match required ones.
            // Construct JSON object
            let data = json!(
                {
                    "Code": 500,
                    "Info": "Server returned unknown response",
                    "Status": "Internal Server Error"
                }
            );

            // Get USER-AGENT from request header, ugly but works.
            let mut ua_string = String::new();
            for v in reqdata.headers().get_all(USER_AGENT) {
                ua_string = format!("{:?}",v);
            };

            // Vec for HttpRequest data to log.
            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
            let vlogdata = vec![
                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                reqdata.connection_info().scheme().to_string(),
                reqdata.path().to_string(),
                reqdata.connection_info().host().to_string(),
                ua_string
            ];

            // Get log function and put requierd data into it.
            let vlog: Vec<String> = logs::log_data(500,"Internal Error",6,"POST",vlogdata);
            // Send information to log.
            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
            let _ = logs::send_logs(logdata);

            // Fetch headers.
            let vheaders = api_headers();

            // Return answer.
            Ok( HttpResponse::BadRequest()
                .append_header(("api-version",vheaders[0].clone()))
                .content_type(vheaders[1].clone())
                .json(data) )
        }
    }
}

// Get new container image.
pub async fn post_common_container_image(sdata: web::Json<GetImage>,reqdata: HttpRequest) -> io::Result<HttpResponse> {
    // Get JSON data from post.
    let jdata= sdata.into_inner();

    // Regex for non allowed characters.
    let rx_repository = Regex::new(r"([^A-Za-z0-9.\/_-])").unwrap();
    let rx_image = Regex::new(r"([^A-Za-z0-9_-])").unwrap();
    let rx_version = Regex::new(r"([^A-Za-z0-9._-])").unwrap();
    
    // Check if regex matches anything in name.
    if rx_repository.find(jdata.repository.as_str()).is_some() || rx_image.find(jdata.image.as_str()).is_some() || rx_version.find(jdata.version.as_str()).is_some() {
        // Construct JSON object
        let data = json!(
            {
                "Code": 400,
                "Info": "Information provided is not in correct format",
                "Status": "Bad Request"
            }
        );

        // Get USER-AGENT from request header, ugly but works.
        let mut ua_string = String::new();
        for v in reqdata.headers().get_all(USER_AGENT) {
            ua_string = format!("{:?}",v);
        };

        // Vec for HttpRequest data to log.
        // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
        let vlogdata = vec![
            reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
            reqdata.connection_info().scheme().to_string(),
            reqdata.path().to_string(),
            reqdata.connection_info().host().to_string(),
            ua_string
        ];

        // Get log function and put requierd data into it.
        let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"POST",vlogdata);
        // Send information to log.
        let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
        let _ = logs::send_logs(logdata);

        // Fetch headers.
        let vheaders = api_headers();

        // Return answer.
        return Ok( HttpResponse::BadRequest()
            .append_header(("api-version",vheaders[0].clone()))
            .content_type(vheaders[1].clone())
            .json(data) );
    }

    // The command.
    let cmd = Command::new("podman")
        .arg("image")
        .arg("pull")
        .arg(format!("{}/{}:{}",jdata.repository.clone().trim(),jdata.image.clone().trim(),jdata.version.clone().trim()))
        .output();

    // Check if command went ok or not.
    match cmd {
        // When Ok build response.
        Ok(cmd_ok) => {
            // Check if stdout returns image string, should always do that.
            if !cmd_ok.stdout.is_empty() {
                // Clean data from unneeded characters.
                let data = json!(
                    {
                        "Code": "201",
                        "Info": "Image successfully pulled",
                        "ID": format!("{}", String::from_utf8_lossy(&cmd_ok.stdout).trim()),
                        "Status": "Created"
                    }
                );

                // Get USER-AGENT from request header, ugly but works.
                let mut ua_string = String::new();
                for v in reqdata.headers().get_all(USER_AGENT) {
                    ua_string = format!("{:?}",v);
                };

                // Vec for HttpRequest data to log.
                // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
                let vlogdata = vec![
                    reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                    reqdata.connection_info().scheme().to_string(),
                    reqdata.path().to_string(),
                    reqdata.connection_info().host().to_string(),
                    ua_string
                ];

                // Get log function and put requierd data into it.
                let vlog: Vec<String> = logs::log_data(201,"Created",0,"GET",vlogdata);
                // Send information to log.
                let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
                let _ = logs::send_logs(logdata);

                // Fetch headers.
                let vheaders = api_headers();

                // Return answer.
                Ok( HttpResponse::Created()
                    .append_header(("api-version",vheaders[0].clone()))
                    .content_type(vheaders[1].clone())
                    .json(data) )
            }
            else {
                // Formated error message string.
                let err_msg = format!("{}", String::from_utf8_lossy(&cmd_ok.stderr));

                // Error string for JSON with default message.
                let mut err_output = "Unknown Error".to_string();

                // Check return message and show cleaner message.
                if err_msg.contains("invalid character") {
                    err_output = "Return data contains invalid characters".to_string();
                }
                else if err_msg.contains("manifest unknown") {
                    err_output = "Cannot find image at source, manifest unknown".to_string();
                }
                else if err_msg.contains("no such host") {
                    err_output = "Cannot connect to repository no such host".to_string();
                }
                else if err_msg.contains("authentication required") {
                    err_output = "Cannot download image, authentication required".to_string();
                }

                // Construct JSON object
                let data = json!(
                    {
                        "Code": 400,
                        "Info": format!("{}",err_output),
                        "Status": "Bad Request"
                    }
                );

                // Get USER-AGENT from request header, ugly but works.
                let mut ua_string = String::new();
                for v in reqdata.headers().get_all(USER_AGENT) {
                    ua_string = format!("{:?}",v);
                };

                // Vec for HttpRequest data to log.
                // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
                let vlogdata = vec![
                    reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                    reqdata.connection_info().scheme().to_string(),
                    reqdata.path().to_string(),
                    reqdata.connection_info().host().to_string(),
                    ua_string
                ];

                // Get log function and put requierd data into it.
                let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"POST",vlogdata);
                // Send information to log.
                let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
                let _ = logs::send_logs(logdata);

                // Fetch headers.
                let vheaders = api_headers();

                // Return answer.
                return Ok( HttpResponse::BadRequest()
                    .append_header(("api-version",vheaders[0].clone()))
                    .content_type(vheaders[1].clone())
                    .json(data) );
                }
        },
        // When error build response.
        Err(cmd_err) => {
            // Construct JSON object
            let data = json!(
                {
                    "Code": 400,
                    "Info": format!("{}", cmd_err),
                    "Status": "Bad Request"
                }
            );

            // Get USER-AGENT from request header, ugly but works.
            let mut ua_string = String::new();
            for v in reqdata.headers().get_all(USER_AGENT) {
                ua_string = format!("{:?}",v);
            };

            // Vec for HttpRequest data to log.
            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
            let vlogdata = vec![
                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                reqdata.connection_info().scheme().to_string(),
                reqdata.path().to_string(),
                reqdata.connection_info().host().to_string(),
                ua_string
            ];

            // Get log function and put requierd data into it.
            let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"POST",vlogdata);
            // Send information to log.
            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
            let _ = logs::send_logs(logdata);

            // Fetch headers.
            let vheaders = api_headers();

            // Return answer.
            Ok( HttpResponse::BadRequest()
                .append_header(("api-version",vheaders[0].clone()))
                .content_type(vheaders[1].clone())
                .json(data) )
        }
    }
}

// Post Repository login.
pub async fn post_common_repository_login(sdata: web::Json<RepoLogin>,reqdata: HttpRequest) -> io::Result<HttpResponse> {
    // Get JSON data from post.
    let jdata= sdata.into_inner();

    // Regex for non allowed characters.
    let rx_repository = Regex::new(r"([^A-Za-z0-9.\/_-])").unwrap();
    let rx_username = Regex::new(r"([^A-Za-z0-9._-])").unwrap();
    let rx_password = Regex::new(r#"([""'/\`´\\=])"#).unwrap();
    
    // Check if regex matches anything in name.
    if jdata.username.is_empty() || jdata.password.is_empty() || jdata.repository.is_empty() || rx_repository.find(&&jdata.repository.as_str()).is_some() || rx_username.find(&jdata.username.as_str()).is_some() || rx_password.find(jdata.password.as_str()).is_some() {
        // Construct JSON object
        let data = json!(
            {
                "Code": 400,
                "Info": "Information provided is not in correct format",
                "Status": "Bad Request"
            }
        );

        // Get USER-AGENT from request header, ugly but works.
        let mut ua_string = String::new();
        for v in reqdata.headers().get_all(USER_AGENT) {
            ua_string = format!("{:?}",v);
        };

        // Vec for HttpRequest data to log.
        // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
        let vlogdata = vec![
            reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
            reqdata.connection_info().scheme().to_string(),
            reqdata.path().to_string(),
            reqdata.connection_info().host().to_string(),
            ua_string
        ];

        // Get log function and put requierd data into it.
        let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"POST",vlogdata);
        // Send information to log.
        let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
        let _ = logs::send_logs(logdata);

        // Fetch headers.
        let vheaders = api_headers();

        // Return answer.
        return Ok( HttpResponse::BadRequest()
            .append_header(("api-version",vheaders[0].clone()))
            .content_type(vheaders[1].clone())
            .json(data) );
    }

    // The command.
    let cmd= format!("podman login '{}' --username='{}' --password='{}'",jdata.repository,jdata.username,jdata.password);
    let output = fake_tty::bash_command(cmd.as_str()).unwrap()
        .output()
        .expect("Logging in to repository...");

    // Check if command went ok or not.
    match output.status.code() {
        // When Ok build response.
        Some(0) => {
            // Clean data from uneeded characters.
            let data = json!(
                {
                    "Code": 200,
                    "Info": "Successfully logged in to repository",
                    "Status": "OK"
                }
            );

            // Get USER-AGENT from request header, ugly but works.
            let mut ua_string = String::new();
            for v in reqdata.headers().get_all(USER_AGENT) {
                ua_string = format!("{:?}",v);
            };
            // Vec for HttpRequest data to log.
            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
            let vlogdata = vec![
                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                reqdata.connection_info().scheme().to_string(),
                reqdata.path().to_string(),
                reqdata.connection_info().host().to_string(),
                ua_string
            ];

            // Get log function and put requierd data into it.
            let vlog: Vec<String> = logs::log_data(200,"Created",0,"GET",vlogdata);
            // Send information to log.
            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
            let _ = logs::send_logs(logdata);

            // Fetch headers.
            let vheaders = api_headers();

            // Return answer.
            Ok( HttpResponse::Ok()
                .append_header(("api-version",vheaders[0].clone()))
                .content_type(vheaders[1].clone())
                .json(data) )
        },
        // When error build response.
        _ => {
            // Formated error message string.
            let err_msg = format!("{}", String::from_utf8_lossy(&output.stdout));

            // Error string for JSON with default message.
            let mut err_output = "Unknown Error".to_string();

            // Check return message and show cleaner message.
            if err_msg.contains("invalid username") {
                err_output = "Cannot login, invalid username/password".to_string();
            }
            else if err_msg.contains("no such host") {
                err_output = "Cannot connect to source, no such host".to_string();
            }

            // Construct JSON object
            let data = json!(
                {
                    "Code": 400,
                    "Info": err_output,
                    "Status": "Bad Request",
                }
            );

            // Get USER-AGENT from request header, ugly but works.
            let mut ua_string = String::new();
            for v in reqdata.headers().get_all(USER_AGENT) {
                ua_string = format!("{:?}",v);
            };

            // Vec for HttpRequest data to log.
            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
            let vlogdata = vec![
                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                reqdata.connection_info().scheme().to_string(),
                reqdata.path().to_string(),
                reqdata.connection_info().host().to_string(),
                ua_string
            ];

            // Get log function and put requierd data into it.
            let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"POST",vlogdata);
            // Send information to log.
            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
            let _ = logs::send_logs(logdata);

            // Fetch headers.
            let vheaders = api_headers();

            // Return answer.
            Ok( HttpResponse::BadRequest()
                .append_header(("api-version",vheaders[0].clone()))
                .content_type(vheaders[1].clone())
                .json(data) )
        }
    }
}

// Post Repository logout.
pub async fn post_common_repository_logout(sdata: web::Json<RepoLogOut>,reqdata: HttpRequest) -> io::Result<HttpResponse> {

    // Get JSON data from post.
    let jdata= sdata.into_inner();

    // Regex for non allowed characters.
    let rx_repository = Regex::new(r"([^A-Za-z0-9.\/_-])").unwrap();
    
    // Check if regex matches anything in name.
    if jdata.repository.is_empty() || rx_repository.find(&&jdata.repository.as_str()).is_some() {
        // Construct JSON object
        let data = json!(
            {
                "Code": 400,
                "Info": "Information provided is not in correct format",
                "Status": "Bad Request"
            }
        );

        // Get USER-AGENT from request header, ugly but works.
        let mut ua_string = String::new();
        for v in reqdata.headers().get_all(USER_AGENT) {
            ua_string = format!("{:?}",v);
        };

        // Vec for HttpRequest data to log.
        // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
        let vlogdata = vec![
            reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
            reqdata.connection_info().scheme().to_string(),
            reqdata.path().to_string(),
            reqdata.connection_info().host().to_string(),
            ua_string
        ];

        // Get log function and put requierd data into it.
        let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"POST",vlogdata);
        // Send information to log.
        let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
        let _ = logs::send_logs(logdata);

        // Fetch headers.
        let vheaders = api_headers();

        // Return answer.
        return Ok( HttpResponse::BadRequest()
            .append_header(("api-version",vheaders[0].clone()))
            .content_type(vheaders[1].clone())
            .json(data) );
    }

    // The command.
    let cmd= format!("podman logout '{}'",jdata.repository);
    let output = fake_tty::bash_command(cmd.as_str()).unwrap()
        .output()
        .expect("logging out of repository...");

    // Check if command went ok or not.
    match output.status.code() {
        // When Ok build response.
        Some(0) => {
            // Clean data from uneeded characters.
            let data = json!(
                {
                    "Code": 200,
                    "Message": "Successfully logged out from repository",
                    "Status": "OK"
                }
            );

            // Get USER-AGENT from request header, ugly but works.
            let mut ua_string = String::new();
            for v in reqdata.headers().get_all(USER_AGENT) {
                ua_string = format!("{:?}",v);
            };

            // Vec for HttpRequest data to log.
            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
            let vlogdata = vec![
                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                reqdata.connection_info().scheme().to_string(),
                reqdata.path().to_string(),
                reqdata.connection_info().host().to_string(),
                ua_string
            ];

            // Get log function and put requierd data into it.
            let vlog: Vec<String> = logs::log_data(200,"Created",0,"GET",vlogdata);
            // Send information to log.
            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
            let _ = logs::send_logs(logdata);

            // Fetch headers.
            let vheaders = api_headers();

            // Return answer.
            Ok( HttpResponse::Ok()
                .append_header(("api-version",vheaders[0].clone()))
                .content_type(vheaders[1].clone())
                .json(data) )
        },
        // When error build response.
        _ => {
            // Formated error message string.
            let err_msg = format!("{}", String::from_utf8_lossy(&output.stdout));

            // Error string for JSON with default message.
            let mut err_output = "Unknown Error".to_string();

            // Check return message and show cleaner message.
            if err_msg.contains("not logged into") {
                err_output = "Already logged out of repository".to_string();
            }

            // Construct JSON object
            let data = json!(
                {
                    "Code": 400,
                    "Info": err_output,
                    "Status": "Bad Request",
                }
            );

            // Get USER-AGENT from request header, ugly but works.
            let mut ua_string = String::new();
            for v in reqdata.headers().get_all(USER_AGENT) {
                ua_string = format!("{:?}",v);
            };

            // Vec for HttpRequest data to log.
            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
            let vlogdata = vec![
                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                reqdata.connection_info().scheme().to_string(),
                reqdata.path().to_string(),
                reqdata.connection_info().host().to_string(),
                ua_string
            ];

            // Get log function and put requierd data into it.
            let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"POST",vlogdata);
            // Send information to log.
            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
            let _ = logs::send_logs(logdata);

            // Fetch headers.
            let vheaders = api_headers();

            // Return answer.
            Ok( HttpResponse::BadRequest()
                .append_header(("api-version",vheaders[0].clone()))
                .content_type(vheaders[1].clone())
                .json(data) )
        }
    }
}

// Delete env file
pub async fn delete_common_envfile(sdata: web::Json<DeleteEnvFile>, reqdata: HttpRequest) -> io::Result<HttpResponse> {
    // Get JSON data from post.
    let jdata = sdata.into_inner();

    // Regex for non allowed characters.
    let rx_name = Regex::new(r"([^A-Za-z0-9_-])").unwrap();
    
    // Check if regex matches anything in name.
    if rx_name.find(jdata.name.as_str()).is_some() {
        // Construct JSON object
        let data = json!(
            {
                "Code": 400,
                "Info": "Name provided is not in correct format",
                "Status": "Bad Request"
            }
        );

        // Get USER-AGENT from request header, ugly but works.
        let mut ua_string = String::new();
        for v in reqdata.headers().get_all(USER_AGENT) {
            ua_string = format!("{:?}",v);
        };

        // Vec for HttpRequest data to log.
        // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
        let vlogdata = vec![
            reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
            reqdata.connection_info().scheme().to_string(),
            reqdata.path().to_string(),
            reqdata.connection_info().host().to_string(),
            ua_string
        ];

        // Get log function and put requierd data into it.
        let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"DELETE",vlogdata);
        // Send information to log.
        let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
        let _ = logs::send_logs(logdata);

        // Fetch headers.
        let vheaders = api_headers();

        // Return answer.
        return Ok( HttpResponse::BadRequest()
            .append_header(("api-version",vheaders[0].clone()))
            .content_type(vheaders[1].clone())
            .json(data) );
    }

    // Build complete path to file.
    let full_path = format!("{}/{}.env", ENV_PATH.get().unwrap(), jdata.name);

    // Get file exist status.
    let file_exist = Path::new(full_path.as_str()).exists();

    // Check if secret isn´t empty and not matching key from config.
    if jdata.key.is_empty() || jdata.key != COMMON_KEY.get().unwrap().to_string() {
        // Construct JSON object
        let data = json!(
            {
                "Code": 400,
                "Info": "Information missing or bad data",
                "Status": "Bad Request"
            }
        );

        // Get USER-AGENT from request header, ugly but works.
        let mut ua_string = String::new();
        for v in reqdata.headers().get_all(USER_AGENT) {
            ua_string = format!("{:?}",v);
        };

        // Vec for HttpRequest data to log.
        // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
        let vlogdata = vec![
            reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
            reqdata.connection_info().scheme().to_string(),
            reqdata.path().to_string(),
            reqdata.connection_info().host().to_string(),
            ua_string
        ];

        // Get log function and put requierd data into it.
        let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"DELETE",vlogdata);
        // Send information to log.
        let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
        let _ = logs::send_logs(logdata);

        // Fetch headers.
        let vheaders = api_headers();

        // Return answer.
        return Ok( HttpResponse::BadRequest()
            .append_header(("api-version",vheaders[0].clone()))
            .content_type(vheaders[1].clone())
            .json(data) );
    }

    // Match key to make sure it is correct and see if file exist.
    if jdata.key == COMMON_KEY.get().unwrap().to_string() || file_exist {
        // Remove file.
        let result = remove_file(full_path);

        // Check if file could be deleted.
        match result {
            Ok(()) => {
                // Construct JSON object.
                let data = json!(
                    {
                        "Code": 200,
                        "Info": "File has been deleted",
                        "File": format!("{}.env",jdata.name),
                        "Status": "OK"
                    }
                );

                // Get USER-AGENT from request header, ugly but works.
                let mut ua_string = String::new();
                for v in reqdata.headers().get_all(USER_AGENT) {
                    ua_string = format!("{:?}",v);
                };

                // Vec for HttpRequest data to log.
                // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
                let vlogdata = vec![
                    reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                    reqdata.connection_info().scheme().to_string(),
                    reqdata.path().to_string(),
                    reqdata.connection_info().host().to_string(),
                    ua_string
                ];

                // Get log function and put requierd data into it.
                let vlog: Vec<String> = logs::log_data(200,"Request OK",0,"DELETE",vlogdata);
                // Send information to log.
                let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={} oCMAPIDeleteID={} oCMAPIDeleteType=envfile",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9],jdata.name);
                let _ = logs::send_logs(logdata);

                // Fetch headers.
                let vheaders = api_headers();

                // Return answer.
                Ok( HttpResponse::Ok()
                    .append_header(("api-version",vheaders[0].clone()))
                    .content_type(vheaders[1].clone())
                    .json(data) )
            },
            Err(_result_err) => {
                // Construct JSON object
                let data = json!(
                    {
                        "Code": 400,
                        "Info": "File could not be deleted",
                        "Status": "Bad Request"
                    }
                );
            
                // Get USER-AGENT from request header, ugly but works.
                let mut ua_string = String::new();
                for v in reqdata.headers().get_all(USER_AGENT) {
                    ua_string = format!("{:?}",v);
                };
            
                // Vec for HttpRequest data to log.
                // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
                let vlogdata = vec![
                    reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                    reqdata.connection_info().scheme().to_string(),
                    reqdata.path().to_string(),
                    reqdata.connection_info().host().to_string(),
                    ua_string
                ];
            
                // Get log function and put requierd data into it.
                let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"DELETE",vlogdata);
                // Send information to log.
                let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
                let _ = logs::send_logs(logdata);
            
                // Fetch headers.
                let vheaders = api_headers();
            
                // Return answer.
                return Ok( HttpResponse::BadRequest()
                    .append_header(("api-version",vheaders[0].clone()))
                    .content_type(vheaders[1].clone())
                    .json(data) );
            }
        }
    }
    else {
        // Construct JSON object
        let data = json!(
            {
                "Code": 400,
                "Info": "Missing or incorrect data",
                "Status": "Bad Request"
            }
        );

        // Get USER-AGENT from request header, ugly but works.
        let mut ua_string = String::new();
        for v in reqdata.headers().get_all(USER_AGENT) {
            ua_string = format!("{:?}",v);
        };

        // Vec for HttpRequest data to log.
        // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
        let vlogdata = vec![
            reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
            reqdata.connection_info().scheme().to_string(),
            reqdata.path().to_string(),
            reqdata.connection_info().host().to_string(),
            ua_string
        ];

        // Get log function and put requierd data into it.
        let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"DELETE",vlogdata);
        // Send information to log.
        let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
        let _ = logs::send_logs(logdata);

        // Fetch headers.
        let vheaders = api_headers();

        // Return answer.
        return Ok( HttpResponse::BadRequest()
            .append_header(("api-version",vheaders[0].clone()))
            .content_type(vheaders[1].clone())
            .json(data) );
    }
}

// Delete secrets file
pub async fn delete_common_secfile(sdata: web::Json<DeleteSecFile>, reqdata: HttpRequest) -> io::Result<HttpResponse> {
    // Get JSON data from post.
    let jdata = sdata.into_inner();

    // Regex for non allowed characters.
    let rx_name = Regex::new(r"([^A-Za-z0-9_-])").unwrap();

    // Check if regex matches anything in name.
    if rx_name.find(jdata.name.as_str()).is_some() {
        // Construct JSON object
        let data = json!(
            {
                "Code": 400,
                "Info": "Name provided is not in correct format",
                "Status": "Bad Request"
            }
        );

        // Get USER-AGENT from request header, ugly but works.
        let mut ua_string = String::new();
        for v in reqdata.headers().get_all(USER_AGENT) {
            ua_string = format!("{:?}",v);
        };

        // Vec for HttpRequest data to log.
        // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
        let vlogdata = vec![
            reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
            reqdata.connection_info().scheme().to_string(),
            reqdata.path().to_string(),
            reqdata.connection_info().host().to_string(),
            ua_string
        ];

        // Get log function and put requierd data into it.
        let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"DELETE",vlogdata);
        // Send information to log.
        let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
        let _ = logs::send_logs(logdata);

        // Fetch headers.
        let vheaders = api_headers();

        // Return answer.
        return Ok( HttpResponse::BadRequest()
            .append_header(("api-version",vheaders[0].clone()))
            .content_type(vheaders[1].clone())
            .json(data) );
    }

    // Build complete path to file.
    let full_path = format!("{}/{}.tmp", SEC_PATH.get().unwrap(), jdata.name);

    // Get file exist status.
    let file_exist = Path::new(full_path.as_str()).exists();

    // Check if secret isn´t empty and not matching key from config.
    if jdata.key.is_empty() || jdata.key != COMMON_KEY.get().unwrap().to_string() {
        // Construct JSON object
        let data = json!(
            {
                "Code": 400,
                "Info": "Missing or incorrect data",
                "Status": "Bad Request"
            }
        );

        // Get USER-AGENT from request header, ugly but works.
        let mut ua_string = String::new();
        for v in reqdata.headers().get_all(USER_AGENT) {
            ua_string = format!("{:?}",v);
        };

        // Vec for HttpRequest data to log.
        // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
        let vlogdata = vec![
            reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
            reqdata.connection_info().scheme().to_string(),
            reqdata.path().to_string(),
            reqdata.connection_info().host().to_string(),
            ua_string
        ];

        // Get log function and put requierd data into it.
        let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"DELETE",vlogdata);
        // Send information to log.
        let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
        let _ = logs::send_logs(logdata);

        // Fetch headers.
        let vheaders = api_headers();

        // Return answer.
        return Ok( HttpResponse::BadRequest()
            .append_header(("api-version",vheaders[0].clone()))
            .content_type(vheaders[1].clone())
            .json(data) );
    }

    // Match key to make sure it is correct and see if file exist.
    if jdata.key == COMMON_KEY.get().unwrap().to_string() || file_exist {
        // Remove file.
        let result = remove_file(full_path);

        // Check if file could be deleted.
        match result {
            Ok(()) => {
                // Construct JSON object.
                let data = json!(
                    {
                        "Code": 200,
                        "Info": "File has been deleted",
                        "File": format!("{}.tmp",jdata.name),
                        "Status": "OK"
                    }
                );

                // Get USER-AGENT from request header, ugly but works.
                let mut ua_string = String::new();
                for v in reqdata.headers().get_all(USER_AGENT) {
                    ua_string = format!("{:?}",v);
                };

                // Vec for HttpRequest data to log.
                // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
                let vlogdata = vec![
                    reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                    reqdata.connection_info().scheme().to_string(),
                    reqdata.path().to_string(),
                    reqdata.connection_info().host().to_string(),
                    ua_string
                ];

                // Get log function and put requierd data into it.
                let vlog: Vec<String> = logs::log_data(200,"Request OK",0,"DELETE",vlogdata);
                // Send information to log.
                let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={} oCMAPIDeleteID={} oCMAPIDeleteType=secfile",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9],jdata.name);
                let _ = logs::send_logs(logdata);

                // Fetch headers.
                let vheaders = api_headers();

                // Return answer.
                Ok( HttpResponse::Ok()
                    .append_header(("api-version",vheaders[0].clone()))
                    .content_type(vheaders[1].clone())
                    .json(data) )
            },
            Err(_result_err) => {
                // Construct JSON object
                let data = json!(
                    {
                        "Code": 400,
                        "Info": "File could nbot be deleted",
                        "Status": "Bad Request"
                    }
                );
            
                // Get USER-AGENT from request header, ugly but works.
                let mut ua_string = String::new();
                for v in reqdata.headers().get_all(USER_AGENT) {
                    ua_string = format!("{:?}",v);
                };
            
                // Vec for HttpRequest data to log.
                // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
                let vlogdata = vec![
                    reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                    reqdata.connection_info().scheme().to_string(),
                    reqdata.path().to_string(),
                    reqdata.connection_info().host().to_string(),
                    ua_string
                ];
            
                // Get log function and put requierd data into it.
                let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"DELETE",vlogdata);
                // Send information to log.
                let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
                let _ = logs::send_logs(logdata);
            
                // Fetch headers.
                let vheaders = api_headers();
            
                // Return answer.
                return Ok( HttpResponse::BadRequest()
                    .append_header(("api-version",vheaders[0].clone()))
                    .content_type(vheaders[1].clone())
                    .json(data) );
            }
        }
    }
    else {
        // Construct JSON object
        let data = json!(
            {
                "Code": 400,
                "Info": "Missing or incorrect data",
                "Status": "Bad Request"
            }
        );

        // Get USER-AGENT from request header, ugly but works.
        let mut ua_string = String::new();
        for v in reqdata.headers().get_all(USER_AGENT) {
            ua_string = format!("{:?}",v);
        };

        // Vec for HttpRequest data to log.
        // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
        let vlogdata = vec![
            reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
            reqdata.connection_info().scheme().to_string(),
            reqdata.path().to_string(),
            reqdata.connection_info().host().to_string(),
            ua_string
        ];

        // Get log function and put requierd data into it.
        let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"DELETE",vlogdata);
        // Send information to log.
        let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
        let _ = logs::send_logs(logdata);

        // Fetch headers.
        let vheaders = api_headers();

        // Return answer.
        return Ok( HttpResponse::BadRequest()
            .append_header(("api-version",vheaders[0].clone()))
            .content_type(vheaders[1].clone())
            .json(data) );
    }
}

// Delete unused images
pub async fn delete_common_images(sdata: web::Json<DeleteImages>, reqdata: HttpRequest) -> io::Result<HttpResponse> {
    // Get JSON data from post.
    let jdata = sdata.into_inner();

    // Match key to make sure it is correct.
    if jdata.key == COMMON_KEY.get().unwrap().to_string() {

        // Prune images.
        let cmd = Command::new("podman")
            .arg("image")
            .arg("prune")
            .arg("--all")
            .arg("--force")
            .output()?;

        // Check exit code to see if prune command went OK or not.
        // Exit codes: 0 = OK, all else not OK.
        match cmd.status.code() {
            Some(0) => {
                // Construct JSON object.
                let data = json!(
                    {
                        "Code": 200,
                        "Name": "Unused images has been deleted",
                        "Status": "OK"
                    }
                );

                // Get USER-AGENT from request header, ugly but works.
                let mut ua_string = String::new();
                for v in reqdata.headers().get_all(USER_AGENT) {
                    ua_string = format!("{:?}",v);
                };
                // Vec for HttpRequest data to log.
                // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
                let vlogdata = vec![
                    reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                    reqdata.connection_info().scheme().to_string(),
                    reqdata.path().to_string(),
                    reqdata.connection_info().host().to_string(),
                    ua_string
                ];

                // Get log function and put requierd data into it.
                let vlog: Vec<String> = logs::log_data(200,"Request OK",0,"DELETE",vlogdata);
                // Send information to log.
                let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={} oCMAPIDeleteID={} oCMAPIDeleteType=PruneImages",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9],"0");
                let _ = logs::send_logs(logdata);

                // Fetch headers.
                let vheaders = api_headers();

                // Return answer.
                Ok( HttpResponse::Ok()
                    .append_header(("api-version",vheaders[0].clone()))
                    .content_type(vheaders[1].clone())
                    .json(data) )
            },
            _ => {
                // Construct JSON object
                let data = json!(
                    {
                        "Code": 400,
                        "Info": "Could not prune images",
                        "Status": "Bad Request"
                    }
                );

                // Get USER-AGENT from request header, ugly but works.
                let mut ua_string = String::new();
                for v in reqdata.headers().get_all(USER_AGENT) {
                    ua_string = format!("{:?}",v);
                };

                // Vec for HttpRequest data to log.
                // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
                let vlogdata = vec![
                    reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                    reqdata.connection_info().scheme().to_string(),
                    reqdata.path().to_string(),
                    reqdata.connection_info().host().to_string(),
                    ua_string
                ];

                // Get log function and put requierd data into it.
                let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"DELETE",vlogdata);
                // Send information to log.
                let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
                let _ = logs::send_logs(logdata);

                // Fetch headers.
                let vheaders = api_headers();

                // Return answer.
                return Ok( HttpResponse::BadRequest()
                    .append_header(("api-version",vheaders[0].clone()))
                    .content_type(vheaders[1].clone())
                    .json(data) );
            }
        }
    }
    else {
        // Construct JSON object
        let data = json!(
            {
                "Code": 400,
                "Info": "Missing or incorrect data",
                "Status": "Bad Request"
            }
        );

        // Get USER-AGENT from request header, ugly but works.
        let mut ua_string = String::new();
        for v in reqdata.headers().get_all(USER_AGENT) {
            ua_string = format!("{:?}",v);
        };

        // Vec for HttpRequest data to log.
        // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
        let vlogdata = vec![
            reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
            reqdata.connection_info().scheme().to_string(),
            reqdata.path().to_string(),
            reqdata.connection_info().host().to_string(),
            ua_string
        ];

        // Get log function and put requierd data into it.
        let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"DELETE",vlogdata);
        // Send information to log.
        let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
        let _ = logs::send_logs(logdata);

        // Fetch headers.
        let vheaders = api_headers();

        // Return answer.
        return Ok( HttpResponse::BadRequest()
            .append_header(("api-version",vheaders[0].clone()))
            .content_type(vheaders[1].clone())
            .json(data) );
    }
}

/* --- Containers API --- */

// Get all containers status information.
pub async fn get_containers_state(reqdata: HttpRequest) -> io::Result<HttpResponse> {
    // The command.
    let cmd = Command::new("podman")
        .arg("ps")
        .arg("--all")
        .arg(
            "--format='
            { \"Names\": {{json .Names}},
            \"State\": {{json .State}},
            \"Status\": {{json .Status}},
            \"Exited\": {{json .Exited}},
            \"ExitCode\": {{json .ExitCode}} },'"
        )
        .output();

    // Check if command went ok or not.
    match cmd {
        // When Ok build response.
        Ok(cmd_ok) => {
            // Clean data from unneeded characters.
            let data = format!("[{}]", String::from_utf8_lossy(&cmd_ok.stdout))
                .trim_start()
                .trim_end()
                .to_string()
                .replace("'", "")
                .replace(",\n]", "]");

            // Get USER-AGENT from request header, ugly but works.
            let mut ua_string = String::new();
            for v in reqdata.headers().get_all(USER_AGENT) {
                ua_string = format!("{:?}",v);
            };

            // Vec for HttpRequest data to log.
            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
            let vlogdata = vec![
                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                reqdata.connection_info().scheme().to_string(),
                reqdata.path().to_string(),
                reqdata.connection_info().host().to_string(),
                ua_string
            ];

            // Get log function and put requierd data into it.
            let vlog: Vec<String> = logs::log_data(200,"Request OK",0,"GET",vlogdata);
            // Send information to log.
            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
            let _ = logs::send_logs(logdata);

            // Fetch headers.
            let vheaders = api_headers();

            // Return answer.
            Ok( HttpResponse::Ok()
                .append_header(("api-version",vheaders[0].clone()))
                .content_type(vheaders[1].clone())
                .body(data) )
        },
        // When error build response.
        Err(cmd_err) => {
            // Construct JSON object
            let data = json!(
                {
                    "Code": 400,
                    "Info": format!("{}", cmd_err),
                    "Status": "Bad Request"
                }
            );

            // Get USER-AGENT from request header, ugly but works.
            let mut ua_string = String::new();
            for v in reqdata.headers().get_all(USER_AGENT) {
                ua_string = format!("{:?}",v);
            };

            // Vec for HttpRequest data to log.
            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
            let vlogdata = vec![
                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                reqdata.connection_info().scheme().to_string(),
                reqdata.path().to_string(),
                reqdata.connection_info().host().to_string(),
                ua_string
            ];

            // Get log function and put requierd data into it.
            let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"GET",vlogdata);
            // Send information to log.
            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
            let _ = logs::send_logs(logdata);

            // Fetch headers.
            let vheaders = api_headers();

            // Return answer.
            Ok( HttpResponse::BadRequest()
                .append_header(("api-version",vheaders[0].clone()))
                .content_type(vheaders[1].clone())
                .json(data) )
        }
    }
}

// Get containers with a specific state.
pub async fn get_containers_state_query(param: web::Path<String>,reqdata: HttpRequest) -> io::Result<HttpResponse> {
    // Create arg_input variable.
    let arg_input;
    // Check param from input, must match.
    match param.to_lowercase().as_str() {
        "created" => { arg_input = format!("status={}", param.to_lowercase()); },
        "exited" => { arg_input = format!("status={}", param.to_lowercase()); },
        "paused" => { arg_input = format!("status={}", param.to_lowercase()); },
        "running" => { arg_input = format!("status={}", param.to_lowercase()); },
        "unknown" => { arg_input = format!("status={}", param.to_lowercase()); },
        _ => { arg_input = format!("status={}", "ERROR"); },
    }

    // Check if ERROR based on input param assignment.
    if arg_input == "status=ERROR" {
        // Construct JSON object
        let data = json!(
            {
                "Code": 400,
                "Info": "Not a valid status option",
                "Status": "Bad Request"
            }
        );
        
        // Get USER-AGENT from request header, ugly but works.
        let mut ua_string = String::new();
        for v in reqdata.headers().get_all(USER_AGENT) {
            ua_string = format!("{:?}",v);
        };
        // Vec for HttpRequest data to log.
        // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
        let vlogdata = vec![
            reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
            reqdata.connection_info().scheme().to_string(),
            reqdata.path().to_string(),
            reqdata.connection_info().host().to_string(),
            ua_string
        ];
        // Get log function and put requierd data into it.
        let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"GET",vlogdata);
        // Send information to log.
        let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
        let _ = logs::send_logs(logdata);

        // Fetch headers.
        let vheaders = api_headers();

        // Return answer.
        return Ok( HttpResponse::BadRequest()
            .append_header(("api-version",vheaders[0].clone()))
            .content_type(vheaders[1].clone())
            .json(data) );
    }

    // The command.
    let cmd = Command::new("podman")
        .arg("container")
        .arg("ps")
        .arg("--filter")
        .arg(arg_input.clone())
        .arg(
            "--format='
            { \"Names\": {{json .Names}},
            \"ID\": {{json .ID}},
            \"State\": {{json .State}},
            \"Status\": {{json .Status}},
            \"Exited\": {{json .Exited}},
            \"ExitCode\": {{json .ExitCode}} },'"
        )
        .output();

    // Check if command went ok or not.
    match cmd {
        Ok(cmd_ok) => {
            // Check length of stdout, returns empty response when no containers are running.
            let count = cmd_ok.stdout.len();
            if count > 4 {
                // Clean data from unneeded characters.
                let data = format!("[{}]", String::from_utf8_lossy(&cmd_ok.stdout))
                    .trim_start()
                    .trim_end()
                    .to_string()
                    .replace("'", "")
                    .replace(",\n]", "]");
                // Get USER-AGENT from request header, ugly but works.
                let mut ua_string = String::new();
                for v in reqdata.headers().get_all(USER_AGENT) {
                    ua_string = format!("{:?}",v);
                };

                // Vec for HttpRequest data to log.
                // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
                let vlogdata = vec![
                    reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                    reqdata.connection_info().scheme().to_string(),
                    reqdata.path().to_string(),
                    reqdata.connection_info().host().to_string(),
                    ua_string
                ];
                // Get log function and put requierd data into it.
                let vlog: Vec<String> = logs::log_data(200,"Request OK",0,"GET",vlogdata);
                // Send information to log.
                let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
                let _ = logs::send_logs(logdata);

                // Fetch headers.
                let vheaders = api_headers();

                // Return answer.
                Ok( HttpResponse::Ok()
                    .append_header(("api-version",vheaders[0].clone()))
                    .content_type(vheaders[1].clone())
                    .body(data) )
            }
            // Return error since no containers are running.
            else {
                // Construct JSON object
                let data = json!(
                    [
                        {
                            "Code": 404,
                            "Info": format!("No containers matching query status: {}",arg_input.replace("status=", "")),
                            "Status": "None"
                        }
                    ]
                );

                // Get USER-AGENT from request header, ugly but works.
                let mut ua_string = String::new();
                for v in reqdata.headers().get_all(USER_AGENT) {
                    ua_string = format!("{:?}",v);
                };
                // Vec for HttpRequest data to log.
                // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
                let vlogdata = vec![
                    reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                    reqdata.connection_info().scheme().to_string(),
                    reqdata.path().to_string(),
                    reqdata.connection_info().host().to_string(),
                    ua_string
                ];
                // Get log function and put requierd data into it.
                let vlog: Vec<String> = logs::log_data(404,"Not Found",0,"GET",vlogdata);
                // Send information to log.
                let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
                let _ = logs::send_logs(logdata);

                // Fetch headers.
                let vheaders = api_headers();

                // Return answer.
                Ok( HttpResponse::NotFound()
                    .append_header(("api-version",vheaders[0].clone()))
                    .content_type(vheaders[1].clone())
                    .json(data) )
            }
        }
        Err(cmd_err) => {
            // Construct JSON object
            let data = json!(
                {
                    "Code": 500,
                    "Info": format!("{}", cmd_err),
                    "Status": "Internal Server Error"
                }
            );

            // Get USER-AGENT from request header, ugly but works.
            let mut ua_string = String::new();
            for v in reqdata.headers().get_all(USER_AGENT) {
                ua_string = format!("{:?}",v);
            };
            // Vec for HttpRequest data to log.
            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
            let vlogdata = vec![
                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                reqdata.connection_info().scheme().to_string(),
                reqdata.path().to_string(),
                reqdata.connection_info().host().to_string(),
                ua_string
            ];

            // Get log function and put requierd data into it.
            let vlog: Vec<String> = logs::log_data(500,"Internal Server Error",6,"GET",vlogdata);
            // Send information to log.
            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
            let _ = logs::send_logs(logdata);

            // Fetch headers.
            let vheaders = api_headers();

            // Return answer.
            Ok( HttpResponse::InternalServerError()
                .append_header(("api-version",vheaders[0].clone()))
                .content_type(vheaders[1].clone())
                .json(data) )
        }
    }
}

// Get state of a single container.
pub async fn get_containers_single_state_query(param: web::Path<String>,reqdata: HttpRequest) -> io::Result<HttpResponse> {
    // Get input data from post.
    let indata= param;

    // Regex for non allowed characters.
    let rx_nameid = Regex::new(r"([^A-Za-z0-9_-])").unwrap();

    // Check if regex matches anything in name.
    if indata.is_empty() || rx_nameid.find(&&indata.as_str()).is_some() {
        // Construct JSON object
        let data = json!(
            {
                "Code": 400,
                "Info": "Information provided is not in correct format",
                "Status": "Bad Request"
            }
        );

        // Get USER-AGENT from request header, ugly but works.
        let mut ua_string = String::new();
        for v in reqdata.headers().get_all(USER_AGENT) {
            ua_string = format!("{:?}",v);
        };
        // Vec for HttpRequest data to log.
        // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
        let vlogdata = vec![
            reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
            reqdata.connection_info().scheme().to_string(),
            reqdata.path().to_string(),
            reqdata.connection_info().host().to_string(),
            ua_string
        ];

        // Get log function and put requierd data into it.
        let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"POST",vlogdata);
        // Send information to log.
        let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
        let _ = logs::send_logs(logdata);

        // Fetch headers.
        let vheaders = api_headers();

        // Return answer.
        return Ok( HttpResponse::BadRequest()
            .append_header(("api-version",vheaders[0].clone()))
            .content_type(vheaders[1].clone())
            .json(data) );
    }

    // The command for getting via name.
    let cmd_name = Command::new("podman")
        .arg("container")
        .arg("ps")
        .arg("--all")
        .arg("--filter")
        .arg(format!("name={}",indata.clone()))
        .arg(
            "--format='
            { \"Name\": {{json .Names}},
            \"ID\": {{json .ID}},
            \"State\": {{json .State}},
            \"Status\": {{json .Status}},
            \"Exited\": {{json .Exited}},
            \"ExitCode\": {{json .ExitCode}} }'"
        )
        .output();

    // Check if command went ok or not.
    match cmd_name {
        Ok(cmd_name_ok) => {
            // Check length of stdout, returns empty response when no match occured.
            let name_count = cmd_name_ok.stdout.len();
            if name_count > 4 {
                // Clean data from unneeded characters.
                let data = format!("{}", String::from_utf8_lossy(&cmd_name_ok.stdout))
                    .trim_start()
                    .trim_end()
                    .to_string()
                    .replace("'", "")
                    .replace(",\n]", "]");

                // Get USER-AGENT from request header, ugly but works.
                let mut ua_string = String::new();
                for v in reqdata.headers().get_all(USER_AGENT) {
                    ua_string = format!("{:?}",v);
                };

                // Vec for HttpRequest data to log.
                // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
                let vlogdata = vec![
                    reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                    reqdata.connection_info().scheme().to_string(),
                    reqdata.path().to_string(),
                    reqdata.connection_info().host().to_string(),
                    ua_string
                ];
                // Get log function and put requierd data into it.
                let vlog: Vec<String> = logs::log_data(200,"Request OK",0,"GET",vlogdata);
                // Send information to log.
                let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
                let _ = logs::send_logs(logdata);

                // Fetch headers.
                let vheaders = api_headers();

                // Return answer.
                return Ok( HttpResponse::Ok()
                    .append_header(("api-version",vheaders[0].clone()))
                    .content_type(vheaders[1].clone())
                    .body(data) );
            }
            // Check if ID was given.
            else {
                // The command for getting via name.
                let cmd_id = Command::new("podman")
                    .arg("container")
                    .arg("ps")
                    .arg("--all")
                    .arg("--filter")
                    .arg(format!("id={}",indata.clone()))
                    .arg(
                        "--format='
                        { \"Name\": {{json .Names}},
                        \"ID\": {{json .ID}},
                        \"State\": {{json .State}},
                        \"Status\": {{json .Status}},
                        \"Exited\": {{json .Exited}},
                        \"ExitCode\": {{json .ExitCode}} }'"
                    )
                    .output();

                // Check if command went ok or not.
                match cmd_id {
                    Ok(cmd_id_ok) => {
                        // Check length of stdout, returns empty response when no match occured.
                        let id_count = cmd_id_ok.stdout.len();
                        if id_count > 4 {
                            // Clean data from unneeded characters.
                            let data = format!("{}", String::from_utf8_lossy(&cmd_id_ok.stdout))
                                .trim_start()
                                .trim_end()
                                .to_string()
                                .replace("'", "")
                                .replace(",\n]", "]");
                            // Get USER-AGENT from request header, ugly but works.
                            let mut ua_string = String::new();
                            for v in reqdata.headers().get_all(USER_AGENT) {
                                ua_string = format!("{:?}",v);
                            };

                            // Vec for HttpRequest data to log.
                            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
                            let vlogdata = vec![
                                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                                reqdata.connection_info().scheme().to_string(),
                                reqdata.path().to_string(),
                                reqdata.connection_info().host().to_string(),
                                ua_string
                            ];
                            // Get log function and put requierd data into it.
                            let vlog: Vec<String> = logs::log_data(200,"Request OK",0,"GET",vlogdata);
                            // Send information to log.
                            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
                            let _ = logs::send_logs(logdata);

                            // Fetch headers.
                            let vheaders = api_headers();

                            // Return answer.
                            return Ok( HttpResponse::Ok()
                                .append_header(("api-version",vheaders[0].clone()))
                                .content_type(vheaders[1].clone())
                                .body(data) );
                        }
                        else {

                            // Construct JSON object
                            let data = json!(
                                [
                                    {
                                        "Code": 404,
                                        "Info": format!("No containers matching name or ID: {}",indata),
                                        "Status": "Not Found"
                                    }
                                ]
                            );
                        
                            // Get USER-AGENT from request header, ugly but works.
                            let mut ua_string = String::new();
                            for v in reqdata.headers().get_all(USER_AGENT) {
                                ua_string = format!("{:?}",v);
                            };
                            // Vec for HttpRequest data to log.
                            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
                            let vlogdata = vec![
                                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                                reqdata.connection_info().scheme().to_string(),
                                reqdata.path().to_string(),
                                reqdata.connection_info().host().to_string(),
                                ua_string
                            ];
                            // Get log function and put requierd data into it.
                            let vlog: Vec<String> = logs::log_data(404,"Not Found",0,"GET",vlogdata);
                            // Send information to log.
                            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
                            let _ = logs::send_logs(logdata);
                        
                            // Fetch headers.
                            let vheaders = api_headers();
                        
                            // Return answer.
                            return Ok( HttpResponse::NotFound()
                                .append_header(("api-version",vheaders[0].clone()))
                                .content_type(vheaders[1].clone())
                                .json(data) );
                        }
                    }
                    Err(cmd_id_err) => {
                        // Construct JSON object
                        let data = json!(
                            {
                                "Code": 500,
                                "Info": format!("{}", cmd_id_err),
                                "Status": "Internal Server Error"
                            }
                        );
                    
                        // Get USER-AGENT from request header, ugly but works.
                        let mut ua_string = String::new();
                        for v in reqdata.headers().get_all(USER_AGENT) {
                            ua_string = format!("{:?}",v);
                        };
                        // Vec for HttpRequest data to log.
                        // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
                        let vlogdata = vec![
                            reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                            reqdata.connection_info().scheme().to_string(),
                            reqdata.path().to_string(),
                            reqdata.connection_info().host().to_string(),
                            ua_string
                        ];
                    
                        // Get log function and put requierd data into it.
                        let vlog: Vec<String> = logs::log_data(500,"Internal Server Error",6,"GET",vlogdata);
                        // Send information to log.
                        let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
                        let _ = logs::send_logs(logdata);
                    
                        // Fetch headers.
                        let vheaders = api_headers();
                    
                        // Return answer.
                        return Ok( HttpResponse::InternalServerError()
                            .append_header(("api-version",vheaders[0].clone()))
                            .content_type(vheaders[1].clone())
                            .json(data) );
                    }
                }
            }
        }
        Err(cmd_name_err) => {
            // Construct JSON object
            let data = json!(
                {
                    "Code": 500,
                    "Info": format!("{}", cmd_name_err),
                    "Status": "Internal Server Error"
                }
            );

            // Get USER-AGENT from request header, ugly but works.
            let mut ua_string = String::new();
            for v in reqdata.headers().get_all(USER_AGENT) {
                ua_string = format!("{:?}",v);
            };
            // Vec for HttpRequest data to log.
            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
            let vlogdata = vec![
                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                reqdata.connection_info().scheme().to_string(),
                reqdata.path().to_string(),
                reqdata.connection_info().host().to_string(),
                ua_string
            ];

            // Get log function and put requierd data into it.
            let vlog: Vec<String> = logs::log_data(500,"Internal Server Error",6,"GET",vlogdata);
            // Send information to log.
            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
            let _ = logs::send_logs(logdata);

            // Fetch headers.
            let vheaders = api_headers();

            // Return answer.
            Ok( HttpResponse::InternalServerError()
                .append_header(("api-version",vheaders[0].clone()))
                .content_type(vheaders[1].clone())
                .json(data) )
        }
    }
}

// Get Names or ID of a all containers.
pub async fn get_containers_short_list(param: web::Path<String>,reqdata: HttpRequest) -> io::Result<HttpResponse> {
    // Create arg_input variable.
    let arg_input;

    // Check param from input, must match.
    match param.to_lowercase().as_str() {
        "name" => { arg_input = "Names" },
        "id" => { arg_input = "ID" },
        _ => { arg_input = "ERROR" },
    }

    // If incorrect value provided error out.
    if arg_input == "ERROR" {
        // Construct JSON object
        let data = json!(
            {
                "Code": 400,
                "Info": "Not a valid list option",
                "Status": "Bad Request"
            }
        );

        // Get USER-AGENT from request header, ugly but works.
        let mut ua_string = String::new();
        for v in reqdata.headers().get_all(USER_AGENT) {
            ua_string = format!("{:?}",v);
        };
        // Vec for HttpRequest data to log.
        // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
        let vlogdata = vec![
            reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
            reqdata.connection_info().scheme().to_string(),
            reqdata.path().to_string(),
            reqdata.connection_info().host().to_string(),
            ua_string
        ];

        // Get log function and put requierd data into it.
        let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"POST",vlogdata);
        // Send information to log.
        let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
        let _ = logs::send_logs(logdata);

        // Fetch headers.
        let vheaders = api_headers();

        // Return answer.
        return Ok( HttpResponse::BadRequest()
            .append_header(("api-version",vheaders[0].clone()))
            .content_type(vheaders[1].clone())
            .json(data) );
    }

    // Get infra containers for exclusions.
    let cmd_infra = Command::new("podman")
        .arg("container")
        .arg("ps")
        .arg("--all")
        .arg("--filter")
        .arg("name=infra")
        .arg(format!("--format={{{{ .{} }}}}", arg_input))
        .output();

    // Create empty infra array.
    let mut vecinfra: Vec<String> = Vec::new();

    // Execute command and continue.
    match cmd_infra {
        Ok(cmd_infra_ok) => {
            // Convert stdout to string.
            let mut infra_list = format!("{:?}", String::from_utf8_lossy(&cmd_infra_ok.stdout)).replace('"', "").replace("\\n", ",");

            // Remove last character from string.
            infra_list.pop();

            // Fill array
            for v in infra_list.split(",").map(String::from) {
                // Fill vec.
                vecinfra.push(v.to_string());
            }
        }
        Err(cmd_infra_err) => {
            // Construct JSON object
            let data = json!(
                {
                    "Code": 400,
                    "Info": format!("{}", cmd_infra_err),
                    "Status": "Bad Request"
                }
            );

            // Get USER-AGENT from request header, ugly but works.
            let mut ua_string = String::new();
            for v in reqdata.headers().get_all(USER_AGENT) {
                ua_string = format!("{:?}",v);
            };

            // Vec for HttpRequest data to log.
            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
            let vlogdata = vec![
                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                reqdata.connection_info().scheme().to_string(),
                reqdata.path().to_string(),
                reqdata.connection_info().host().to_string(),
                ua_string
            ];

            // Get log function and put requierd data into it.
            let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"GET",vlogdata);
            // Send information to log.
            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
            let _ = logs::send_logs(logdata);

            // Fetch headers.
            let vheaders = api_headers();

            // Return answer.
            return Ok( HttpResponse::InternalServerError()
                .append_header(("api-version",vheaders[0].clone()))
                .content_type(vheaders[1].clone())
                .json(data) );
        }
    }

    // The command.
    let cmd = Command::new("podman")
        .arg("container")
        .arg("ps")
        .arg("--all")
        .arg(format!("--format={{{{ .{} }}}}", arg_input))
        .output();

    // Check if command went ok or not.
    match cmd {
        Ok(cmd_ok) => {
            // Convert stdout to string.
            let mut cnt_list = format!("{:?}", String::from_utf8_lossy(&cmd_ok.stdout)).replace('"', "").replace("\\n", ",");

            // Remove last character from string.
            cnt_list.pop();

            // Create empty array
            let mut vecdata: Vec<String> = Vec::new();

            // Fill array
            for v in cnt_list.split(",").map(String::from) {
                // Fill vec.
                vecdata.push(v.to_string());
            }

            // Remove infra containers from result.
            for infra in vecinfra.clone() {
                vecdata.retain(|value| *value != infra);
            }

            // Build result.
            let data = format!("{{\"{}\":{:?}}}", arg_input, &vecdata)
                .trim_start()
                .trim_end()
                .to_string();

            // Get USER-AGENT from request header, ugly but works.
            let mut ua_string = String::new();
            for v in reqdata.headers().get_all(USER_AGENT) {
                ua_string = format!("{:?}",v);
            };

            // Vec for HttpRequest data to log.
            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
            let vlogdata = vec![
                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                reqdata.connection_info().scheme().to_string(),
                reqdata.path().to_string(),
                reqdata.connection_info().host().to_string(),
                ua_string
            ];

            // Get log function and put requierd data into it.
            let vlog: Vec<String> = logs::log_data(200,"Request OK",0,"GET",vlogdata);
            // Send information to log.
            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
            let _ = logs::send_logs(logdata);

            // Fetch headers.
            let vheaders = api_headers();

            // Return answer.
            return Ok( HttpResponse::Ok()
                .append_header(("api-version",vheaders[0].clone()))
                .content_type(vheaders[1].clone())
                .body(data) );
        }
        Err(cmd_err) => {
            // Construct JSON object
            let data = json!(
                {
                    "Code": 500,
                    "Info": format!("{}", cmd_err),
                    "Status": "Internal Server Error"
                }
            );

            // Get USER-AGENT from request header, ugly but works.
            let mut ua_string = String::new();
            for v in reqdata.headers().get_all(USER_AGENT) {
                ua_string = format!("{:?}",v);
            };

            // Vec for HttpRequest data to log.
            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
            let vlogdata = vec![
                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                reqdata.connection_info().scheme().to_string(),
                reqdata.path().to_string(),
                reqdata.connection_info().host().to_string(),
                ua_string
            ];

            // Get log function and put requierd data into it.
            let vlog: Vec<String> = logs::log_data(500,"Internal Server Error",6,"GET",vlogdata);
            // Send information to log.
            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
            let _ = logs::send_logs(logdata);

            // Fetch headers.
            let vheaders = api_headers();

            // Return answer.
            Ok( HttpResponse::InternalServerError()
                .append_header(("api-version",vheaders[0].clone()))
                .content_type(vheaders[1].clone())
                .json(data) )
        }
    }
}

// Post containers create data.
pub async fn post_containers_create(cdata: web::Json<CreateContainer>,reqdata: HttpRequest) -> io::Result<HttpResponse> {
    // Get JSON data from post.
    let jdata = cdata.into_inner();

    // Create empty array
    let mut vecdata: Vec<String> = Vec::new();

    // Check if required json tags has values.
    if jdata.name.len() > 2 && jdata.image.len() > 5 {
        // Fill vec.
        vecdata.push("container".to_string());
        vecdata.push("create".to_string());
        vecdata.push(format!("--name={}",jdata.name));

        // Check that options are constructed right.
        if jdata.options.len() > 4 && jdata.options.starts_with("--") {
            // iterate through options tag and split back to vec.
            for v in jdata.options.split(",-").map(String::from) {
                if v.starts_with("--") {
                    vecdata.push(v)
                }
                else {
                    vecdata.push(format!("-{}", v))
                }
            }
        }
        // if requirements is in wrong format, error out.
        else if !jdata.options.is_empty() {
            // Construct JSON object
            let data = json!(
                {
                    "Code": 400,
                    "Info": "Check options tag, must be at least 4 characters long and start with -- to be valid",
                    "Status": "Bad Request"
                }
            );

            // Get USER-AGENT from request header, ugly but works.
            let mut ua_string = String::new();
            for v in reqdata.headers().get_all(USER_AGENT) {
                ua_string = format!("{:?}",v);
            };

            // Vec for HttpRequest data to log.
            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
            let vlogdata = vec![
                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                reqdata.connection_info().scheme().to_string(),
                reqdata.path().to_string(),
                reqdata.connection_info().host().to_string(),
                ua_string
            ];

            // Get log function and put requierd data into it.
            let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"POST",vlogdata);
            // Send information to log.
            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
            let _ = logs::send_logs(logdata);

            // Fetch headers.
            let vheaders = api_headers();

            // Return answer.
            return Ok(HttpResponse::BadRequest()
                .append_header(("api-version",vheaders[0].clone()))
                .content_type(vheaders[1].clone())
                .json(data));
        }

        //  Const with bad options.
        const BAD_OPTIONS: [&str; 5] = ["--help", "--tty", "--attach", "--quiet", "--rm"];
        // Remove the bad options.
        for bad in BAD_OPTIONS {
            vecdata.retain(|value| *value != bad);
        }

        // Add image to vec.
        vecdata.push(jdata.image);
    }
    // if requirements are not met, error out.
    else {
        // Construct JSON object
        let data = json!(
            {
                "Code": 400,
                "Info": "Required tags are missing information",
                "Status": "Bad Request"
            }
        );

        // Get USER-AGENT from request header, ugly but works.
        let mut ua_string = String::new();
        for v in reqdata.headers().get_all(USER_AGENT) {
            ua_string = format!("{:?}",v);
        };

        // Vec for HttpRequest data to log.
        // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
        let vlogdata = vec![
            reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
            reqdata.connection_info().scheme().to_string(),
            reqdata.path().to_string(),
            reqdata.connection_info().host().to_string(),
            ua_string
        ];

        // Get log function and put requierd data into it.
        let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"POST",vlogdata);
        // Send information to log.
        let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
        let _ = logs::send_logs(logdata);

        // Fetch headers.
        let vheaders = api_headers();

        // Return answer.
        return Ok(HttpResponse::BadRequest()
            .append_header(("api-version",vheaders[0].clone()))
            .content_type(vheaders[1].clone())
            .json(data));
    }

    // Build the command.
    let mut cmd = Command::new("podman");
    for v in vecdata.iter() {
        cmd.arg(v);
    }

    // Check if command went ok or not.
    match cmd.output() {
        Ok(cmd_ok) => {
            // check if info has been return on stderr, command do not generate error code correctly.
            if !cmd_ok.stdout.is_empty() {
                // Build answer.
                let data = json!(
                    {
                        "Code": "201",
                        "Info": "Successfully created container",
                        "ID": format!("{}", String::from_utf8_lossy(&cmd_ok.stdout).trim_end()),
                        "Status": "Created"
                    }
                );

                // Get USER-AGENT from request header, ugly but works.
                let mut ua_string = String::new();
                for v in reqdata.headers().get_all(USER_AGENT) {
                    ua_string = format!("{:?}",v);
                };

                // Vec for HttpRequest data to log.
                // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
                let vlogdata = vec![
                    reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                    reqdata.connection_info().scheme().to_string(),
                    reqdata.path().to_string(),
                    reqdata.connection_info().host().to_string(),
                    ua_string
                ];

                // Get log function and put requierd data into it.
                let vlog: Vec<String> = logs::log_data(201,"Created",0,"POST",vlogdata);
                // Extra create data for log.
                let cdata = format!("{:?}",String::from_utf8_lossy(&cmd_ok.stdout).trim_end());
                // Send information to log.
                let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={} oCMAPICreateID={} oCMAPICreateType=Container",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9],cdata);
                let _ = logs::send_logs(logdata);

                // Fetch headers.
                let vheaders = api_headers();

                // Return answer.
                Ok( HttpResponse::Created()
                    .append_header(("api-version",vheaders[0].clone()))
                    .content_type(vheaders[1].clone())
                    .json(data) )
            }
            else {
                // Construct JSON object
                let data = json!(
                    {
                        "Code": 400,
                        "Info": format!("{}", String::from_utf8_lossy(&cmd_ok.stderr).trim_end().replace("\n", ", ")),
                        "Status": "Bad Request"
                    }
                );

                // Get USER-AGENT from request header, ugly but works.
                let mut ua_string = String::new();
                for v in reqdata.headers().get_all(USER_AGENT) {
                    ua_string = format!("{:?}",v);
                };

                // Vec for HttpRequest data to log.
                // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
                let vlogdata = vec![
                    reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                    reqdata.connection_info().scheme().to_string(),
                    reqdata.path().to_string(),
                    reqdata.connection_info().host().to_string(),
                    ua_string
                ];

                // Get log function and put requierd data into it.
                let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"POST",vlogdata);
                // Send information to log.
                let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
                let _ = logs::send_logs(logdata);

                // Fetch headers.
                let vheaders = api_headers();

                // Return answer.
                Ok( HttpResponse::BadRequest()
                    .append_header(("api-version",vheaders[0].clone()))
                    .content_type(vheaders[1].clone())
                    .json(data) )
            }
        },
        // When error build response.
        Err(cmd_err) => {
            // Construct JSON object
            let data = json!(
                {
                    "Code": 500,
                    "Info": format!("{}", cmd_err).trim_end(),
                    "Status": "Internal Server Error"
                }
            );

            // Get USER-AGENT from request header, ugly but works.
            let mut ua_string = String::new();
            for v in reqdata.headers().get_all(USER_AGENT) {
                ua_string = format!("{:?}",v);
            };

            // Vec for HttpRequest data to log.
            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
            let vlogdata = vec![
                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                reqdata.connection_info().scheme().to_string(),
                reqdata.path().to_string(),
                reqdata.connection_info().host().to_string(),
                ua_string
            ];

            // Get log function and put requierd data into it.
            let vlog: Vec<String> = logs::log_data(500,"Internal Server Error",6,"POST",vlogdata);
            // Send information to log.
            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
            let _ = logs::send_logs(logdata);

            // Fetch headers.
            let vheaders = api_headers();

            // Return answer.
            Ok( HttpResponse::InternalServerError()
                .append_header(("api-version",vheaders[0].clone()))
                .content_type(vheaders[1].clone())
                .json(data) )
        }
    }
}

// Post containers setstate data.
pub async fn post_containers_setstate(sdata: web::Json<StateContainer>,reqdata: HttpRequest) -> io::Result<HttpResponse> {
    // Get JSON data from post.
    let jdata = sdata.into_inner();

    // Check and block invalid name format and options.
    let name_input: String = match jdata.name.as_str() {
        s if s.starts_with("--") => { "ERROR".to_string() },
        s if s.starts_with("-") => { "ERROR".to_string() },
        s if s.contains(" ") => { "ERROR".to_string() },
        "" => { "ERROR".to_string() },
        _ => { jdata.state.to_string() }
    };

    // Check state from input, must match given or get ERROR as result.
    let arg_input: String = match jdata.state.to_lowercase().as_str() {
        "start" => { jdata.state.to_string() },
        "stop" => { jdata.state.to_string() },
        "pause" => { jdata.state.to_string() },
        "unpause" => { jdata.state.to_string() },
        "restart" => { jdata.state.to_string() },
        _ => { "ERROR".to_string() }
    };

    // Check if ERROR is assigned based on input param given.
    if arg_input == "ERROR" || name_input == "ERROR" {
        // Construct JSON object
        let data = json!(
            {
                "Code": 400,
                "Info": "State/Name provided is not in correct format or a valid state",
                "Status": "Bad Request"
            }
        );

        // Get USER-AGENT from request header, ugly but works.
        let mut ua_string = String::new();
        for v in reqdata.headers().get_all(USER_AGENT) {
            ua_string = format!("{:?}",v);
        };

        // Vec for HttpRequest data to log.
        // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
        let vlogdata = vec![
            reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
            reqdata.connection_info().scheme().to_string(),
            reqdata.path().to_string(),
            reqdata.connection_info().host().to_string(),
            ua_string
        ];

        // Get log function and put requierd data into it.
        let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"POST",vlogdata);
        // Send information to log.
        let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
        let _ = logs::send_logs(logdata);

        // Fetch headers.
        let vheaders = api_headers();

        // Return answer.
        return Ok( HttpResponse::BadRequest()
            .append_header(("api-version",vheaders[0].clone()))
            .content_type(vheaders[1].clone())
            .json(data) );
    }

    // The command.
    let cmd = Command::new("podman")
        .arg("container")
        .arg(arg_input.to_lowercase().clone())
        .arg(jdata.name.clone())
        .output();

    // Check if command went ok or not.
    match cmd {
        Ok(cmd_ok) => {
            // Check length of stdout, returns empty response when no containers matched state set.
            let count = cmd_ok.stdout.len();
            if count > 4 {
                // Clean data from unneeded characters.
                let result =  format!("{}", String::from_utf8_lossy(&cmd_ok.stdout))
                    .trim_start()
                    .trim_end()
                    .replace("\n", "")
                    .to_string();

                // Construct JSON object.
                let data = json!(
                    {
                        "Code": 200,
                        "Info": format!("Container '{}' has been set to '{}' state",result,jdata.state.to_lowercase()),
                        "Status": "OK"
                    }
                );

                // Get USER-AGENT from request header, ugly but works.
                let mut ua_string = String::new();
                for v in reqdata.headers().get_all(USER_AGENT) {
                    ua_string = format!("{:?}",v);
                };

                // Vec for HttpRequest data to log.
                // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
                let vlogdata = vec![
                    reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                    reqdata.connection_info().scheme().to_string(),
                    reqdata.path().to_string(),
                    reqdata.connection_info().host().to_string(),
                    ua_string
                ];

                // Get log function and put requierd data into it.
                let vlog: Vec<String> = logs::log_data(200,"Request OK",0,"POST",vlogdata);
                // Send information to log.
                let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={} oCMAPIContainerName={} oCMAPISetState={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9],jdata.name,jdata.state);
                let _ = logs::send_logs(logdata);

                // Fetch headers.
                let vheaders = api_headers();

                // Return answer.
                Ok( HttpResponse::Ok()
                    .append_header(("api-version",vheaders[0].clone()))
                    .content_type(vheaders[1].clone())
                    .json(data) )
            }
            // Return status when no containers are matching state.
            else {
                // Construct JSON object
                let data = json!(
                    [
                        {
                            "Code": 400,
                            "Info": format!("{}",String::from_utf8_lossy(&cmd_ok.stderr).replace("\n", "").replace("\t", "")),
                            "State": "Bad 'Request"
                        }
                    ]
                );

                // Get USER-AGENT from request header, ugly but works.
                let mut ua_string = String::new();
                for v in reqdata.headers().get_all(USER_AGENT) {
                    ua_string = format!("{:?}",v);
                };

                // Vec for HttpRequest data to log.
                // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
                let vlogdata = vec![
                    reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                    reqdata.connection_info().scheme().to_string(),
                    reqdata.path().to_string(),
                    reqdata.connection_info().host().to_string(),
                    ua_string
                ];

                // Get log function and put requierd data into it.
                let vlog: Vec<String> = logs::log_data(400,"Bad 'Request",0,"POST",vlogdata);
                // Send information to log.
                let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
                let _ = logs::send_logs(logdata);

                // Fetch headers.
                let vheaders = api_headers();

                // Return answer.
                Ok( HttpResponse::BadRequest()
                    .append_header(("api-version",vheaders[0].clone()))
                    .content_type(vheaders[1].clone())
                    .json(data) )
            }
        }
        Err(cmd_err) => {
            // Construct JSON object
            let data = json!(
                {
                    "Code": 500,
                    "Info": format!("{}", cmd_err),
                    "Status": "Internal Server Error"
                }
            );

            // Get USER-AGENT from request header, ugly but works.
            let mut ua_string = String::new();
            for v in reqdata.headers().get_all(USER_AGENT) {
                ua_string = format!("{:?}",v);
            };

            // Vec for HttpRequest data to log.
            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
            let vlogdata = vec![
                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                reqdata.connection_info().scheme().to_string(),
                reqdata.path().to_string(),
                reqdata.connection_info().host().to_string(),
                ua_string
            ];

            // Get log function and put requierd data into it.
            let vlog: Vec<String> = logs::log_data(500,"Internal Server Error",6,"POST",vlogdata);
            // Send information to log.
            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
            let _ = logs::send_logs(logdata);

            // Fetch headers.
            let vheaders = api_headers();

            // Return answer.
            Ok( HttpResponse::InternalServerError()
                .append_header(("api-version",vheaders[0].clone()))
                .content_type(vheaders[1].clone())
                .json(data) )
        }
    }
}

// Delete contain.to_lowercase()er.
pub async fn delete_containers_data(sdata: web::Json<DeleteContainer>, reqdata: HttpRequest) -> io::Result<HttpResponse> {
    // Get JSON data from post.
    let jdata = sdata.into_inner();

    // Regex for non allowed characters.
    let rx_name = Regex::new(r"([^A-Za-z0-9_-])").unwrap();

    // Check if regex matches anything in name.
    if rx_name.find(jdata.name.as_str()).is_some() {
        // Construct JSON object
        let data = json!(
            {
                "Code": 400,
                "Info": "Name provided is not in correct format",
                "Status": "Bad Request"
            }
        );

        // Get USER-AGENT from request header, ugly but works.
        let mut ua_string = String::new();
        for v in reqdata.headers().get_all(USER_AGENT) {
            ua_string = format!("{:?}",v);
        };

        // Vec for HttpRequest data to log.
        // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
        let vlogdata = vec![
            reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
            reqdata.connection_info().scheme().to_string(),
            reqdata.path().to_string(),
            reqdata.connection_info().host().to_string(),
            ua_string
        ];

        // Get log function and put requierd data into it.
        let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"DELETE",vlogdata);
        // Send information to log.
        let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
        let _ = logs::send_logs(logdata);

        // Fetch headers.
        let vheaders = api_headers();

        // Return answer.
        return Ok( HttpResponse::BadRequest()
            .append_header(("api-version",vheaders[0].clone()))
            .content_type(vheaders[1].clone())
            .json(data) );
    }

    // Check if secret isn´t empty and not matching key from config.
    if jdata.key.is_empty() || jdata.key != CONTAINERS_KEY.get().unwrap().to_string() {
        // Construct JSON object
        let data = json!(
            {
                "Code": 400,
                "Info": "Missing or incorrect data",
                "Status": "Bad Request"
            }
        );

        // Get USER-AGENT from request header, ugly but works.
        let mut ua_string = String::new();
        for v in reqdata.headers().get_all(USER_AGENT) {
            ua_string = format!("{:?}",v);
        };

        // Vec for HttpRequest data to log.
        // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
        let vlogdata = vec![
            reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
            reqdata.connection_info().scheme().to_string(),
            reqdata.path().to_string(),
            reqdata.connection_info().host().to_string(),
            ua_string
        ];

        // Get log function and put requierd data into it.
        let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"DELETE",vlogdata);
        // Send information to log.
        let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
        let _ = logs::send_logs(logdata);

        // Fetch headers.
        let vheaders = api_headers();

        // Return answer.
        return Ok( HttpResponse::BadRequest()
            .append_header(("api-version",vheaders[0].clone()))
            .content_type(vheaders[1].clone())
            .json(data) );
    }

    // Match key to make sure it is correct.
    if jdata.key == CONTAINERS_KEY.get().unwrap().to_string() {
        // Container filter.
        let cfilter = format!("name={}", jdata.name);

        // Check if container is stopped, if not abort.
        let cmdrunning = Command::new("podman")
            .arg("container")
            .arg("ps")
            .arg("--all")
            .arg("--filter")
            .arg(cfilter)
            .arg("--format='{{ .State }}'")
            .output()?;

        // Convert result to string.
        let state = format!("[{}]", String::from_utf8_lossy(&cmdrunning.stdout)).replace("['", "").replace("'\n]", "");

        // Check if file could be deleted.
        match state.as_str() {
            "exited" => {
                // Command, return name of the container if successful.
                let cmd = Command::new("podman")
                    .arg("container")
                    .arg("rm")
                    .arg(jdata.name.clone())
                    .output()?;

                // Convert result to string.
                let result = format!("[{}]", String::from_utf8_lossy(&cmd.stdout)).replace("[", "").replace("\n]", "");

                // Check return status of the command.
                if result == jdata.name {
                    // Construct JSON object.
                    let data = json!(
                        {
                            "Code": 200,
                            "Info": format!("Container '{}' has been deleted",jdata.name),
                            "Status": "OK"
                        }
                    );

                    // Get USER-AGENT from request header, ugly but works.
                    let mut ua_string = String::new();
                    for v in reqdata.headers().get_all(USER_AGENT) {
                        ua_string = format!("{:?}",v);
                    };

                    // Vec for HttpRequest data to log.
                    // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
                    let vlogdata = vec![
                        reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                        reqdata.connection_info().scheme().to_string(),
                        reqdata.path().to_string(),
                        reqdata.connection_info().host().to_string(),
                        ua_string
                    ];

                    // Get log function and put requierd data into it.
                    let vlog: Vec<String> = logs::log_data(200,"Request OK",0,"DELETE",vlogdata);
                    // Send information to log.
                    let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={} oCMAPIDeleteID={} oCMAPIDeleteType=container",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9],jdata.name);
                    let _ = logs::send_logs(logdata);

                    // Fetch headers.
                    let vheaders = api_headers();

                    // Return answer.
                    Ok( HttpResponse::Ok()
                        .append_header(("api-version",vheaders[0].clone()))
                        .content_type(vheaders[1].clone())
                        .json(data) )

                }
                else {
                // Construct JSON object
                let data = json!(
                    {
                        "Code": 400,
                        "Info": "Could not remove container",
                        "Status": "Bad Request"
                    }
                );

                // Get USER-AGENT from request header, ugly but works.
                let mut ua_string = String::new();
                for v in reqdata.headers().get_all(USER_AGENT) {
                    ua_string = format!("{:?}",v);
                };

                // Vec for HttpRequest data to log.
                // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
                let vlogdata = vec![
                    reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                    reqdata.connection_info().scheme().to_string(),
                    reqdata.path().to_string(),
                    reqdata.connection_info().host().to_string(),
                    ua_string
                ];

                // Get log function and put requierd data into it.
                let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"DELETE",vlogdata);
                // Send information to log.
                let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
                let _ = logs::send_logs(logdata);

                // Fetch headers.
                let vheaders = api_headers();

                // Return answer.
                return Ok( HttpResponse::BadRequest()
                    .append_header(("api-version",vheaders[0].clone()))
                    .content_type(vheaders[1].clone())
                    .json(data) )

                }
            },
            _ => {
                // Construct JSON object
                let data = json!(
                    {
                        "Code": 400,
                        "Info": "Container is not in 'exited' state or do not exist",
                        "Status": "Bad Request"
                    }
                );

                // Get USER-AGENT from request header, ugly but works.
                let mut ua_string = String::new();
                for v in reqdata.headers().get_all(USER_AGENT) {
                    ua_string = format!("{:?}",v);
                };

                // Vec for HttpRequest data to log.
                // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
                let vlogdata = vec![
                    reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                    reqdata.connection_info().scheme().to_string(),
                    reqdata.path().to_string(),
                    reqdata.connection_info().host().to_string(),
                    ua_string
                ];

                // Get log function and put requierd data into it.
                let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"DELETE",vlogdata);
                // Send information to log.
                let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
                let _ = logs::send_logs(logdata);

                // Fetch headers.
                let vheaders = api_headers();

                // Return answer.
                return Ok( HttpResponse::BadRequest()
                    .append_header(("api-version",vheaders[0].clone()))
                    .content_type(vheaders[1].clone())
                    .json(data) );
            }
        }
    }
    else {
        // Construct JSON object
        let data = json!(
            {
                "Code": 400,
                "Info": "Missing or incorrect data",
                "Status": "Bad Request"
            }
        );

        // Get USER-AGENT from request header, ugly but works.
        let mut ua_string = String::new();
        for v in reqdata.headers().get_all(USER_AGENT) {
            ua_string = format!("{:?}",v);
        };

        // Vec for HttpRequest data to log.
        // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
        let vlogdata = vec![
            reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
            reqdata.connection_info().scheme().to_string(),
            reqdata.path().to_string(),
            reqdata.connection_info().host().to_string(),
            ua_string
        ];

        // Get log function and put requierd data into it.
        let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"DELETE",vlogdata);
        // Send information to log.
        let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
        let _ = logs::send_logs(logdata);

        // Fetch headers.
        let vheaders = api_headers();

        // Return answer.
        return Ok( HttpResponse::BadRequest()
            .append_header(("api-version",vheaders[0].clone()))
            .content_type(vheaders[1].clone())
            .json(data) );
    }
}

// Delete volumes.
pub async fn delete_containers_volume_data(sdata: web::Json<DeleteVolume>, reqdata: HttpRequest) -> io::Result<HttpResponse> {
    // Get JSON data from post.
    let jdata = sdata.into_inner();

    // Regex for non allowed characters.
    let rx_name = Regex::new(r"([^A-Za-z0-9_-])").unwrap();
    
    // Check if regex matches anything in name.
    if rx_name.find(jdata.name.as_str()).is_some() {
        // Construct JSON object
        let data = json!(
            {
                "Code": 400,
                "Info": "Name provided is not in correct format",
                "Status": "Bad Request"
            }
        );

        // Get USER-AGENT from request header, ugly but works.
        let mut ua_string = String::new();
        for v in reqdata.headers().get_all(USER_AGENT) {
            ua_string = format!("{:?}",v);
        };
        // Vec for HttpRequest data to log.
        // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
        let vlogdata = vec![
            reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
            reqdata.connection_info().scheme().to_string(),
            reqdata.path().to_string(),
            reqdata.connection_info().host().to_string(),
            ua_string
        ];

        // Get log function and put requierd data into it.
        let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"DELETE",vlogdata);
        // Send information to log.
        let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
        let _ = logs::send_logs(logdata);

        // Fetch headers.
        let vheaders = api_headers();

        // Return answer.
        return Ok( HttpResponse::BadRequest()
            .append_header(("api-version",vheaders[0].clone()))
            .content_type(vheaders[1].clone())
            .json(data) );
    }

    // Check if secret isn´t empty and not matching key from config.
    if jdata.key.is_empty() || jdata.key != CONTAINERS_KEY.get().unwrap().to_string() {
        // Construct JSON object
        let data = json!(
            {
                "Code": 400,
                "Info": "Missing or incorrect data",
                "Status": "Bad Request"
            }
        );

        // Get USER-AGENT from request header, ugly but works.
        let mut ua_string = String::new();
        for v in reqdata.headers().get_all(USER_AGENT) {
            ua_string = format!("{:?}",v);
        };

        // Vec for HttpRequest data to log.
        // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
        let vlogdata = vec![
            reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
            reqdata.connection_info().scheme().to_string(),
            reqdata.path().to_string(),
            reqdata.connection_info().host().to_string(),
            ua_string
        ];

        // Get log function and put requierd data into it.
        let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"DELETE",vlogdata);
        // Send information to log.
        let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
        let _ = logs::send_logs(logdata);

        // Fetch headers.
        let vheaders = api_headers();

        // Return answer.
        return Ok( HttpResponse::BadRequest()
            .append_header(("api-version",vheaders[0].clone()))
            .content_type(vheaders[1].clone())
            .json(data) );
    }

    // Match key to make sure it is correct.
    if jdata.key == CONTAINERS_KEY.get().unwrap().to_string() {

        // Command, return name of the container if successful.
        let cmd = Command::new("podman")
            .arg("volume")
            .arg("rm")
            .arg(jdata.name.clone())
            .output()
            .expect("Exit code 0...");
        
        // Check exit code.
        match cmd.status.code() {
            Some(0) => {
                // Construct JSON object.
                let data = json!(
                    {
                        "Code": 200,
                        "Info": format!("Volume '{}' has been deleted",jdata.name),
                        "Status": "OK"
                    }
                );
                // Get USER-AGENT from request header, ugly but works.
                let mut ua_string = String::new();
                for v in reqdata.headers().get_all(USER_AGENT) {
                    ua_string = format!("{:?}",v);
                };
                // Vec for HttpRequest data to log.
                // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
                let vlogdata = vec![
                    reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                    reqdata.connection_info().scheme().to_string(),
                    reqdata.path().to_string(),
                    reqdata.connection_info().host().to_string(),
                    ua_string
                ];
                // Get log function and put requierd data into it.
                let vlog: Vec<String> = logs::log_data(200,"Request OK",0,"DELETE",vlogdata);
                // Send information to log.
                let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={} oCMAPIDeleteID={} oCMAPIDeleteType=volume",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9],jdata.name);
                let _ = logs::send_logs(logdata);
                // Fetch headers.
                let vheaders = api_headers();
                // Return answer.
                return Ok( HttpResponse::Ok()
                    .append_header(("api-version",vheaders[0].clone()))
                    .content_type(vheaders[1].clone())
                    .json(data) );
            },
            _ => {
            // Construct JSON object
            let data = json!(
                {
                    "Code": 400,
                    "Info": format!("Cannot remove volume '{}', still used or do not exist",jdata.name),
                    "Status": "Bad Request",
                }
            );
            // Get USER-AGENT from request header, ugly but works.
            let mut ua_string = String::new();
            for v in reqdata.headers().get_all(USER_AGENT) {
                ua_string = format!("{:?}",v);
            };
            // Vec for HttpRequest data to log.
            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
            let vlogdata = vec![
                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                reqdata.connection_info().scheme().to_string(),
                reqdata.path().to_string(),
                reqdata.connection_info().host().to_string(),
                ua_string
            ];
            // Get log function and put requierd data into it.
            let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"GET",vlogdata);
            // Send information to log.
            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
            let _ = logs::send_logs(logdata);
            // Fetch headers.
            let vheaders = api_headers();
            // Return answer.
            return Ok( HttpResponse::BadRequest()
                .append_header(("api-version",vheaders[0].clone()))
                .content_type(vheaders[1].clone())
                .json(data) );
            }
        }
    }
    else {
        // Construct JSON object
        let data = json!(
            {
                "Code": 400,
                "Info": "Missing or incorrect data",
                "Status": "Bad Request"
            }
        );

        // Get USER-AGENT from request header, ugly but works.
        let mut ua_string = String::new();
        for v in reqdata.headers().get_all(USER_AGENT) {
            ua_string = format!("{:?}",v);
        };

        // Vec for HttpRequest data to log.
        // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
        let vlogdata = vec![
            reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
            reqdata.connection_info().scheme().to_string(),
            reqdata.path().to_string(),
            reqdata.connection_info().host().to_string(),
            ua_string
        ];

        // Get log function and put requierd data into it.
        let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"DELETE",vlogdata);
        // Send information to log.
        let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
        let _ = logs::send_logs(logdata);

        // Fetch headers.
        let vheaders = api_headers();

        // Return answer.
        return Ok( HttpResponse::BadRequest()
            .append_header(("api-version",vheaders[0].clone()))
            .content_type(vheaders[1].clone())
            .json(data) );
    }
}

/* --- Pods API --- */

// Get all Pods status information.
pub async fn get_pods_status(reqdata: HttpRequest) -> io::Result<HttpResponse> {
    // The command.
    let cmd = Command::new("podman")
        .arg("pod")
        .arg("list")
        .arg(
            "--format='
            { \"Name\": {{json .Name}},
            \"ID\": {{json .Id}},
            \"Status\": {{json .Status}} },'"
        )
        .output();

    // Check if command went ok or not.
    match cmd {
        // When Ok build response.
        Ok(cmd_ok) => {
            // Clean data from unneeded characters.
            let data = format!("[{}]", String::from_utf8_lossy(&cmd_ok.stdout))
                .trim_start()
                .trim_end()
                .to_string()
                .replace("'", "")
                .replace(",\n]", "]");

            // Get USER-AGENT from request header, ugly but works.
            let mut ua_string = String::new();
            for v in reqdata.headers().get_all(USER_AGENT) {
                ua_string = format!("{:?}",v);
            };

            // Vec for HttpRequest data to log.
            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
            let vlogdata = vec![
                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                reqdata.connection_info().scheme().to_string(),
                reqdata.path().to_string(),
                reqdata.connection_info().host().to_string(),
                ua_string
            ];

            // Get log function and put requierd data into it.
            let vlog: Vec<String> = logs::log_data(200,"Request OK",0,"GET",vlogdata);
            // Send information to log.
            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
            let _ = logs::send_logs(logdata);

            // Fetch headers.
            let vheaders = api_headers();

            // Return answer.
            Ok( HttpResponse::Ok()
                .append_header(("api-version",vheaders[0].clone()))
                .content_type(vheaders[1].clone())
                .body(data) )
        },
        // When error build response.
        Err(cmd_err) => {
            // Construct JSON object
            let data = json!(
                {
                    "Code": 400,
                    "Info": format!("{}", cmd_err),
                    "Status": "Bad Request"
                }
            );

            // Get USER-AGENT from request header, ugly but works.
            let mut ua_string = String::new();
            for v in reqdata.headers().get_all(USER_AGENT) {
                ua_string = format!("{:?}",v);
            };

            // Vec for HttpRequest data to log.
            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
            let vlogdata = vec![
                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                reqdata.connection_info().scheme().to_string(),
                reqdata.path().to_string(),
                reqdata.connection_info().host().to_string(),
                ua_string
            ];

            // Get log function and put requierd data into it.
            let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"GET",vlogdata);
            // Send information to log.
            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
            let _ = logs::send_logs(logdata);

            // Fetch headers.
            let vheaders = api_headers();

            // Return answer.
            Ok( HttpResponse::InternalServerError()
                .append_header(("api-version",vheaders[0].clone()))
                .content_type(vheaders[1].clone())
                .json(data) )
        }
    }
}

// Get Pods with a specific status.
pub async fn get_pods_status_query(param: web::Path<String>,reqdata: HttpRequest) -> io::Result<HttpResponse> {
    // Create arg_input variable.
    let arg_input;

    // Check param from input, must match.
    match param.to_lowercase().as_str() {
        "created" => { arg_input = format!("status={}", param.to_lowercase()); },
        "exited" => { arg_input = format!("status={}", param.to_lowercase()); },
        "paused" => { arg_input = format!("status={}", param.to_lowercase()); },
        "running" => { arg_input = format!("status={}", param.to_lowercase()); },
        "unknown" => { arg_input = format!("status={}", param.to_lowercase()); },
        _ => { arg_input = format!("status={}", "ERROR"); },
    }

    // The command.
    let cmd = Command::new("podman")
        .arg("pod")
        .arg("list")
        .arg(format!("--filter={}", arg_input.clone()))
        .arg(
            "--format='
            { \"Name\": {{json .Name}},
            \"ID\": {{json .ID}},
            \"Status\": {{json .Status}} },'"
        )
        .output();

    // Check if command went ok or not.
    match cmd {
        Ok(cmd_ok) => {
            // Check if ERROR based on input param assignment.
            if arg_input == "status=ERROR" {
                // Construct JSON object
                let data = json!(
                    {
                        "Code": 400,
                        "Info": "Could not process data",
                        "Status": "Bad Request"
                    }
                );

                // Get USER-AGENT from request header, ugly but works.
                let mut ua_string = String::new();
                for v in reqdata.headers().get_all(USER_AGENT) {
                    ua_string = format!("{:?}",v);
                };

                // Vec for HttpRequest data to log.
                // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
                let vlogdata = vec![
                    reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                    reqdata.connection_info().scheme().to_string(),
                    reqdata.path().to_string(),
                    reqdata.connection_info().host().to_string(),
                    ua_string
                ];

                // Get log function and put requierd data into it.
                let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"GET",vlogdata);
                // Send information to log.
                let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
                let _ = logs::send_logs(logdata);

                // Fetch headers.
                let vheaders = api_headers();

                // Return answer.
                Ok( HttpResponse::BadRequest()
                    .append_header(("api-version",vheaders[0].clone()))
                    .content_type(vheaders[1].clone())
                    .json(data) )
            }
            else {
                // Check length of stdout, returns empty response when no containers are running.
                let count = cmd_ok.stdout.len();
                if count > 4 {
                    // Clean data from unneeded characters.
                    let data = format!("[{}]", String::from_utf8_lossy(&cmd_ok.stdout))
                        .trim_start()
                        .trim_end()
                        .to_string()
                        .replace("'", "")
                        .replace(",\n]", "]");

                    // Get USER-AGENT from request header, ugly but works.
                    let mut ua_string = String::new();
                    for v in reqdata.headers().get_all(USER_AGENT) {
                        ua_string = format!("{:?}",v);
                    };

                    // Vec for HttpRequest data to log.
                    // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
                    let vlogdata = vec![
                        reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                        reqdata.connection_info().scheme().to_string(),
                        reqdata.path().to_string(),
                        reqdata.connection_info().host().to_string(),
                        ua_string
                    ];

                    // Get log function and put requierd data into it.
                    let vlog: Vec<String> = logs::log_data(200,"Request OK",0,"GET",vlogdata);
                    // Send information to log.
                    let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
                    let _ = logs::send_logs(logdata);

                    // Fetch headers.
                    let vheaders = api_headers();

                    // Return answer.
                    Ok( HttpResponse::Ok()
                        .append_header(("api-version",vheaders[0].clone()))
                        .content_type(vheaders[1].clone())
                        .body(data) )
                }
                // Return error since no containers are running.
                else {
                    let data = json!(
                        [
                            {
                                "Code": 404,
                                "Info": format!("No pods matching query status: {}",arg_input.replace("status=", "")),
                                "Status": "None"
                            }
                        ]
                    );

                    // Get USER-AGENT from request header, ugly but works.
                    let mut ua_string = String::new();
                    for v in reqdata.headers().get_all(USER_AGENT) {
                        ua_string = format!("{:?}",v);
                    };

                    // Vec for HttpRequest data to log.
                    // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
                    let vlogdata = vec![
                        reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                        reqdata.connection_info().scheme().to_string(),
                        reqdata.path().to_string(),
                        reqdata.connection_info().host().to_string(),
                        ua_string
                    ];

                    // Get log function and put requierd data into it.
                    let vlog: Vec<String> = logs::log_data(404,"Not Found",0,"GET",vlogdata);
                    // Send information to log.
                    let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
                    let _ = logs::send_logs(logdata);

                    // Fetch headers.
                    let vheaders = api_headers();

                    // Return answer.
                    Ok( HttpResponse::NotFound()
                        .append_header(("api-version",vheaders[0].clone()))
                        .content_type(vheaders[1].clone())
                        .json(data) )
                }
            }
        }
        Err(cmd_err) => {
            // Construct JSON object
            let data = json!(
                {
                    "Code": 500,
                    "Info": format!("{}", cmd_err),
                    "Status": "Internal Server Error"
                }
            );

            // Get USER-AGENT from request header, ugly but works.
            let mut ua_string = String::new();
            for v in reqdata.headers().get_all(USER_AGENT) {
                ua_string = format!("{:?}",v);
            };

            // Vec for HttpRequest data to log.
            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
            let vlogdata = vec![
                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                reqdata.connection_info().scheme().to_string(),
                reqdata.path().to_string(),
                reqdata.connection_info().host().to_string(),
                ua_string
            ];

            // Get log function and put requierd data into it.
            let vlog: Vec<String> = logs::log_data(500,"Internal Server Error",6,"GET",vlogdata);
            // Send information to log.
            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
            let _ = logs::send_logs(logdata);

            // Fetch headers.
            let vheaders = api_headers();

            // Return answer.
            Ok( HttpResponse::InternalServerError()
                .append_header(("api-version",vheaders[0].clone()))
                .content_type(vheaders[1].clone())
                .json(data) )
        }
    }
}

// Get state of a single pod.
pub async fn get_pods_single_state_query(param: web::Path<String>,reqdata: HttpRequest) -> io::Result<HttpResponse> {
    // Get input data from post.
    let indata= param;

    // Regex for non allowed characters.
    let rx_nameid = Regex::new(r"([^A-Za-z0-9_-])").unwrap();

    // Check if regex matches anything in name.
    if indata.is_empty() || rx_nameid.find(&&indata.as_str()).is_some() {
        // Construct JSON object
        let data = json!(
            {
                "Code": 400,
                "Info": "Information provided is not in correct format",
                "Status": "Bad Request"
            }
        );

        // Get USER-AGENT from request header, ugly but works.
        let mut ua_string = String::new();
        for v in reqdata.headers().get_all(USER_AGENT) {
            ua_string = format!("{:?}",v);
        };

        // Vec for HttpRequest data to log.
        // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
        let vlogdata = vec![
            reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
            reqdata.connection_info().scheme().to_string(),
            reqdata.path().to_string(),
            reqdata.connection_info().host().to_string(),
            ua_string
        ];

        // Get log function and put requierd data into it.
        let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"POST",vlogdata);
        // Send information to log.
        let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
        let _ = logs::send_logs(logdata);

        // Fetch headers.
        let vheaders = api_headers();

        // Return answer.
        return Ok( HttpResponse::BadRequest()
            .append_header(("api-version",vheaders[0].clone()))
            .content_type(vheaders[1].clone())
            .json(data) );
    }

    // The command for getting via name.
    let cmd_name = Command::new("podman")
        .arg("pod")
        .arg("ps")
        .arg("--filter")
        .arg(format!("name={}",indata.clone()))
        .arg(
            "--format='
            { \"Name\": {{json .Name}},
            \"ID\": {{json .Id}},
            \"Status\": {{json .Status}} }'"
        )
        .output();

    // Check if command went ok or not.
    match cmd_name {
        Ok(cmd_name_ok) => {
            // Check length of stdout, returns empty response when no match occured.
            let name_count = cmd_name_ok.stdout.len();
            if name_count > 4 {
                // Clean data from unneeded characters.
                let data = format!("{}", String::from_utf8_lossy(&cmd_name_ok.stdout))
                    .trim_start()
                    .trim_end()
                    .to_string()
                    .replace("'", "")
                    .replace(",\n]", "]");
                // Get USER-AGENT from request header, ugly but works.
                let mut ua_string = String::new();
                for v in reqdata.headers().get_all(USER_AGENT) {
                    ua_string = format!("{:?}",v);
                };

                // Vec for HttpRequest data to log.
                // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
                let vlogdata = vec![
                    reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                    reqdata.connection_info().scheme().to_string(),
                    reqdata.path().to_string(),
                    reqdata.connection_info().host().to_string(),
                    ua_string
                ];

                // Get log function and put requierd data into it.
                let vlog: Vec<String> = logs::log_data(200,"Request OK",0,"GET",vlogdata);
                // Send information to log.
                let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
                let _ = logs::send_logs(logdata);

                // Fetch headers.
                let vheaders = api_headers();

                // Return answer.
                return Ok( HttpResponse::Ok()
                    .append_header(("api-version",vheaders[0].clone()))
                    .content_type(vheaders[1].clone())
                    .body(data) );
            }
            // Check if ID was given.
            else {
                // The command for getting via name.
                let cmd_id = Command::new("podman")
                    .arg("pod")
                    .arg("ps")
                    .arg("--filter")
                    .arg(format!("id={}",indata.clone()))
                    .arg(
                        "--format='
                        { \"Name\": {{json .Name}},
                        \"ID\": {{json .Id}},
                        \"Status\": {{json .Status}} }'"
                    )
                    .output();

                // Check if command went ok or not.
                match cmd_id {
                    Ok(cmd_id_ok) => {
                        // Check length of stdout, returns empty response when no match occured.
                        let id_count = cmd_id_ok.stdout.len();
                        if id_count > 4 {
                            // Clean data from unneeded characters.
                            let data = format!("{}", String::from_utf8_lossy(&cmd_id_ok.stdout))
                                .trim_start()
                                .trim_end()
                                .to_string()
                                .replace("'", "")
                                .replace(",\n]", "]");

                            // Get USER-AGENT from request header, ugly but works.
                            let mut ua_string = String::new();
                            for v in reqdata.headers().get_all(USER_AGENT) {
                                ua_string = format!("{:?}",v);
                            };

                            // Vec for HttpRequest data to log.
                            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
                            let vlogdata = vec![
                                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                                reqdata.connection_info().scheme().to_string(),
                                reqdata.path().to_string(),
                                reqdata.connection_info().host().to_string(),
                                ua_string
                            ];

                            // Get log function and put requierd data into it.
                            let vlog: Vec<String> = logs::log_data(200,"Request OK",0,"GET",vlogdata);
                            // Send information to log.
                            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
                            let _ = logs::send_logs(logdata);

                            // Fetch headers.
                            let vheaders = api_headers();

                            // Return answer.
                            return Ok( HttpResponse::Ok()
                                .append_header(("api-version",vheaders[0].clone()))
                                .content_type(vheaders[1].clone())
                                .body(data) );
                        }
                        else {

                            // Construct JSON object
                            let data = json!(
                                [
                                    {
                                        "Code": 404,
                                        "Info": format!("No containers matching name or ID: {}",indata),
                                        "Status": "Not Found"
                                    }
                                ]
                            );

                            // Get USER-AGENT from request header, ugly but works.
                            let mut ua_string = String::new();
                            for v in reqdata.headers().get_all(USER_AGENT) {
                                ua_string = format!("{:?}",v);
                            };

                            // Vec for HttpRequest data to log.
                            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
                            let vlogdata = vec![
                                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                                reqdata.connection_info().scheme().to_string(),
                                reqdata.path().to_string(),
                                reqdata.connection_info().host().to_string(),
                                ua_string
                            ];

                            // Get log function and put requierd data into it.
                            let vlog: Vec<String> = logs::log_data(404,"Not Found",0,"GET",vlogdata);
                            // Send information to log.
                            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
                            let _ = logs::send_logs(logdata);

                            // Fetch headers.
                            let vheaders = api_headers();

                            // Return answer.
                            return Ok( HttpResponse::NotFound()
                                .append_header(("api-version",vheaders[0].clone()))
                                .content_type(vheaders[1].clone())
                                .json(data) );
                        }
                    }
                    Err(cmd_id_err) => {
                        // Construct JSON object
                        let data = json!(
                            {
                                "Code": 500,
                                "Info": format!("{}", cmd_id_err),
                                "Status": "Internal Server Error"
                            }
                        );

                        // Get USER-AGENT from request header, ugly but works.
                        let mut ua_string = String::new();
                        for v in reqdata.headers().get_all(USER_AGENT) {
                            ua_string = format!("{:?}",v);
                        };

                        // Vec for HttpRequest data to log.
                        // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
                        let vlogdata = vec![
                            reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                            reqdata.connection_info().scheme().to_string(),
                            reqdata.path().to_string(),
                            reqdata.connection_info().host().to_string(),
                            ua_string
                        ];

                        // Get log function and put requierd data into it.
                        let vlog: Vec<String> = logs::log_data(500,"Internal Server Error",6,"GET",vlogdata);
                        // Send information to log.
                        let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
                        let _ = logs::send_logs(logdata);

                        // Fetch headers.
                        let vheaders = api_headers();

                        // Return answer.
                        return Ok( HttpResponse::InternalServerError()
                            .append_header(("api-version",vheaders[0].clone()))
                            .content_type(vheaders[1].clone())
                            .json(data) );
                    }
                }
            }
        }
        Err(cmd_name_err) => {
            // Construct JSON object
            let data = json!(
                {
                    "Code": 500,
                    "Info": format!("{}", cmd_name_err),
                    "Status": "Internal Server Error"
                }
            );

            // Get USER-AGENT from request header, ugly but works.
            let mut ua_string = String::new();
            for v in reqdata.headers().get_all(USER_AGENT) {
                ua_string = format!("{:?}",v);
            };

            // Vec for HttpRequest data to log.
            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
            let vlogdata = vec![
                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                reqdata.connection_info().scheme().to_string(),
                reqdata.path().to_string(),
                reqdata.connection_info().host().to_string(),
                ua_string
            ];

            // Get log function and put requierd data into it.
            let vlog: Vec<String> = logs::log_data(500,"Internal Server Error",6,"GET",vlogdata);
            // Send information to log.
            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
            let _ = logs::send_logs(logdata);

            // Fetch headers.
            let vheaders = api_headers();

            // Return answer.
            Ok( HttpResponse::InternalServerError()
                .append_header(("api-version",vheaders[0].clone()))
                .content_type(vheaders[1].clone())
                .json(data) )
        }
    }
}

// Get Names or ID of a all pods.
pub async fn get_pods_short_list(param: web::Path<String>,reqdata: HttpRequest) -> io::Result<HttpResponse> {
    // Create arg_input variable.
    let mut arg_input;

    // Check param from input, must match.
    match param.to_lowercase().as_str() {
        "name" => { arg_input = "Name" },
        "id" => { arg_input = "ID" },
        _ => { arg_input = "ERROR" },
    }

    // If incorrect value provided error out.
    if arg_input == "ERROR" {
        // Construct JSON object
        let data = json!(
            {
                "Code": 400,
                "Info": "Not a valid list option",
                "Status": "Bad Request"
            }
        );

        // Get USER-AGENT from request header, ugly but works.
        let mut ua_string = String::new();
        for v in reqdata.headers().get_all(USER_AGENT) {
            ua_string = format!("{:?}",v);
        };

        // Vec for HttpRequest data to log.
        // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
        let vlogdata = vec![
            reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
            reqdata.connection_info().scheme().to_string(),
            reqdata.path().to_string(),
            reqdata.connection_info().host().to_string(),
            ua_string
        ];

        // Get log function and put requierd data into it.
        let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"POST",vlogdata);
        // Send information to log.
        let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
        let _ = logs::send_logs(logdata);

        // Fetch headers.
        let vheaders = api_headers();

        // Return answer.
        return Ok( HttpResponse::BadRequest()
            .append_header(("api-version",vheaders[0].clone()))
            .content_type(vheaders[1].clone())
            .json(data) );
    }

    // The command.
    let cmd = Command::new("podman")
        .arg("pod")
        .arg("ps")
        .arg(format!("--format={{{{ .{} }}}}", arg_input))
        .output();

    // Check if command went ok or not.
    match cmd {
        Ok(cmd_ok) => {
            // Convert stdout to string.
            let mut cnt_list = format!("{:?}", String::from_utf8_lossy(&cmd_ok.stdout)).replace('"', "").replace("\\n", ",");

            // Remove last character from string.
            cnt_list.pop();

            // Create empty array
            let mut vecdata: Vec<String> = Vec::new();

            // Fill array
            for v in cnt_list.split(",").map(String::from) {
                // Fill vec.
                vecdata.push(v.to_string());
            }

            // Cosmetic change for output.
            if arg_input == "Name" {
                arg_input = "Names";
            }

            // Clean data from unneeded characters.
            let data = format!("{{\"{}\":{:?}}}", arg_input, &vecdata)
                .trim_start()
                .trim_end()
                .to_string();

            // Get USER-AGENT from request header, ugly but works.
            let mut ua_string = String::new();
            for v in reqdata.headers().get_all(USER_AGENT) {
                ua_string = format!("{:?}",v);
            };

            // Vec for HttpRequest data to log.
            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
            let vlogdata = vec![
                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                reqdata.connection_info().scheme().to_string(),
                reqdata.path().to_string(),
                reqdata.connection_info().host().to_string(),
                ua_string
            ];

            // Get log function and put requierd data into it.
            let vlog: Vec<String> = logs::log_data(200,"Request OK",0,"GET",vlogdata);
            // Send information to log.
            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
            let _ = logs::send_logs(logdata);

            // Fetch headers.
            let vheaders = api_headers();

            // Return answer.
            return Ok( HttpResponse::Ok()
                .append_header(("api-version",vheaders[0].clone()))
                .content_type(vheaders[1].clone())
                .body(data) );
        }
        Err(cmd_err) => {
            // Construct JSON object
            let data = json!(
                {
                    "Code": 500,
                    "Info": format!("{}", cmd_err),
                    "Status": "Internal Server Error"
                }
            );

            // Get USER-AGENT from request header, ugly but works.
            let mut ua_string = String::new();
            for v in reqdata.headers().get_all(USER_AGENT) {
                ua_string = format!("{:?}",v);
            };

            // Vec for HttpRequest data to log.
            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
            let vlogdata = vec![
                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                reqdata.connection_info().scheme().to_string(),
                reqdata.path().to_string(),
                reqdata.connection_info().host().to_string(),
                ua_string
            ];

            // Get log function and put requierd data into it.
            let vlog: Vec<String> = logs::log_data(500,"Internal Server Error",6,"GET",vlogdata);
            // Send information to log.
            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
            let _ = logs::send_logs(logdata);

            // Fetch headers.
            let vheaders = api_headers();

            // Return answer.
            Ok( HttpResponse::InternalServerError()
                .append_header(("api-version",vheaders[0].clone()))
                .content_type(vheaders[1].clone())
                .json(data) )
        }
    }
}

// Post pod create data.
pub async fn post_pods_create(cdata: web::Json<CreatePod>,reqdata: HttpRequest) -> io::Result<HttpResponse> {
    // Get JSON data from post.
    let jdata = cdata.into_inner();

    // Create empty array
    let mut vecdata: Vec<String> = Vec::new();

    // Check if required json tags has values.
    if jdata.name.len() > 2 {
        // Fill vec.
        vecdata.push("pod".to_string());
        vecdata.push("create".to_string());
        vecdata.push(format!("--name={}",jdata.name));

        // Check if requirements are meet.
        if jdata.options.len() > 4 && jdata.options.starts_with("--") {
            // iterate through options tag and split back to vec.
            for v in jdata.options.split(",-").map(String::from) {
                if v.starts_with("--") {
                    vecdata.push(v)
                }
                else {
                    vecdata.push(format!("-{}", v))
                }
            }
        }
        // if requirements is in wrong format, error out.
        else if !jdata.options.is_empty() {

            // Construct JSON object
            let data = json!(
                {
                    "Code": 400,
                    "Info": "Check options tag, must be at least 4 characters long and start with -- to be valid",
                    "Status": "Bad Request"
                }
            );

            // Get USER-AGENT from request header, ugly but works.
            let mut ua_string = String::new();
            for v in reqdata.headers().get_all(USER_AGENT) {
                ua_string = format!("{:?}",v);
            };

            // Vec for HttpRequest data to log.
            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
            let vlogdata = vec![
                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                reqdata.connection_info().scheme().to_string(),
                reqdata.path().to_string(),
                reqdata.connection_info().host().to_string(),
                ua_string
            ];

            // Get log function and put requierd data into it.
            let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"POST",vlogdata);
            // Send information to log.
            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
            let _ = logs::send_logs(logdata);

            // Fetch headers.
            let vheaders = api_headers();

            // Return answer.
            return Ok(HttpResponse::BadRequest()
                .append_header(("api-version",vheaders[0].clone()))
                .content_type(vheaders[1].clone())
                .json(data));
        }
    }
    // if requirements are not met, error out.
    else {
        // Construct JSON object
        let data = json!(
            {
                "Code": 400,
                "Info": "Required tags are missing information",
                "Status": "Bad Request"
            }
        );

        // Get USER-AGENT from request header, ugly but works.
        let mut ua_string = String::new();
        for v in reqdata.headers().get_all(USER_AGENT) {
            ua_string = format!("{:?}",v);
        };

        // Vec for HttpRequest data to log.
        // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
        let vlogdata = vec![
            reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
            reqdata.connection_info().scheme().to_string(),
            reqdata.path().to_string(),
            reqdata.connection_info().host().to_string(),
            ua_string
        ];

        // Get log function and put requierd data into it.
        let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"POST",vlogdata);
        // Send information to log.
        let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
        let _ = logs::send_logs(logdata);

        // Fetch headers.
        let vheaders = api_headers();

        // Return answer.
        return Ok(HttpResponse::BadRequest()
            .append_header(("api-version",vheaders[0].clone()))
            .content_type(vheaders[1].clone())
            .json(data));
    }

    //  Const with bad options.
    const BAD_OPTIONS: [&str; 1] = ["--help"];
    // Remove the bad options.
    for bad in BAD_OPTIONS {
        vecdata.retain(|value| *value != bad);
    }

    // Build the command.
    let mut cmd = Command::new("podman");
    for v in vecdata.iter() {
        cmd.arg(v);
    }

    // Check if command went ok or not.
    match cmd.output() {
        Ok(cmd_ok) => {
            // check if info has been return on stderr, command do not generate error code correctly.
            if !cmd_ok.stdout.is_empty() {
                // Build answer.
                let data = json!(
                    {
                        "Code": "201",
                        "Info": "Successfully created pod",
                        "ID": format!("{}", String::from_utf8_lossy(&cmd_ok.stdout).trim_end()),
                        "Status": "Created"
                    }
                );

                // Get USER-AGENT from request header, ugly but works.
                let mut ua_string = String::new();
                for v in reqdata.headers().get_all(USER_AGENT) {
                    ua_string = format!("{:?}",v);
                };

                // Vec for HttpRequest data to log.
                // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
                let vlogdata = vec![
                    reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                    reqdata.connection_info().scheme().to_string(),
                    reqdata.path().to_string(),
                    reqdata.connection_info().host().to_string(),
                    ua_string
                ];

                // Get log function and put requierd data into it.
                let vlog: Vec<String> = logs::log_data(201,"Created",0,"POST",vlogdata);
                // Extra create data for log.
                let cdata = format!("{:?}",String::from_utf8_lossy(&cmd_ok.stdout).trim_end());
                // Send information to log.
                let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={} oCMAPICreateID={} oCMAPICreateType=Pod",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9],cdata);
                let _ = logs::send_logs(logdata);

                // Fetch headers.
                let vheaders = api_headers();

                // Return answer.
                Ok( HttpResponse::Created()
                    .append_header(("api-version",vheaders[0].clone()))
                    .content_type(vheaders[1].clone())
                    .json(data) )
            }
            else {
                // Construct JSON object
                let data = json!(
                    {
                        "Code": 400,
                        "Info": format!("{}", String::from_utf8_lossy(&cmd_ok.stderr).trim_end().replace("\n", ", ")),
                        "Status": "Bad Request"
                    }
                );

                // Get USER-AGENT from request header, ugly but works.
                let mut ua_string = String::new();
                for v in reqdata.headers().get_all(USER_AGENT) {
                    ua_string = format!("{:?}",v);
                };

                // Vec for HttpRequest data to log.
                // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
                let vlogdata = vec![
                    reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                    reqdata.connection_info().scheme().to_string(),
                    reqdata.path().to_string(),
                    reqdata.connection_info().host().to_string(),
                    ua_string
                ];

                // Get log function and put requierd data into it.
                let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"POST",vlogdata);
                // Send information to log.
                let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
                let _ = logs::send_logs(logdata);

                // Fetch headers.
                let vheaders = api_headers();

                // Return answer.
                Ok( HttpResponse::BadRequest()
                    .append_header(("api-version",vheaders[0].clone()))
                    .content_type(vheaders[1].clone())
                    .json(data) )
            }
        },
        // When error build response.
        Err(cmd_err) => {
            // Construct JSON object
            let data = json!(
                {
                    "Code": 500,
                    "Info": format!("{}", cmd_err).trim_end(),
                    "Status": "Internal Server Error"
                }
            );

            // Get USER-AGENT from request header, ugly but works.
            let mut ua_string = String::new();
            for v in reqdata.headers().get_all(USER_AGENT) {
                ua_string = format!("{:?}",v);
            };

            // Vec for HttpRequest data to log.
            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
            let vlogdata = vec![
                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                reqdata.connection_info().scheme().to_string(),
                reqdata.path().to_string(),
                reqdata.connection_info().host().to_string(),
                ua_string
            ];

            // Get log function and put requierd data into it.
            let vlog: Vec<String> = logs::log_data(500,"Internal Server Error",6,"POST",vlogdata);
            // Send information to log.
            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
            let _ = logs::send_logs(logdata);

            // Fetch headers.
            let vheaders = api_headers();

            // Return answer.
            Ok( HttpResponse::InternalServerError()
                .append_header(("api-version",vheaders[0].clone()))
                .content_type(vheaders[1].clone())
                .json(data) )
        }
    }
}

// Post pods setstate data.
pub async fn post_pods_setstate(sdata: web::Json<StatePod>,reqdata: HttpRequest) -> io::Result<HttpResponse> {
    // Get JSON data from post.
    let jdata = sdata.into_inner();

    // Check and block invalid name format and options.
    let name_input: String = match jdata.name.as_str() {
        s if s.starts_with("-") => { "ERROR".to_string() },
        s if s.contains(" ") => { "ERROR".to_string() },
        "" => { "ERROR".to_string() },
        _ => { jdata.state.to_string() }
    };

    // Check state from input, must match given or get ERROR as result.
    let arg_input: String = match jdata.state.to_lowercase().as_str() {
        "start" => { jdata.state.to_string() },
        "stop" => { jdata.state.to_string() },
        "pause" => { jdata.state.to_string() },
        "unpause" => { jdata.state.to_string() },
        "restart" => { jdata.state.to_string() },
        _ => { "ERROR".to_string() }
    };

    // Check if ERROR is assigned based on input param given.
    if arg_input == "ERROR" || name_input == "ERROR" {
        // Construct JSON object
        let data = json!(
            {
                "Code": 400,
                "Info": "State/Name provided is not correct format or value",
                "Status": "Bad Request"
            }
        );

        // Get USER-AGENT from request header, ugly but works.
        let mut ua_string = String::new();
        for v in reqdata.headers().get_all(USER_AGENT) {
            ua_string = format!("{:?}",v);
        };

        // Vec for HttpRequest data to log.
        // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
        let vlogdata = vec![
            reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
            reqdata.connection_info().scheme().to_string(),
            reqdata.path().to_string(),
            reqdata.connection_info().host().to_string(),
            ua_string
        ];

        // Get log function and put requierd data into it.
        let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"POST",vlogdata);
        // Send information to log.
        let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
        let _ = logs::send_logs(logdata);

        // Fetch headers.
        let vheaders = api_headers();

        // Return answer.
        return Ok( HttpResponse::BadRequest()
            .append_header(("api-version",vheaders[0].clone()))
            .content_type(vheaders[1].clone())
            .json(data) );
    }

    // The command.
    let cmd = Command::new("podman")
        .arg("pod")
        .arg(arg_input.to_lowercase().clone())
        .arg(jdata.name.clone())
        .output();

    // Check if command went ok or not.
    match cmd {
        Ok(cmd_ok) => {
            // Check length of stdout, returns empty response when no pods matched state set.
            let count = cmd_ok.stdout.len();
            if count > 4 {
                // Clean data from unneeded characters.
                let result =  format!("{}", String::from_utf8_lossy(&cmd_ok.stdout))
                    .trim_start()
                    .trim_end()
                    .replace("\n", "")
                    .to_string();

                // Construct JSON object.
                let data = json!(
                    {
                        "Code": 200,
                        "Info": format!("Sucessfully set Pod '{}' state to '{}'",result,jdata.state.to_lowercase()),
                        "Status": "OK" 
                    }
                );

                // Get USER-AGENT from request header, ugly but works.
                let mut ua_string = String::new();
                for v in reqdata.headers().get_all(USER_AGENT) {
                    ua_string = format!("{:?}",v);
                };

                // Vec for HttpRequest data to log.
                // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
                let vlogdata = vec![
                    reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                    reqdata.connection_info().scheme().to_string(),
                    reqdata.path().to_string(),
                    reqdata.connection_info().host().to_string(),
                    ua_string
                ];

                // Get log function and put requierd data into it.
                let vlog: Vec<String> = logs::log_data(200,"Request OK",0,"POST",vlogdata);
                // Send information to log.
                let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={} oCMAPIPodName={} oCMAPISetState={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9],jdata.name,jdata.state);
                let _ = logs::send_logs(logdata);

                // Fetch headers.
                let vheaders = api_headers();

                // Return answer.
                Ok( HttpResponse::Ok()
                    .append_header(("api-version",vheaders[0].clone()))
                    .content_type(vheaders[1].clone())
                    .json(data) )
            }
            // Return status when no pods are matching state.
            else {
                // Construct JSON object
                let data = json!(
                    {
                        "Code": 400,
                        "Info": format!("{}",String::from_utf8_lossy(&cmd_ok.stderr).replace("\n", "").replace("\t", "")),
                        "Status": "Bad Request"
                    }
                );

                // Get USER-AGENT from request header, ugly but works.
                let mut ua_string = String::new();
                for v in reqdata.headers().get_all(USER_AGENT) {
                    ua_string = format!("{:?}",v);
                };

                // Vec for HttpRequest data to log.
                // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
                let vlogdata = vec![
                    reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                    reqdata.connection_info().scheme().to_string(),
                    reqdata.path().to_string(),
                    reqdata.connection_info().host().to_string(),
                    ua_string
                ];

                // Get log function and put requierd data into it.
                let vlog: Vec<String> = logs::log_data(404,"Not Found",0,"POST",vlogdata);
                // Send information to log.
                let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
                let _ = logs::send_logs(logdata);

                // Fetch headers.
                let vheaders = api_headers();

                // Return answer.
                Ok( HttpResponse::BadRequest()
                    .append_header(("api-version",vheaders[0].clone()))
                    .content_type(vheaders[1].clone())
                    .json(data) )
            }
        }
        Err(cmd_err) => {
            // Construct JSON object
            let data = json!(
                {
                    "Code": 500,
                    "Info": format!("{}", cmd_err),
                    "Status": "Internal Server Error"
                }
            );

            // Get USER-AGENT from request header, ugly but works.
            let mut ua_string = String::new();
            for v in reqdata.headers().get_all(USER_AGENT) {
                ua_string = format!("{:?}",v);
            };

            // Vec for HttpRequest data to log.
            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
            let vlogdata = vec![
                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                reqdata.connection_info().scheme().to_string(),
                reqdata.path().to_string(),
                reqdata.connection_info().host().to_string(),
                ua_string
            ];

            // Get log function and put requierd data into it.
            let vlog: Vec<String> = logs::log_data(500,"Internal Server Error",6,"POST",vlogdata);
            // Send information to log.
            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
            let _ = logs::send_logs(logdata);

            // Fetch headers.
            let vheaders = api_headers();

            // Return answer.
            Ok( HttpResponse::InternalServerError()
                .append_header(("api-version",vheaders[0].clone()))
                .content_type(vheaders[1].clone())
                .json(data) )
        }
    }
}

// Delete Pod data.
pub async fn delete_pods_data(sdata: web::Json<DeletePod>,reqdata: HttpRequest) -> io::Result<HttpResponse> {
    // Get JSON data from post.
    let jdata = sdata.into_inner();

    // Regex for non allowed characters.
    let rx_name = Regex::new(r"([^A-Za-z0-9_-])").unwrap();

    // Check if regex matches anything in name.
    if rx_name.find(jdata.name.as_str()).is_some() {
        // Construct JSON object
        let data = json!(
            {
                "Code": 400,
                "Info": "Name provided is not in correct format",
                "Status": "Bad Request"
            }
        );

        // Get USER-AGENT from request header, ugly but works.
        let mut ua_string = String::new();
        for v in reqdata.headers().get_all(USER_AGENT) {
            ua_string = format!("{:?}",v);
        };

        // Vec for HttpRequest data to log.
        // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
        let vlogdata = vec![
            reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
            reqdata.connection_info().scheme().to_string(),
            reqdata.path().to_string(),
            reqdata.connection_info().host().to_string(),
            ua_string
        ];

        // Get log function and put requierd data into it.
        let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"POST",vlogdata);
        // Send information to log.
        let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
        let _ = logs::send_logs(logdata);

        // Fetch headers.
        let vheaders = api_headers();

        // Return answer.
        return Ok( HttpResponse::BadRequest()
            .append_header(("api-version",vheaders[0].clone()))
            .content_type(vheaders[1].clone())
            .json(data) );
    }

    // Check if secret isn´t empty and not matching key from config.
    if jdata.key.is_empty() || jdata.key != PODS_KEY.get().unwrap().to_string() {
        // Construct JSON object
        let data = json!(
            {
                "Code": 400,
                "Info": "Missing or incorrect data",
                "Status": "Bad Request"
            }
        );

        // Get USER-AGENT from request header, ugly but works.
        let mut ua_string = String::new();
        for v in reqdata.headers().get_all(USER_AGENT) {
            ua_string = format!("{:?}",v);
        };

        // Vec for HttpRequest data to log.
        // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
        let vlogdata = vec![
            reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
            reqdata.connection_info().scheme().to_string(),
            reqdata.path().to_string(),
            reqdata.connection_info().host().to_string(),
            ua_string
        ];

        // Get log function and put requierd data into it.
        let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"DELETE",vlogdata);
        // Send information to log.
        let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
        let _ = logs::send_logs(logdata);

        // Fetch headers.
        let vheaders = api_headers();

        // Return answer.
        return Ok( HttpResponse::BadRequest()
            .append_header(("api-version",vheaders[0].clone()))
            .content_type(vheaders[1].clone())
            .json(data) );
    }

    // Match key to make sure it is correct.
    if jdata.key == PODS_KEY.get().unwrap().to_string() {
        // Command.
        let cmd_get= format!("podman pod ps --format='{{{{ .NumberOfContainers }}}}' --filter name='{}' --ctr-names",jdata.name);
        let output = fake_tty::bash_command(cmd_get.as_str()).unwrap()
            .output()
            .expect("Number of attached containers...");

        // Get data and convert.
        let strsum = fake_tty::get_stdout(output.stdout).unwrap_or_default();
        let getsum: i32 = strsum.trim().parse().unwrap_or_default();

        // Only remove when there are one container attached, should be the *infra*.
        if getsum == 1 {
            // Command
            let cmd_remove = Command::new("podman")
            .arg("pod")
            .arg("rm")
            .arg(jdata.name.clone())
            .output()?;

            // Check result.
            match cmd_remove.status.code() {
                Some(0) => {
                    // Construct JSON object.
                    let data = json!(
                        {
                            "Code": 200,
                            "Name": format!("Pod '{}' has been deleted",jdata.name),
                            "Status": "Ok"
                        }
                    );
                    // Get USER-AGENT from request header, ugly but works.
                    let mut ua_string = String::new();
                    for v in reqdata.headers().get_all(USER_AGENT) {
                        ua_string = format!("{:?}",v);
                    };

                    // Vec for HttpRequest data to log.
                    // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
                    let vlogdata = vec![
                        reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                        reqdata.connection_info().scheme().to_string(),
                        reqdata.path().to_string(),
                        reqdata.connection_info().host().to_string(),
                        ua_string
                    ];

                    // Get log function and put requierd data into it.
                    let vlog: Vec<String> = logs::log_data(200,"Request OK",0,"DELETE",vlogdata);
                    // Send information to log.
                    let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={} oCMAPIDeleteID={} oCMAPIDeleteType=pod",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9],jdata.name);
                    let _ = logs::send_logs(logdata);

                    // Fetch headers.
                    let vheaders = api_headers();

                    // Return answer.
                    return Ok( HttpResponse::Ok()
                        .append_header(("api-version",vheaders[0].clone()))
                        .content_type(vheaders[1].clone())
                        .json(data) );
                },
                _ => {
                    // Construct JSON object
                    let data = json!(
                        {
                            "Code": 400,
                            "Info": format!("Cannot remove pod '{}'",jdata.name),
                            "Status": "Bad Request",
                        }
                    );
                    // Get USER-AGENT from request header, ugly but works.
                    let mut ua_string = String::new();
                    for v in reqdata.headers().get_all(USER_AGENT) {
                        ua_string = format!("{:?}",v);
                    };

                    // Vec for HttpRequest data to log.
                    // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
                    let vlogdata = vec![
                        reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                        reqdata.connection_info().scheme().to_string(),
                        reqdata.path().to_string(),
                        reqdata.connection_info().host().to_string(),
                        ua_string
                    ];

                    // Get log function and put requierd data into it.
                    let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"GET",vlogdata);
                    // Send information to log.
                    let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
                    let _ = logs::send_logs(logdata);

                    // Fetch headers.
                    let vheaders = api_headers();

                    // Return answer.
                    return Ok( HttpResponse::BadRequest()
                        .append_header(("api-version",vheaders[0].clone()))
                        .content_type(vheaders[1].clone())
                        .json(data) );
                }
            }
        }
        else {
            // Construct JSON object
            let data = json!(
                {
                    "Code": 400,
                    "Info": format!("Cannot remove pod '{}' check your data",jdata.name),
                    "Status": "Bad Request",
                }
            );

            // Get USER-AGENT from request header, ugly but works.
            let mut ua_string = String::new();
            for v in reqdata.headers().get_all(USER_AGENT) {
                ua_string = format!("{:?}",v);
            };

            // Vec for HttpRequest data to log.
            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
            let vlogdata = vec![
                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                reqdata.connection_info().scheme().to_string(),
                reqdata.path().to_string(),
                reqdata.connection_info().host().to_string(),
                ua_string
            ];

            // Get log function and put requierd data into it.
            let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"GET",vlogdata);
            // Send information to log.
            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
            let _ = logs::send_logs(logdata);

            // Fetch headers.
            let vheaders = api_headers();

            // Return answer.
            return Ok( HttpResponse::BadRequest()
                .append_header(("api-version",vheaders[0].clone()))
                .content_type(vheaders[1].clone())
                .json(data) );
        }
    }
    else {
        // Construct JSON object
        let data = json!(
            {
                "Code": 400,
                "Info": "Missing or incorrect data",
                "Status": "Bad Request"
            }
        );

        // Get USER-AGENT from request header, ugly but works.
        let mut ua_string = String::new();
        for v in reqdata.headers().get_all(USER_AGENT) {
            ua_string = format!("{:?}",v);
        };

        // Vec for HttpRequest data to log.
        // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
        let vlogdata = vec![
            reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
            reqdata.connection_info().scheme().to_string(),
            reqdata.path().to_string(),
            reqdata.connection_info().host().to_string(),
            ua_string
        ];

        // Get log function and put requierd data into it.
        let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"DELETE",vlogdata);
        // Send information to log.
        let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
        let _ = logs::send_logs(logdata);

        // Fetch headers.
        let vheaders = api_headers();

        // Return answer.
        return Ok( HttpResponse::BadRequest()
            .append_header(("api-version",vheaders[0].clone()))
            .content_type(vheaders[1].clone())
            .json(data) );
    }
}

/* --- Networks API --- */

// Get network information.
pub async fn get_networks_info(reqdata: HttpRequest) -> io::Result<HttpResponse> {
    // The command.
    let cmd = Command::new("podman")
        .arg("network")
        .arg("ls")
        .arg(
            "--format='
            { \"Name\": {{json .Name}},
            \"ID\": {{json .ID}},
            \"Driver\": {{json .Driver}},
            \"Created\": {{json .Created}} },'"
        )
        .output();

    // Check if command went ok or not.
    match cmd {
        // When Ok build response.
        Ok(cmd_ok) => {
            // Clean data from unneeded characters.
            let data = format!("[{}]", String::from_utf8_lossy(&cmd_ok.stdout))
                .trim_start()
                .trim_end()
                .to_string()
                .replace("'", "")
                .replace(",\n]", "]");

            // Get USER-AGENT from request header, ugly but works.
            let mut ua_string = String::new();
            for v in reqdata.headers().get_all(USER_AGENT) {
                ua_string = format!("{:?}",v);
            };

            // Vec for HttpRequest data to log.
            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
            let vlogdata = vec![
                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                reqdata.connection_info().scheme().to_string(),
                reqdata.path().to_string(),
                reqdata.connection_info().host().to_string(),
                ua_string
            ];

            // Get log function and put requierd data into it.
            let vlog: Vec<String> = logs::log_data(200,"Request OK",0,"GET",vlogdata);
            // Send information to log.
            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
            let _ = logs::send_logs(logdata);

            // Fetch headers.
            let vheaders = api_headers();

            // Return answer.
            Ok( HttpResponse::Ok()
                .append_header(("api-version",vheaders[0].clone()))
                .content_type(vheaders[1].clone())
                .body(data) )
        },
        // When error build response.
        Err(cmd_err) => {
            // Construct JSON object
            let data = json!(
                {
                    "Code": 400,
                    "Info": format!("{}", cmd_err),
                    "Status": "Bad Request"
                }
            );

            // Get USER-AGENT from request header, ugly but works.
            let mut ua_string = String::new();
            for v in reqdata.headers().get_all(USER_AGENT) {
                ua_string = format!("{:?}",v);
            };

            // Vec for HttpRequest data to log.
            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
            let vlogdata = vec![
                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                reqdata.connection_info().scheme().to_string(),
                reqdata.path().to_string(),
                reqdata.connection_info().host().to_string(),
                ua_string
            ];

            // Get log function and put requierd data into it.
            let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"GET",vlogdata);
            // Send information to log.
            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
            let _ = logs::send_logs(logdata);

            // Fetch headers.
            let vheaders = api_headers();

            // Return answer.
            Ok( HttpResponse::InternalServerError()
                .append_header(("api-version",vheaders[0].clone()))
                .content_type(vheaders[1].clone())
                .json(data) )
        }
    }
}

// Get single network information.
pub async fn get_networks_info_single(param: web::Path<String>,reqdata: HttpRequest) -> io::Result<HttpResponse> {
     // The command.
    let cmd = Command::new("podman")
        .arg("network")
        .arg("inspect")
        .arg(format!("{}", param))
        .arg("--format=json")
        .output();

    // Check if command went ok or not.
    match cmd {
        Ok(cmd_ok) => {
            // Clean data from unneeded characters.
            let data = format!("{}", String::from_utf8_lossy(&cmd_ok.stdout));

            // Check length of stdout, returns empty response when no network is found.
            let count = cmd_ok.stdout.len();
            if count > 4 {

                // Get USER-AGENT from request header, ugly but works.
                let mut ua_string = String::new();
                for v in reqdata.headers().get_all(USER_AGENT) {
                    ua_string = format!("{:?}",v);
                };

                // Vec for HttpRequest data to log.
                // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
                let vlogdata = vec![
                    reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                    reqdata.connection_info().scheme().to_string(),
                    reqdata.path().to_string(),
                    reqdata.connection_info().host().to_string(),
                    ua_string
                ];

                // Get log function and put requierd data into it.
                let vlog: Vec<String> = logs::log_data(200,"Request OK",0,"GET",vlogdata);
                // Send information to log.
                let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
                let _ = logs::send_logs(logdata);

                // Fetch headers.
                let vheaders = api_headers();

                // Return answer.
                Ok( HttpResponse::Ok()
                    .append_header(("api-version",vheaders[0].clone()))
                    .content_type(vheaders[1].clone())
                    .body(data) )
            }
            else {
                // Build JSON response.
                let data = json!(
                    {
                        "Code": 400,
                        "Info": "No network found with that name",
                        "Status": "Bad Request"
                    }
                );

                // Get USER-AGENT from request header, ugly but works.
                let mut ua_string = String::new();
                for v in reqdata.headers().get_all(USER_AGENT) {
                    ua_string = format!("{:?}",v);
                };

                // Vec for HttpRequest data to log.
                // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
                let vlogdata = vec![
                    reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                    reqdata.connection_info().scheme().to_string(),
                    reqdata.path().to_string(),
                    reqdata.connection_info().host().to_string(),
                    ua_string
                ];

                // Get log function and put requierd data into it.
                let vlog: Vec<String> = logs::log_data(400,"Bad Request",0,"GET",vlogdata);
                // Send information to log.
                let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
                let _ = logs::send_logs(logdata);

                // Fetch headers.
                let vheaders = api_headers();

                // Return answer.
                Ok( HttpResponse::BadRequest()
                    .append_header(("api-version",vheaders[0].clone()))
                    .content_type(vheaders[1].clone())
                    .json(data) )
            }

        }
        Err(cmd_err) => {
            // Construct JSON object
            let data = json!(
                {
                    "Code": 500,
                    "Info": format!("{}", cmd_err),
                    "Status": "Internal Server Error"
                }
            );

            // Get USER-AGENT from request header, ugly but works.
            let mut ua_string = String::new();
            for v in reqdata.headers().get_all(USER_AGENT) {
                ua_string = format!("{:?}",v);
            };

            // Vec for HttpRequest data to log.
            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
            let vlogdata = vec![
                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                reqdata.connection_info().scheme().to_string(),
                reqdata.path().to_string(),
                reqdata.connection_info().host().to_string(),
                ua_string
            ];

            // Get log function and put requierd data into it.
            let vlog: Vec<String> = logs::log_data(500,"Internal Server Error",4,"GET",vlogdata);
            // Send information to log.
            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
            let _ = logs::send_logs(logdata);

            // Fetch headers.
            let vheaders = api_headers();

            // Return answer.
            Ok( HttpResponse::BadRequest()
                .append_header(("api-version",vheaders[0].clone()))
                .content_type(vheaders[1].clone())
                .json(data) )
        }
    }
}

// Post networks create data.
pub async fn post_networks_create(cdata: web::Json<CreateNetwork>,reqdata: HttpRequest) -> io::Result<HttpResponse> {
    // Get JSON data from post.
    let jdata = cdata.into_inner();

    // Create empty array
    let mut vecdata: Vec<String> = Vec::new();

    // Check if required json tags has values.
    if jdata.name.len() > 2 {
        // Fill vec.
        vecdata.push("network".to_string());
        vecdata.push("create".to_string());
        vecdata.push(jdata.name);

        // Check if requirements are meet.
        if jdata.options.len() > 4 && jdata.options.starts_with("--") {
            // iterate through options tag and split back to vec.
            for v in jdata.options.split(",-").map(String::from) {
                if v.starts_with("--") {
                    vecdata.push(v);
                }
                else {
                    vecdata.push(format!("-{}", v));
                }
            }
        }
        // if requirements is in wrong format, error out.
        else if !jdata.options.is_empty() {
            // Construct JSON object
            let data = json!(
                {
                    "Code": 400,
                    "Info": "Check options tag, must be at least 4 characters long and start with -- to be valid",
                    "Status": "Bad Request"
                }
            );

            // Get USER-AGENT from request header, ugly but works.
            let mut ua_string = String::new();
            for v in reqdata.headers().get_all(USER_AGENT) {
                ua_string = format!("{:?}",v);
            };

            // Vec for HttpRequest data to log.
            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
            let vlogdata = vec![
                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                reqdata.connection_info().scheme().to_string(),
                reqdata.path().to_string(),
                reqdata.connection_info().host().to_string(),
                ua_string
            ];

            // Get log function and put requierd data into it.
            let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"POST",vlogdata);
            // Send information to log.
            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
            let _ = logs::send_logs(logdata);

            // Fetch headers.
            let vheaders = api_headers();

            // Return answer.
            return Ok( HttpResponse::BadRequest()
                .append_header(("api-version",vheaders[0].clone()))
                .content_type(vheaders[1].clone())
                .json(data));
        }
    }
    // if requirements are not met, error out.
    else {

        // Construct JSON object
        let data = json!(
            {
                "Code": 400,
                "Info": "Required tags are missing information",
                "Status": "Bad Request"
            }
        );

        // Get USER-AGENT from request header, ugly but works.
        let mut ua_string = String::new();
        for v in reqdata.headers().get_all(USER_AGENT) {
            ua_string = format!("{:?}",v);
        };

        // Vec for HttpRequest data to log.
        // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
        let vlogdata = vec![
            reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
            reqdata.connection_info().scheme().to_string(),
            reqdata.path().to_string(),
            reqdata.connection_info().host().to_string(),
            ua_string
        ];

        // Get log function and put requierd data into it.
        let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"POST",vlogdata);
        // Send information to log.
        let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
        let _ = logs::send_logs(logdata);

        // Fetch headers.
        let vheaders = api_headers();

        // Return answer.
        return Ok(HttpResponse::BadRequest()
            .append_header(("api-version",vheaders[0].clone()))
            .content_type(vheaders[1].clone())
            .json(data));
    }

    //  Const with bad options.
    const BAD_OPTIONS: [&str; 2] = ["--help", "--ignore"];
    // Remove the bad options.
    for bad in BAD_OPTIONS {
        vecdata.retain(|value| *value != bad);
    }

    // Build the command.
    let mut cmd = Command::new("podman");
    for v in &vecdata {
        cmd.arg(format!("{}",v));
    }

    // Check if command went ok or not.
    match cmd.output() {
        Ok(cmd_ok) => {
            // check if info has been return on stderr, command do not generate error code correctly.
            if !cmd_ok.stdout.is_empty() {
                // Build answer.
                let data = json!(
                    {
                        "Code": "201",
                        "Info": "Successfully created network",
                        "ID": format!("{}", String::from_utf8_lossy(&cmd_ok.stdout).trim_end()),
                        "Status": "Created"
                    }
                );

                // Get USER-AGENT from request header, ugly but works.
                let mut ua_string = String::new();
                for v in reqdata.headers().get_all(USER_AGENT) {
                    ua_string = format!("{:?}",v);
                };

                // Vec for HttpRequest data to log.
                // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
                let vlogdata = vec![
                    reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                    reqdata.connection_info().scheme().to_string(),
                    reqdata.path().to_string(),
                    reqdata.connection_info().host().to_string(),
                    ua_string
                ];

                // Get log function and put requierd data into it.
                let vlog: Vec<String> = logs::log_data(201,"Created",0,"POST",vlogdata);
                // Extra create data for log.
                let cdata = format!("{:?}",String::from_utf8_lossy(&cmd_ok.stdout).trim_end());
                // Send information to log.
                let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={} oCMAPICreateID={} oCMAPICreateType=Network",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9],cdata);
                let _ = logs::send_logs(logdata);

                // Fetch headers.
                let vheaders = api_headers();

                // Return answer.
                Ok( HttpResponse::Created()
                    .append_header(("api-version",vheaders[0].clone()))
                    .content_type(vheaders[1].clone())
                    .json(data) )
            }
            else {
                // Construct JSON object
                let data = json!(
                    {
                        "Code": 400,
                        "Info": format!("{}", String::from_utf8_lossy(&cmd_ok.stderr).trim_end().replace("\n", ", ")),
                        "Status": "Bad Request"
                    }
                );

                // Get USER-AGENT from request header, ugly but works.
                let mut ua_string = String::new();
                for v in reqdata.headers().get_all(USER_AGENT) {
                    ua_string = format!("{:?}",v);
                };

                // Vec for HttpRequest data to log.
                // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
                let vlogdata = vec![
                    reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                    reqdata.connection_info().scheme().to_string(),
                    reqdata.path().to_string(),
                    reqdata.connection_info().host().to_string(),
                    ua_string
                ];

                // Get log function and put requierd data into it.
                let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"POST",vlogdata);
                // Send information to log.
                let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
                let _ = logs::send_logs(logdata);

                // Fetch headers.
                let vheaders = api_headers();

                // Return answer.
                Ok( HttpResponse::BadRequest()
                    .append_header(("api-version",vheaders[0].clone()))
                    .content_type(vheaders[1].clone())
                    .json(data) )
            }
        },
        // When error build response.
        Err(cmd_err) => {
            // Construct JSON object
            let data = json!(
                {
                    "Code": 500,
                    "Info": format!("{}", cmd_err).trim_end(),
                    "Status": "Internal Server Error"
                }
            );

            // Get USER-AGENT from request header, ugly but works.
            let mut ua_string = String::new();
            for v in reqdata.headers().get_all(USER_AGENT) {
                ua_string = format!("{:?}",v);
            };

            // Vec for HttpRequest data to log.
            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
            let vlogdata = vec![
                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                reqdata.connection_info().scheme().to_string(),
                reqdata.path().to_string(),
                reqdata.connection_info().host().to_string(),
                ua_string
            ];

            // Get log function and put requierd data into it.
            let vlog: Vec<String> = logs::log_data(500,"Internal Server Error",6,"POST",vlogdata);
            // Send information to log.
            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
            let _ = logs::send_logs(logdata);

            // Fetch headers.
            let vheaders = api_headers();

            // Return answer.
            Ok( HttpResponse::InternalServerError()
                .append_header(("api-version",vheaders[0].clone()))
                .content_type(vheaders[1].clone())
                .json(data) )
        }
    }
}

// Delete networks
pub async fn delete_networks_data(sdata: web::Json<DeleteNetwork>, reqdata: HttpRequest) -> io::Result<HttpResponse> {
    // Get JSON data from post.
    let jdata = sdata.into_inner();

    // Regex for non allowed characters.
    let rx_name = Regex::new(r"([^A-Za-z0-9_-])").unwrap();

    // Check if regex matches anything in name.
    if rx_name.find(jdata.name.as_str()).is_some() {
        // Construct JSON object
        let data = json!(
            {
                "Code": 400,
                "Info": "Name provided is not in correct format",
                "Status": "Bad Request"
            }
        );

        // Get USER-AGENT from request header, ugly but works.
        let mut ua_string = String::new();
        for v in reqdata.headers().get_all(USER_AGENT) {
            ua_string = format!("{:?}",v);
        };

        // Vec for HttpRequest data to log.
        // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
        let vlogdata = vec![
            reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
            reqdata.connection_info().scheme().to_string(),
            reqdata.path().to_string(),
            reqdata.connection_info().host().to_string(),
            ua_string
        ];

        // Get log function and put requierd data into it.
        let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"POST",vlogdata);
        // Send information to log.
        let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
        let _ = logs::send_logs(logdata);

        // Fetch headers.
        let vheaders = api_headers();

        // Return answer.
        return Ok( HttpResponse::BadRequest()
            .append_header(("api-version",vheaders[0].clone()))
            .content_type(vheaders[1].clone())
            .json(data) );
    }

    // Check if secret isn´t empty and not matching key from config.
    if jdata.key.is_empty() || jdata.key != NETWORKS_KEY.get().unwrap().to_string() {
        // Construct JSON object
        let data = json!(
            {
                "Code": 400,
                "Info": "Missing or incorrect data",
                "Status": "Bad Request"
            }
        );

        // Get USER-AGENT from request header, ugly but works.
        let mut ua_string = String::new();
        for v in reqdata.headers().get_all(USER_AGENT) {
            ua_string = format!("{:?}",v);
        };

        // Vec for HttpRequest data to log.
        // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
        let vlogdata = vec![
            reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
            reqdata.connection_info().scheme().to_string(),
            reqdata.path().to_string(),
            reqdata.connection_info().host().to_string(),
            ua_string
        ];

        // Get log function and put requierd data into it.
        let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"DELETE",vlogdata);
        // Send information to log.
        let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
        let _ = logs::send_logs(logdata);

        // Fetch headers.
        let vheaders = api_headers();

        // Return answer.
        return Ok( HttpResponse::BadRequest()
            .append_header(("api-version",vheaders[0].clone()))
            .content_type(vheaders[1].clone())
            .json(data) );
    }

    // Match key to make sure it is correct.
    if jdata.key == NETWORKS_KEY.get().unwrap().to_string() {
        // Command
        let cmd_connected = Command::new("podman")
            .arg("network")
            .arg("inspect")
            .arg(jdata.name.clone())
            .arg("--format='{{json .Containers }}'")
            .output();

        // Check if network has containers attached.
        match cmd_connected {
            Ok (cmd_ok) => {
                // Check length of stdout, returns empty response when no container is connected.
                let count = cmd_ok.stdout.len();
                if count > 6 {
                    // Build JSON response.
                    let data = json!(
                        {
                            "Code": 400,
                            "Info": format!("Network '{}' has containers attached or do not exist",jdata.name),
                            "Status": "Bad Request"
                        }
                    );

                    // Get USER-AGENT from request header, ugly but works.
                    let mut ua_string = String::new();
                    for v in reqdata.headers().get_all(USER_AGENT) {
                        ua_string = format!("{:?}",v)
                    };

                    // Vec for HttpRequest data to log.
                    // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
                    let vlogdata = vec![
                        reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                        reqdata.connection_info().scheme().to_string(),
                        reqdata.path().to_string(),
                        reqdata.connection_info().host().to_string(),
                        ua_string
                    ];

                    // Get log function and put requierd data into it.
                    let vlog: Vec<String> = logs::log_data(400,"Bad Request",0,"GET",vlogdata);
                    // Send information to log.
                    let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
                    let _ = logs::send_logs(logdata);

                    // Fetch headers.
                    let vheaders = api_headers();

                    // Return answer.
                    return Ok( HttpResponse::BadRequest()
                        .append_header(("api-version",vheaders[0].clone()))
                        .content_type(vheaders[1].clone())
                        .json(data) );
                }
            },
            Err(cmd_err) => {
                // Construct JSON object
                let data = json!(
                    {
                        "Code": 400,
                        "Info": format!("{}", cmd_err),
                        "Status": "Bad Request",
                    }
                );

                // Get USER-AGENT from request header, ugly but works.
                let mut ua_string = String::new();
                for v in reqdata.headers().get_all(USER_AGENT) {
                    ua_string = format!("{:?}",v);
                };

                // Vec for HttpRequest data to log.
                // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
                let vlogdata = vec![
                    reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                    reqdata.connection_info().scheme().to_string(),
                    reqdata.path().to_string(),
                    reqdata.connection_info().host().to_string(),
                    ua_string
                ];

                // Get log function and put requierd data into it.
                let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"GET",vlogdata);
                // Send information to log.
                let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
                let _ = logs::send_logs(logdata);

                // Fetch headers.
                let vheaders = api_headers();

                // Return answer.
                return Ok( HttpResponse::BadRequest()
                    .append_header(("api-version",vheaders[0].clone()))
                    .content_type(vheaders[1].clone())
                    .json(data) );
            }
        };

        // Remove network if not attached.
        let cmd_remove = Command::new("podman")
            .arg("network")
            .arg("rm")
            .arg(jdata.name.clone())
            .output()?;

            // Check result.
            match cmd_remove.status.code() {
            Some(0) => {
                // Construct JSON object.
                let data = json!(
                    {
                        "Code": 200,
                        "Info": format!("Network '{}' has been deleted",jdata.name),
                        "Status": "OK"
                    }
                );

                // Get USER-AGENT from request header, ugly but works.
                let mut ua_string = String::new();
                for v in reqdata.headers().get_all(USER_AGENT) {
                    ua_string = format!("{:?}",v);
                };

                // Vec for HttpRequest data to log.
                // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
                let vlogdata = vec![
                    reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                    reqdata.connection_info().scheme().to_string(),
                    reqdata.path().to_string(),
                    reqdata.connection_info().host().to_string(),
                    ua_string
                ];

                // Get log function and put requierd data into it.
                let vlog: Vec<String> = logs::log_data(200,"Request OK",0,"DELETE",vlogdata);
                // Send information to log.
                let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={} oCMAPIDeleteID={} oCMAPIDeleteType=network",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9],jdata.name);
                let _ = logs::send_logs(logdata);

                // Fetch headers.
                let vheaders = api_headers();

                // Return answer.
                return Ok( HttpResponse::Ok()
                    .append_header(("api-version",vheaders[0].clone()))
                    .content_type(vheaders[1].clone())
                    .json(data) );
            },
            _ => {
            // Construct JSON object
            let data = json!(
                {
                    "Code": 400,
                    "Info": format!("Cannot remove network '{}', still connected or do not exist",jdata.name),
                    "Status": "Bad Request",
                }
            );

            // Get USER-AGENT from request header, ugly but works.
            let mut ua_string = String::new();
            for v in reqdata.headers().get_all(USER_AGENT) {
                ua_string = format!("{:?}",v);
            };

            // Vec for HttpRequest data to log.
            // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
            let vlogdata = vec![
                reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
                reqdata.connection_info().scheme().to_string(),
                reqdata.path().to_string(),
                reqdata.connection_info().host().to_string(),
                ua_string
            ];

            // Get log function and put requierd data into it.
            let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"GET",vlogdata);
            // Send information to log.
            let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
            let _ = logs::send_logs(logdata);

            // Fetch headers.
            let vheaders = api_headers();

            // Return answer.
            return Ok( HttpResponse::BadRequest()
                .append_header(("api-version",vheaders[0].clone()))
                .content_type(vheaders[1].clone())
                .json(data) );
            }
        }
    }
    else {
        // Construct JSON object
        let data = json!(
            {
                "Code": 400,
                "Info": "Missing or incorrect data",
                "Status": "Bad Request"
            }
        );

        // Get USER-AGENT from request header, ugly but works.
        let mut ua_string = String::new();
        for v in reqdata.headers().get_all(USER_AGENT) {
            ua_string = format!("{:?}",v);
        };

        // Vec for HttpRequest data to log.
        // 0 = src, 1 = scheme, 2 = path, 3 = request, 4 = requestClientApplication
        let vlogdata = vec![
            reqdata.connection_info().peer_addr().unwrap_or("0.0.0.0").to_string(),
            reqdata.connection_info().scheme().to_string(),
            reqdata.path().to_string(),
            reqdata.connection_info().host().to_string(),
            ua_string
        ];

        // Get log function and put requierd data into it.
        let vlog: Vec<String> = logs::log_data(400,"Bad Request",4,"DELETE",vlogdata);
        // Send information to log.
        let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
        let _ = logs::send_logs(logdata);

        // Fetch headers.
        let vheaders = api_headers();

        // Return answer.
        return Ok( HttpResponse::BadRequest()
            .append_header(("api-version",vheaders[0].clone()))
            .content_type(vheaders[1].clone())
            .json(data) );
    }
}