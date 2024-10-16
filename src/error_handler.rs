use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::time::SystemTime;
use std::path::Path;
use chrono::prelude::*;

pub fn log_error(err_msg: String) {
    let now = SystemTime::now();
    let datetime: DateTime<Utc> = now.into();
    let log_dir = "logs";
    let log_file_name = format!("{}/{}.log", log_dir, datetime.format("%Y-%m-%d_%H-%M-%S"));

    // Create logs directory if it doesn't exist
    if !Path::new(log_dir).exists() {
        fs::create_dir(log_dir).expect("Failed to create logs directory");
    }

    // Append to or create the log file
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file_name)
        .expect("Failed to open log file");

    writeln!(file, "[{}] ERROR: {}", datetime, err_msg).expect("Failed to write to log file");
    println!("An error occurred: {}. Check logs for details.", err_msg);
}
