use crate::_config::Config;
use std::io::{self, Write};

pub fn update() -> Result<(), String> {
    let query_file = "query/query_data.json";
    let mut config = Config::new().map_err(|e| format!("Failed to create config: {}", e))?;

    config.load_queries(query_file)?;

    println!("Available Queries:");
    for (query_name, _) in &config.queries {
        println!("- {}", query_name);
    }

    let selected_query = prompt_user("Enter the query name to select (or leave empty for None): ")?;

    if !selected_query.is_empty() {
        if let Some(query_content) = config.get_query(&selected_query) {
            println!("Selected Query: {}", selected_query);
            println!("Query Content: {}", query_content);
            config.set_selected_query(&selected_query);
        } else {
            println!("Query '{}' does not exist. No query selected.", selected_query);
        }
    } else {
        println!("No query selected.");
    }

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
