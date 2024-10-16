use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use serde_json::{json, Value}; // For creating JSON objects

pub fn register() -> Result<(), String> {
    // Define the path to the configuration file
    let config_dir = "configure";
    let config_file = format!("{}/aws_config.json", config_dir);

    // Create the configuration directory if it doesn't exist
    if !Path::new(config_dir).exists() {
        fs::create_dir(config_dir).map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    // Collect profile name and AWS credentials from the user
    let profile_name = prompt_user("Enter a profile name (e.g., profile1): ")?;
    let aws_access_key_id = prompt_user("Enter your AWS Access Key ID: ")?;
    let aws_secret_access_key = prompt_user("Enter your AWS Secret Access Key: ")?;
    let region = prompt_user("Enter your AWS Region: ")?;

    // Load existing config data if the file exists
    let mut config_data = load_config_file(&config_file)?;

    // Add the new profile to the config data
    config_data[&profile_name] = json!({
        "aws_access_key_id": aws_access_key_id,
        "aws_secret_access_key": aws_secret_access_key,
        "region": region,
    });

    // Save the updated JSON data to the configuration file
    save_to_config_file(&config_file, config_data)?;

    println!("AWS configuration for profile '{}' saved successfully.", profile_name);
    Ok(())
}

// Helper function to prompt the user for input
fn prompt_user(prompt: &str) -> Result<String, String> {
    print!("{}", prompt);
    io::stdout().flush().map_err(|e| e.to_string())?;

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| format!("Failed to read input: {}", e))?;

    Ok(input.trim().to_string())
}

// Helper function to load existing configuration data from a file
fn load_config_file(config_file: &str) -> Result<Value, String> {
    if Path::new(config_file).exists() {
        let file_content = fs::read_to_string(config_file)
            .map_err(|e| format!("Failed to read config file: {}", e))?;
        let json_data: Value = serde_json::from_str(&file_content)
            .map_err(|e| format!("Failed to parse config file: {}", e))?;
        Ok(json_data)
    } else {
        Ok(json!({})) // Return an empty object if the file does not exist
    }
}

// Helper function to save JSON data to a file
fn save_to_config_file(config_file: &str, config_data: Value) -> Result<(), String> {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true) // Overwrite the file if it exists
        .open(config_file)
        .map_err(|e| format!("Failed to open config file: {}", e))?;

    serde_json::to_writer(file, &config_data).map_err(|e| format!("Failed to write to config file: {}", e))?;
    Ok(())
}