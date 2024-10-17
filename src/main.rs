mod _config;
mod aws_register;
mod aws_update;
mod aws_delete;
mod aws_show;
mod query_register;
mod query_update;
mod query_delete;
mod query_show;
mod error_handler;

use std::io::{self, Write};
use std::panic;

fn main() {

    panic::set_hook(Box::new(|panic_info| {
        let msg = match panic_info.payload().downcast_ref::<&str>() {
            Some(s) => s.to_string(),
            None => "Unknown panic occurred!".to_string(),
        };

        let location = if let Some(location) = panic_info.location() {
            format!("Panicked at {}:{}", location.file(), location.line())
        } else {
            "No information about panic location.".to_string()
        };

        error_handler::log_error(format!("{} - {}", location, msg));
    }));

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
                if let Err(e) = run_query() {
                    error_handler::log_error(e);
                }
            }
            "2" => {
                aws_config_menu();
            }
            "3" => {
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

fn run_query() -> Result<(), String> {
    println!("Running current query...");
    Err("Query execution failed".to_string())
}

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
            if let Err(e) = aws_register::register() {
                error_handler::log_error(e);
            }
        }
        "2" => {
            if let Err(e) = aws_update::update() {
                error_handler::log_error(e);
            }
        }
        "3" => {
            if let Err(e) = aws_delete::delete() {
                error_handler::log_error(e);
            }
        }
        "4" => {
            if let Err(e) = aws_show::show() {
                error_handler::log_error(e);
            }
        }
        _ => println!("Invalid choice! Please select 1, 2, 3, 4."),
    }
}

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
            if let Err(e) = query_register::register() {
                error_handler::log_error(e);
            }
        }
        "2" => {
            if let Err(e) = query_update::update() {
                error_handler::log_error(e);
            }
        }
        "3" => {
            if let Err(e) = query_delete::delete() {
                error_handler::log_error(e);
            }
        }
        "4" => {
            if let Err(e) = query_show::show() {
                error_handler::log_error(e);
            }
        }
        _ => println!("Invalid choice! Please select 1, 2, 3."),
    }
}