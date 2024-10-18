use crate::_config::Config;
// Add this for date handling
use crate::error_handler::log_error;
use chrono::Local;
use std::path::Path;
use std::process::{Command, Stdio};
// Import your error handler
use colored::*;
// Import the colored crate

pub(crate) fn run(config: &Config) -> Result<(), String> {
    // Check if a profile is selected
    let profile_name = match &config.selected_profile {
        Some(profile) => profile,
        None => {
            let error_message = "No profile selected.".to_string();
            log_error(error_message.clone());
            return Err(error_message.red().to_string()); // Color the error message
        },
    };

    // Retrieve the selected query
    let query_name = match config.get_selected_query() {
        Some(query_name) => query_name,
        None => {
            let error_message = "No query selected.".to_string();
            log_error(error_message.clone());
            return Err(error_message.red().to_string()); // Color the error message
        },
    };

    let query = match config.get_query(query_name) {
        Some(q) => q,
        None => {
            let error_message = format!("Query '{}' not found.", query_name);
            log_error(error_message.clone());
            return Err(error_message.red().to_string()); // Color the error message
        },
    };

    // Extract the query statement from the query
    let query_statement = query.get("query_statement")
        .ok_or_else(|| {
            let error_message = format!("No query statement found in query '{}'", query_name);
            log_error(error_message.clone());
            error_message.red().to_string() // Color the error message
        })?
        .as_str()
        .ok_or_else(|| {
            let error_message = format!("Query statement for '{}' is not in valid string format", query_name);
            log_error(error_message.clone());
            error_message.red().to_string() // Color the error message
        })?;

    println!("Starting query '{}' with profile '{}'", query_name, profile_name);

    // Execute the start-query command
    let command_output = Command::new("aws")
        .arg("cloudtrail")
        .arg("start-query")
        .arg("--query-statement")
        .arg(query_statement)
        .env("AWS_PROFILE", profile_name)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| {
            let error_message = format!("Failed to start query: {}", e);
            log_error(error_message.clone());
            error_message.red().to_string() // Color the error message
        })?;

    if !command_output.status.success() {
        let error_message = format!("Query execution failed: {}", String::from_utf8_lossy(&command_output.stderr));
        log_error(error_message.clone());
        return Err(error_message.red().to_string()); // Color the error message
    }

    let output_str = String::from_utf8_lossy(&command_output.stdout);
    let json_response: serde_json::Value = serde_json::from_str(&output_str)
        .map_err(|e| {
            let error_message = format!("Failed to parse query response to JSON: {}", e);
            log_error(error_message.clone());
            error_message.red().to_string() // Color the error message
        })?;

    // Extract the query ID from the response
    let query_id = json_response.get("query_id")
        .ok_or_else(|| {
            let error_message = "No query ID found in the response.".to_string();
            log_error(error_message.clone());
            error_message.red().to_string() // Color the error message
        })?
        .as_str()
        .ok_or_else(|| {
            let error_message = "Query ID is not in valid string format.".to_string();
            log_error(error_message.clone());
            error_message.red().to_string() // Color the error message
        })?;

    // Wait for the query results
    println!("Waiting for results for query ID '{}'", query_id);
    let results_command_output = Command::new("aws")
        .arg("cloudtrail")
        .arg("get-query-results")
        .arg("--query-id")
        .arg(query_id)
        .env("AWS_PROFILE", profile_name)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| {
            let error_message = format!("Failed to retrieve query results: {}", e);
            log_error(error_message.clone());
            error_message.red().to_string() // Color the error message
        })?;

    if !results_command_output.status.success() {
        let error_message = format!("Failed to get query results: {}", String::from_utf8_lossy(&results_command_output.stderr));
        log_error(error_message.clone());
        return Err(error_message.red().to_string()); // Color the error message
    }

    let results_output_str = String::from_utf8_lossy(&results_command_output.stdout);
    let json_results: serde_json::Value = serde_json::from_str(&results_output_str)
        .map_err(|e| {
            let error_message = format!("Failed to parse query results to JSON: {}", e);
            log_error(error_message.clone());
            error_message.red().to_string() // Color the error message
        })?;

    let current_date = Local::now().format("%Y-%m-%d").to_string();
    let output_filename = format!("{}_{}_{}.csv", profile_name, query_name, current_date);
    let output_path = Path::new("results").join(output_filename);

    std::fs::create_dir_all("results").map_err(|e| {
        let error_message = format!("Failed to create results directory: {}", e);
        log_error(error_message.clone());
        error_message.red().to_string() // Color the error message
    })?;

    json_to_csv(&json_results, &output_path)?;

    println!("Query results successfully written to: {}", output_path.display());

    Ok(())
}

fn json_to_csv(json_data: &serde_json::Value, output_file: &Path) -> Result<(), String> {
    let file = std::fs::File::create(output_file).map_err(|e| format!("Failed to create output file: {}", e))?;
    let mut writer = csv::Writer::from_writer(file);

    if let Some(array) = json_data.as_array() {
        if let Some(first_obj) = array.first() {
            if let Some(obj) = first_obj.as_object() {
                let headers: Vec<&str> = obj.keys().map(|k| k.as_str()).collect();
                writer.write_record(headers).map_err(|e| format!("Failed to write headers: {}", e))?;
            }
        }
        for item in array {
            if let Some(obj) = item.as_object() {
                let record: Vec<String> = obj.values().map(|v| v.to_string()).collect();
                writer.write_record(record).map_err(|e| format!("Failed to write record: {}", e))?;
            }
        }
    } else {
        return Err("Expected JSON array for query results".to_string());
    }

    writer.flush().map_err(|e| format!("Failed to flush CSV writer: {}", e))?;
    Ok(())
}
