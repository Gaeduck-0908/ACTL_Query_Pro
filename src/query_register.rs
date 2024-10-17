use serde_json::{json, Value};
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

pub fn register() -> Result<(), String> {
    let query_dir = "query";
    let query_file = format!("{}/query_data.json", query_dir);

    if !Path::new(query_dir).exists() {
        fs::create_dir(query_dir).map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    let query_name = prompt_user("Enter the query name: ")?;
    if query_name.is_empty() {
        return Err("Query name cannot be empty.".to_string());
    }

    let query_content = prompt_user("Enter the query content: ")?;
    if query_content.is_empty() {
        return Err("Query content cannot be empty.".to_string());
    }

    let mut query_data = load_query_file(&query_file)?;

    query_data[&query_name] = json!({
        "query_content": query_content,
    });

    save_to_query_file(&query_file, query_data)?;

    println!("Query '{}' registered successfully.", query_name);
    Ok(())
}

fn prompt_user(prompt: &str) -> Result<String, String> {
    print!("{}", prompt);
    io::stdout().flush().map_err(|e| e.to_string())?;

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| format!("Failed to read input: {}", e))?;

    Ok(input.trim().to_string())
}

fn load_query_file(query_file: &str) -> Result<Value, String> {
    if Path::new(query_file).exists() {
        let file_content = fs::read_to_string(query_file)
            .map_err(|e| format!("Failed to read query file: {}", e))?;

        if file_content.trim().is_empty() {
            Ok(json!({}))
        } else {
            let json_data: Value = serde_json::from_str(&file_content)
                .map_err(|e| format!("Failed to parse query file: {}", e))?;
            Ok(json_data)
        }
    } else {
        Ok(json!({}))
    }
}

fn save_to_query_file(query_file: &str, query_data: Value) -> Result<(), String> {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(query_file)
        .map_err(|e| format!("Failed to open query file: {}", e))?;

    serde_json::to_writer(file, &query_data).map_err(|e| format!("Failed to write to query file: {}", e))?;
    Ok(())
}
