// Program Config
mod _config;

// AWS Config
mod aws_register;
mod aws_update;
mod aws_delete;

// CloudTrail Lake Query Config
mod query_register;
mod query_update;
mod query_delete;

// Error Handler
mod error_handler;

use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use std::time::SystemTime;

fn main() {
    loop {
        println!("\nSelect an option:");
        println!("1. Run Query");
        println!("2. AWS Config");
        println!("3. Query Config");
        println!("4. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");

        match choice.trim() {
            "1" => {
                // Run Query logic
                if let Err(e) = run_query() {
                    log_error(e);
                }
            }
            "2" => {
                // AWS Config logic
                aws_config_menu();
            }
            "3" => {
                // Query Config logic
                query_config_menu();
            }
            "4" => {
                // Exit
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice! Please select 1, 2, 3, 4."),
        }
    }
}

// Function to run the query
fn run_query() -> Result<(), String> {
    // Load and execute the query (use query_change.rs for the loaded query)
    // Placeholder logic - Replace with actual query execution logic
    println!("Running current query...");
    // Simulating an error for demo purposes
    Err("Query execution failed".to_string())
}

// AWS Config Menu
fn aws_config_menu() {
    println!("\nAWS Config:");
    println!("1. AWS Register");
    println!("2. AWS Update");
    println!("3. AWS Delete");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");

    match choice.trim() {
        "1" => {
            if let Err(e) = aws_register::register() {
                log_error(e);
            }
        }
        "2" => {
            if let Err(e) = aws_update::update() {
                log_error(e);
            }
        }
        "3" => {
            if let Err(e) = aws_delete::delete() {
                log_error(e);
            }
        }
        _ => println!("Invalid choice! Please select 1, 2, or 3."),
    }
}

// Query Config Menu
fn query_config_menu() {
    println!("\nQuery Config:");
    println!("1. Query Register");
    println!("2. Query Update");
    println!("3. Query Delete");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");

    match choice.trim() {
        "1" => {
            if let Err(e) = query_register::register() {
                log_error(e);
            }
        }
        "2" => {
            if let Err(e) = query_update::update() {
                log_error(e);
            }
        }
        "3" => {
            if let Err(e) = query_delete::delete() {
                log_error(e);
            }
        }
        _ => println!("Invalid choice! Please select 1, 2, or 3."),
    }
}

// Function to log errors into logs/ directory with timestamp
fn log_error(err_msg: String) {
    let now = SystemTime::now();
    let datetime: chrono::DateTime<chrono::Utc> = now.into();
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