// Load crates
use std::fs::{File};
use std::io::Write;
use chrono::{DateTime, Local};
use gethostname::gethostname;
// Load local modules
use crate::config::{LOG_PATH,SERVER_IP,SERVER_PORT};

pub fn log_data(code: i32,status: &str, severity: i8, method: &str, vdata: Vec<String>) -> Vec<String> {
    // Vec for collecting log information from HttpRequest
    let mut vlogdata: Vec<String> = Vec::new();

    // (index[0]) - Start of log 
    vlogdata.push(format!("CEF:1|oCMAPI|API|1.0|{}|{}|{}|startTime=<starttime>",code,status,severity));

    // (index[1]) - value for "src"
    vlogdata.push(vdata[0].to_string());

    // (index[2]) - value for "proto", is a static value.
    vlogdata.push("TCP".to_string());

    // (index[3]) - value for "scheme"
    vlogdata.push(vdata[1].to_string());

    // (index[4]) - value for "dst" Gives server adress assigned in config for now...
    vlogdata.push(SERVER_IP.get().unwrap().to_string());
    // (index[5]) - Value for "dpt"
    vlogdata.push(SERVER_PORT.get().unwrap().to_string());

    // (index[6]) - value for "path"
    vlogdata.push(vdata[2].to_string());

    // (index[7]) - value for "requestMethod".
    vlogdata.push(method.to_string());

    // (index[8]) - value for "request"
    vlogdata.push(vdata[3].to_string());

    // (index[9]) - value for "requestClientApplication" (User-Agent), ugly but works.
    vlogdata.push(vdata[4].to_string());

    // Return data.
    vlogdata
}

// Function for sending log data to file.
pub fn send_logs(logdata: String) -> std::io::Result<()> {
    //Get current Date & Time.
    let date_time_now: DateTime<Local> = Local::now();
    // Date format for log filename.
    let log_file_date = date_time_now.format("%Y%m%d");
    // Date format for syslog prefix in log data.
    let syslog_datetime = date_time_now.format("%b %e %T");
    // Get Hostname
    let host_name = gethostname().to_str().unwrap_or("ErrorNoHostname").to_string();
    // Log file name.
    //let log_path: String;
    // Get log path and check if empty.
    let log_path: String = if !LOG_PATH.get().unwrap().trim().is_empty() {
        // Puts logs in assigned log path.
        //log_path = 
        format!("{}/ocmapi_{}.log", LOG_PATH.get().unwrap().trim(),log_file_date)
    }
    else {
        // Puts logs in same directory as the binary.
        //log_path = 
        format!("./ocmapi_{}.log",log_file_date)
    };
    // Construct log start.
    let log_start = format!("{} {}",syslog_datetime,host_name);
    // Replace certain tag with real world values.
    let raw_log = logdata
        .replace("<starttime>", &date_time_now.to_string());
    // Complete log data.
    let log_data = format!("{} {}",log_start,raw_log);
    // Write to log.
    let mut log_file = File::options().append(true).create(true).open(log_path)?;
    writeln!(&mut log_file,"{}", log_data)?;
    Ok(())
}