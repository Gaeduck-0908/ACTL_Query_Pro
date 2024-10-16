mod _config;
mod aws_register;
mod aws_update;
mod aws_delete;
mod aws_show;
mod query_register;
mod query_update; // Updated from query_change to query_update
mod query_delete;
mod query_show;
mod error_handler;

use std::io::{self, Write};
use log::error;

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
                    error_handler::log_error(e); // Call the error handler
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
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice! Please select 1, 2, 3, 4."),
        }
    }
}

// Function to run the query
fn run_query() -> Result<(), String> {
    // TODO: Load and execute the current query using the query_update module
    println!("Running current query...");
    // Placeholder error to simulate a failure
    Err("Query execution failed".to_string())
}

// AWS Config Menu
fn aws_config_menu() {
    println!("\nAWS Config:");
    println!("1. AWS Register");
    println!("2. AWS Update");
    println!("3. AWS Delete");
    println!("4. AWS SHOW");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");

    match choice.trim() {
        "1" => {
            // AWS Register
            if let Err(e) = aws_register::register() {
                error_handler::log_error(e); // Call the error handler
            }
        }
        "2" => {
            // AWS Update
            if let Err(e) = aws_update::update() {
                error_handler::log_error(e); // Call the error handler
            }
        }
        "3" => {
            // AWS Delete
            if let Err(e) = aws_delete::delete() {
                error_handler::log_error(e); // Call the error handler
            }
        }
        "4" => {
            // AWS Show
            if let Err(e) = aws_show::show() {
                error_handler::log_error(e);
            }
        }
        _ => println!("Invalid choice! Please select 1, 2, 3, 4."),
    }
}

// Query Config Menu
fn query_config_menu() {
    println!("\nQuery Config:");
    println!("1. Query Register");
    println!("2. Query Update");
    println!("3. Query Delete");
    println!("4. Query Show");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");

    match choice.trim() {
        "1" => {
            // Query Register
            if let Err(e) = query_register::register() {
                error_handler::log_error(e); // Call the error handler
            }
        }
        "2" => {
            // Query Update
            if let Err(e) = query_update::update() {
                error_handler::log_error(e); // Call the error handler
            }
        }
        "3" => {
            // Query Delete
            if let Err(e) = query_delete::delete() {
                error_handler::log_error(e); // Call the error handler
            }
        }
        "4" => {
            // Query Show
            if let Err(e) = query_show::show() {
                error_handler::log_error(e);
            }
        }
        _ => println!("Invalid choice! Please select 1, 2, 3."),
    }
}
