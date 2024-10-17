use crate::_config::Config;
use std::io::{self, Write};

pub fn update() -> Result<(), String> {
    let query_file = "query/query_data.json";
    let mut config = Config::new();

    config.load_queries(query_file)?;

    println!("Available Queries:");
    for (query_name, _) in &config.queries {
        println!("- {}", query_name);
    }

    let selected_query = prompt_user("Enter the query name to select (or leave empty for None): ")?;

    if !selected_query.is_empty() {
        if config.queries.contains_key(&selected_query) {
            println!("Selected Query: {}", selected_query);
            let new_query_content = prompt_user("Enter the new query content: ")?;
            config.add_query(&selected_query, serde_json::json!(new_query_content));
            println!("Query '{}' updated successfully.", selected_query);
        } else {
            println!("Query '{}' does not exist. No query selected.", selected_query);
        }
    } else {
        println!("No query selected.");
    }

    save_to_query_file(query_file, &config.queries)?;

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

fn save_to_query_file(query_file: &str, queries: &std::collections::HashMap<String, serde_json::Value>) -> Result<(), String> {
    let file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(query_file)
        .map_err(|e| format!("Failed to open query file: {}", e))?;

    serde_json::to_writer_pretty(file, queries).map_err(|e| format!("Failed to write to query file: {}", e))?;
    Ok(())
}
