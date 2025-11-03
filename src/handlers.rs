// Load crates
use actix_web::{http::{header::{ContentType, USER_AGENT}, StatusCode}, web::{self}, HttpRequest, HttpResponse};
use std::{ process::Command };
use serde_json::{ json };
use std::io::{ self };
use include_dir::{include_dir, Dir};
// Load local modules
use crate::models::{ CreateContainer, CreateNetwork, CreatePod, StateContainer, StatePod };
use crate::logs;

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

    // Add correct headers.
    let ctype = "text/html; charset=utf-8";
    // Send response.
    Ok( HttpResponse::build(StatusCode::OK).content_type(ctype).body(html_body) )
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

                // Add correct headers.
                let add_headers = ContentType::json();

                // Return answer.
                Ok( HttpResponse::Ok().insert_header(add_headers).body(data) )
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
                        "Code": "404",
                        "Message": "No stats available, No containers running?",
                        "Error": "Empty Respons"
                    }
                );

                // Add correct headers.
                let add_headers = ContentType::json();

                // Return answer.
                Ok( HttpResponse::NotFound().insert_header(add_headers).json(data) )
            }
        },
        // When error build response.
        Err(cmd_err) => {
            // Construct JSON object
            let data = json!(
                {
                    "Code": 500,
                    "Message": "Internal Server Error: Could not process data",
                    "Error": format!("{}", cmd_err),
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

            // Return answer.
            Ok( HttpResponse::InternalServerError().json(data) )
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

            // Add correct headers.
            let add_headers = ContentType::json();

            // Return answer.
            Ok( HttpResponse::Ok().insert_header(add_headers).body(data) )
        },
        // When error build response.
        Err(cmd_err) => {
            // Construct JSON object
            let data = json!(
                {
                    "Code": 400,
                    "Message": "Bad Request: Could not process data",
                    "Error": format!("{}", cmd_err)
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

            // Return answer.
            Ok( HttpResponse::BadRequest().json(data) )
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

            // Add correct headers.
            let add_headers = ContentType::json();

            // Return answer.
            Ok( HttpResponse::Ok().insert_header(add_headers).body(data) )
        },
        // When error build response.
        Err(cmd_err) => {
            // Construct JSON object
            let data = json!(
                {
                    "Code": 400,
                    "Message": "Bad Request: Could not process data",
                    "Error": format!("{}", cmd_err),
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

            // Return answer.
            Ok( HttpResponse::BadRequest().json(data) )
        }
    }
}

/* --- Containers API --- */

// Get all containers status information.
pub async fn get_containers_status(reqdata: HttpRequest) -> io::Result<HttpResponse> {
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

            // Add correct headers.
            let add_headers = ContentType::json();

            // Return answer.
            Ok( HttpResponse::Ok().insert_header(add_headers).body(data) )
        },
        // When error build response.
        Err(cmd_err) => {
            // Construct JSON object
            let data = json!(
                {
                    "Code": 400,
                    "Message": "Bad Request: Could not process data",
                    "Error": format!("{}", cmd_err),
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

            // Return answer.
            Ok( HttpResponse::InternalServerError().json(data) )
        }
    }
}

// Get containers with a specific status.
pub async fn get_containers_status_query(param: web::Path<String>,reqdata: HttpRequest) -> io::Result<HttpResponse> {
    // Create arg_input variable.
    #[allow(unused_assignments)]
    let mut arg_input= String::new();
    // Check param from input, must match.
    match param.as_str() {
        "created" => { arg_input = format!("status={}", param); },
        "exited" => { arg_input = format!("status={}", param); },
        "paused" => { arg_input = format!("status={}", param); },
        "running" => { arg_input = format!("status={}", param); },
        "unknown" => { arg_input = format!("status={}", param); },
        _ => { arg_input = format!("status={}", "ERROR"); },
    }

    // Check if ERROR based on input param assignment.
    if arg_input == "status=ERROR" {
        // Construct JSON object
        let data = json!(
            {
                "Code": 400,
                "Message": "Bad Request: not a valid status option",
                "Error": "Invalid Options"
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
        // Return answer.
        return Ok( HttpResponse::BadRequest().json(data) );
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
                // Add correct headers.
                let add_headers = ContentType::json();
                // Return answer.
                Ok( HttpResponse::Ok().insert_header(add_headers).body(data) )
            }
            // Return error since no containers are running.
            else {
                // Construct JSON object
                let data = json!(
                    [
                        {
                            "Code": 204,
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
                let vlog: Vec<String> = logs::log_data(204,"No Content",0,"GET",vlogdata);
                // Send information to log.
                let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
                let _ = logs::send_logs(logdata);
                // Add correct headers.
                let add_headers = ContentType::json();
                // Return answer.
                Ok( HttpResponse::NoContent().insert_header(add_headers).json(data) )
            }
        }
        Err(cmd_err) => {
            // Construct JSON object
            let data = json!(
                {
                    "Code": 500,
                    "Message": "Internal Server Error: Could not process data",
                    "Error": format!("{}", cmd_err),
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

            // Return answer.
            Ok( HttpResponse::InternalServerError().json(data) )
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
                    "Message": "Check options tag, must be at least 4 characters long and start with -- to be valid",
                    "Error": "Invalid Options"
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

            // Return answer.
            return Ok(HttpResponse::BadRequest().json(data));
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
                "Message": "Bad Request: Required tags are missing information",
                "Error": "Missing Information"
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

        // Return answer.
        return Ok(HttpResponse::BadRequest().json(data));
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
                        "Status": "201",
                        "Message": "Successfully created container",
                        "ID": format!("{}", String::from_utf8_lossy(&cmd_ok.stdout).trim_end())
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

                // Add correct headers.
                let add_headers = ContentType::json();

                // Return answer.
                Ok( HttpResponse::Created().insert_header(add_headers).json(data) )
            }
            else {
                // Construct JSON object
                let data = json!(
                    {
                        "Code": 400,
                        "Message": "Bad Request: Could not create container",
                        "Error": format!("{}", String::from_utf8_lossy(&cmd_ok.stderr).trim_end().replace("\n", ", "))
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

                // Return answer.
                Ok( HttpResponse::BadRequest().json(data) )
            }
        },
        // When error build response.
        Err(cmd_err) => {
            // Construct JSON object
            let data = json!(
                {
                    "Code": 500,
                    "Message": "Internal Server Error: Could not process data",
                    "Error": format!("{}", cmd_err).trim_end(),
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

            // Return answer.
            Ok( HttpResponse::InternalServerError().json(data) )
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
    let arg_input: String = match jdata.state.as_str() {
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
                "Message": "Bad Request: State/Name provided is not in correct format or a valid state",
                "Error": "Invalid Information"
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

        // Return answer.
        return Ok( HttpResponse::BadRequest().json(data) );
    }

    // The command.
    let cmd = Command::new("podman")
        .arg("container")
        .arg(arg_input.clone())
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
                        "Name": result,
                        "State": jdata.state
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

                // Add correct headers.
                let add_headers = ContentType::json();
                // Return answer.
                Ok( HttpResponse::Ok().insert_header(add_headers).json(data) )
            }
            // Return status when no containers are matching state.
            else {
                // Construct JSON object
                let data = json!(
                    [
                        {
                            "Code": 204,
                            "Info": format!("No container handled based on given input state: {}",arg_input),
                            "State": "None"
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
                let vlog: Vec<String> = logs::log_data(204,"No Content",0,"POST",vlogdata);
                // Send information to log.
                let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
                let _ = logs::send_logs(logdata);

                // Add correct headers.
                let add_headers = ContentType::json();
                // Return answer.
                Ok( HttpResponse::Ok().insert_header(add_headers).json(data) )
            }
        }
        Err(cmd_err) => {
            // Construct JSON object
            let data = json!(
                {
                    "Code": 500,
                    "Message": "Internal Server Error: Could not process data",
                    "Error": format!("{}", cmd_err),
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

            // Return answer.
            Ok( HttpResponse::InternalServerError().json(data) )
        }
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
            \"Created\": {{json .Status}} },'"
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

            // Add correct headers.
            let add_headers = ContentType::json();

            // Return answer.
            Ok( HttpResponse::Ok().insert_header(add_headers).body(data) )
        },
        // When error build response.
        Err(cmd_err) => {
            // Construct JSON object
            let data = json!(
                {
                    "Code": 400,
                    "Message": "Bad Request: Could not process data",
                    "Error": format!("{}", cmd_err),
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

            // Return answer.
            Ok( HttpResponse::InternalServerError().json(data) )
        }
    }
}

// Get Pods with a specific status.
pub async fn get_pods_status_query(param: web::Path<String>,reqdata: HttpRequest) -> io::Result<HttpResponse> {
    // Create arg_input variable.
    #[allow(unused_assignments)]
    let mut arg_input= String::new();
    // Check param from input, must match.
    match param.as_str() {
        "created" => { arg_input = format!("status={}", param); },
        "exited" => { arg_input = format!("status={}", param); },
        "paused" => { arg_input = format!("status={}", param); },
        "running" => { arg_input = format!("status={}", param); },
        "unknown" => { arg_input = format!("status={}", param); },
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
                        "Message": "Bad Request: Could not process data",
                        "Error": "Bad Status"
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

                // Return answer.
                Ok( HttpResponse::BadRequest().json(data) )
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

                    // Add correct headers.
                    let add_headers = ContentType::json();

                    // Return answer.
                    Ok( HttpResponse::Ok().insert_header(add_headers).body(data) )
                }
                // Return error since no containers are running.
                else {
                    let data = json!(
                        [
                            {
                                "Code": 204,
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
                    let vlog: Vec<String> = logs::log_data(204,"No Content",0,"GET",vlogdata);
                    // Send information to log.
                    let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
                    let _ = logs::send_logs(logdata);

                    // Add correct headers.
                    let add_headers = ContentType::json();
                    
                    // Return answer.
                    Ok( HttpResponse::NoContent().insert_header(add_headers).json(data) )
                }
            }
        }
        Err(cmd_err) => {
            // Construct JSON object
            let data = json!(
                {
                    "Code": 500,
                    "Message": "Internal Server Error: Could not process data",
                    "Error": format!("{}", cmd_err),
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

            // Return answer.
            Ok( HttpResponse::InternalServerError().json(data) )
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
                    "Message": "Check options tag, must be at least 4 characters long and start with -- to be valid",
                    "Error": "Invalid Options"
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

            // Return answer.
            return Ok(HttpResponse::BadRequest().json(data));
        }
    }
    // if requirements are not met, error out.
    else {
        // Construct JSON object
        let data = json!(
            {
                "Code": 400,
                "Message": "Bad Request: Required tags are missing information",
                "Error": "Missing Information"
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

        // Return answer.
        return Ok(HttpResponse::BadRequest().json(data));
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
                        "Status": "201",
                        "Message": "Successfully created pod",
                        "ID": format!("{}", String::from_utf8_lossy(&cmd_ok.stdout).trim_end()),
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

                // Add correct headers.
                let add_headers = ContentType::json();

                // Return answer.
                Ok( HttpResponse::Created().insert_header(add_headers).json(data) )
            }
            else {
                // Construct JSON object
                let data = json!(
                    {
                        "Code": 400,
                        "Message": "Bad Request: Could not create pod",
                        "Error": format!("{}", String::from_utf8_lossy(&cmd_ok.stderr).trim_end().replace("\n", ", ")),
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

                // Return answer.
                Ok( HttpResponse::BadRequest().json(data) )
            }
        },
        // When error build response.
        Err(cmd_err) => {
            // Construct JSON object
            let data = json!(
                {
                    "Code": 500,
                    "Message": "Internal Server Error: Could not process data",
                    "Error": format!("{}", cmd_err).trim_end(),
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

            // Return answer.
            Ok( HttpResponse::InternalServerError().json(data) )
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
    let arg_input: String = match jdata.state.as_str() {
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
                "Message": "Bad Request: State/Name provided is not correct format or value",
                "Error": "Invalid Value"
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

        // Return answer.
        return Ok( HttpResponse::BadRequest().json(data) );
    }

    // The command.
    let cmd = Command::new("podman")
        .arg("pod")
        .arg(arg_input.clone())
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
                        "Name": result,
                        "State": jdata.state
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

                // Add correct headers.
                let add_headers = ContentType::json();
                // Return answer.
                Ok( HttpResponse::Ok().insert_header(add_headers).json(data) )
            }
            // Return status when no pods are matching state.
            else {
                // Construct JSON object
                let data = json!(
                    {
                        "Code": 204,
                        "Info": format!("No pod handled based on given input state: {}",arg_input),
                        "State": "None"
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
                let vlog: Vec<String> = logs::log_data(204,"No Content",0,"POST",vlogdata);
                // Send information to log.
                let logdata = format!("{} src={} proto={} scheme={} dst={} dpt={} path={} requestMethod={} Request={} requestClientApplication={}",vlog[0],vlog[1],vlog[2],vlog[3],vlog[4],vlog[5],vlog[6],vlog[7],vlog[8],vlog[9]);
                let _ = logs::send_logs(logdata);

                // Add correct headers.
                let add_headers = ContentType::json();
                // Return answer.
                Ok( HttpResponse::Ok().insert_header(add_headers).json(data) )
            }
        }
        Err(cmd_err) => {
            // Construct JSON object
            let data = json!(
                {
                    "Code": 500,
                    "Message": "Internal Server Error: Could not send data",
                    "Error": format!("{}", cmd_err),
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

            // Return answer.
            Ok( HttpResponse::InternalServerError().json(data) )
        }
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

            // Add correct headers.
            let add_headers = ContentType::json();

            // Return answer.
            Ok( HttpResponse::Ok().insert_header(add_headers).body(data) )
        },
        // When error build response.
        Err(cmd_err) => {
            // Construct JSON object
            let data = json!(
                {
                    "Code": 400,
                    "Message": "Bad Request: Could not process data",
                    "Error": format!("{}", cmd_err),
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

            // Return answer.
            Ok( HttpResponse::InternalServerError().json(data) )
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

                // Add correct headers.
                let add_headers = ContentType::json();

                // Return answer.
                Ok( HttpResponse::Ok().insert_header(add_headers).body(data) )
            }
            else {

                // Build JSON response.
                let data = json!(
                    {
                        "Code": 400,
                        "Message": "Bad Request: No network found with that name",
                        "Error": "Empty Respons"
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

                // Add correct headers.
                let add_headers = ContentType::json();

                // Return answer.
                Ok( HttpResponse::BadRequest().insert_header(add_headers).json(data) )
            }

        }
        Err(cmd_err) => {
            // Construct JSON object
            let data = json!(
                {
                    "Code": 500,
                    "Message": "Internal Server Error: Could not process data",
                    "Error": format!("{}", cmd_err),
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

            // Return answer.
            Ok( HttpResponse::BadRequest().json(data) )
        }
    }
}

// Post pod create data.
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
                    "Message": "Check options tag, must be at least 4 characters long and start with -- to be valid",
                    "Error": "Invalid Options"
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

            // Return answer.
            return Ok( HttpResponse::BadRequest().json(data));
        }
    }
    // if requirements are not met, error out.
    else {

        // Construct JSON object
        let data = json!(
            {
                "Code": 400,
                "Message": "Bad Request: Required tags are missing information",
                "Error": "Missing Information"
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

        // Return answer.
        return Ok(HttpResponse::BadRequest().json(data));
    }

    //  Const with bad options.
    const BAD_OPTIONS: [&str; 2] = ["--help", "--ignore"];
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
                        "Status": "201",
                        "Message": "Successfully created network",
                        "ID": format!("{}", String::from_utf8_lossy(&cmd_ok.stdout).trim_end())
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

                // Add correct headers.
                let add_headers = ContentType::json();

                // Return answer.
                Ok( HttpResponse::Created().insert_header(add_headers).json(data) )
            }
            else {
                // Construct JSON object
                let data = json!(
                    {
                        "Code": 400,
                        "Message": "Bad Request: Could not create network",
                        "Error": format!("{}", String::from_utf8_lossy(&cmd_ok.stderr).trim_end().replace("\n", ", ")),
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

                // Return answer.
                Ok( HttpResponse::BadRequest().json(data) )
            }
        },
        // When error build response.
        Err(cmd_err) => {
            // Construct JSON object
            let data = json!(
                {
                    "Code": 500,
                    "Message": "Internal Server Error: Could not process data",
                    "Error": format!("{}", cmd_err).trim_end(),
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

            // Return answer.
            Ok( HttpResponse::InternalServerError().json(data) )
        }
    }
}
