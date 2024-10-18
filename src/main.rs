mod _config;
mod run_query;
mod aws_register;
mod aws_update;
mod aws_delete;
mod aws_show;
mod query_register;
mod query_update;
mod query_delete;
mod query_show;
mod error_handler;

use _config::Config;
use colored::*;
use std::io::{self};
use std::panic;
// Import the colored crate

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

    let config = match Config::new() {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("{}", format!("Failed to initialize config: {}", e).red());
            return;
        }
    };

    loop {
        println!("\n{}", "Select an option:".bold().cyan());
        println!("{}", "1. Run Query".green());
        println!("{}", "2. AWS Config".green());
        println!("{}", "3. Query Config".green());
        println!("{}", "4. Exit".green());

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");

        match choice.trim() {
            "1" => {
                if let Err(e) = run_query::run(&config) { // Pass the config to run_query
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
                println!("{}", "Exiting...".yellow());
                break;
            }
            _ => println!("{}", "Invalid choice! Please select 1, 2, 3, or 4.".red()),
        }
    }
}

fn aws_config_menu() {
    println!("\n{}", "AWS Config:".bold().cyan());
    println!("{}", "1. AWS Register".green());
    println!("{}", "2. AWS Update".green());
    println!("{}", "3. AWS Delete".green());
    println!("{}", "4. AWS Show".green());

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
        _ => println!("{}", "Invalid choice! Please select 1, 2, 3, or 4.".red()),
    }
}

fn query_config_menu() {
    println!("\n{}", "Query Config:".bold().cyan());
    println!("{}", "1. Query Register".green());
    println!("{}", "2. Query Update".green());
    println!("{}", "3. Query Delete".green());
    println!("{}", "4. Query Show".green());

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
        _ => println!("{}", "Invalid choice! Please select 1, 2, 3, or 4.".red()),
    }
}
