use serde_json::Value;
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn delete() -> Result<(), String> {
    let query_file = "query/query_data.json";

    if !Path::new(query_file).exists() {
        return Err("Query file does not exist.".to_string());
    }

    let query_name = prompt_user("Enter the query name to delete: ")?;

    let file_content = fs::read_to_string(query_file)
        .map_err(|e| format!("Failed to read query file: {}", e))?;

    let mut query_data: Value = serde_json::from_str(&file_content)
        .map_err(|e| format!("Failed to parse query file: {}", e))?;

    if query_data.as_object().unwrap().contains_key(&query_name) {
        query_data.as_object_mut().unwrap().remove(&query_name);
        save_to_query_file(query_file, query_data)?;
        println!("Query '{}' deleted successfully.", query_name);
    } else {
        println!("Query '{}' does not exist.", query_name);
    }

    Ok(())
}

fn prompt_user(prompt: &str) -> Result<String, String> {
    print!("{}", prompt);
    std::io::stdout().flush().map_err(|e| e.to_string())?;

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .map_err(|e| format!("Failed to read input: {}", e))?;

    Ok(input.trim().to_string())
}

fn save_to_query_file(query_file: &str, query_data: Value) -> Result<(), String> {
    let file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(query_file)
        .map_err(|e| format!("Failed to open query file: {}", e))?;

    serde_json::to_writer_pretty(file, &query_data).map_err(|e| format!("Failed to write to query file: {}", e))?;
    Ok(())
}