use serde_json::Value;
use std::fs;
use std::path::Path;

pub fn show() -> Result<(), String> {
    let query_file = "query/query_data.json";

    if !Path::new(query_file).exists() {
        return Err("Query file does not exist.".to_string());
    }

    let file_content = fs::read_to_string(query_file)
        .map_err(|e| format!("Failed to read query file: {}", e))?;

    let query_data: Value = serde_json::from_str(&file_content)
        .map_err(|e| format!("Failed to parse query file: {}", e))?;

    println!("Registered Queries:");
    if let Some(queries) = query_data.as_object() {
        for (query_name, query_info) in queries {
            println!("Query Name: {}", query_name);
            if let Some(query_content) = query_info.get("query_content") {
                println!("  Query Content: {}", query_content);
            }
        }
    } else {
        println!("No queries found.");
    }

    Ok(())
}