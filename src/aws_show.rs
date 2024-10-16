use serde_json::Value;
use std::fs;
use std::path::Path;

pub fn show() -> Result<(), String> {
    let config_file = "configure/aws_config.json";

    if !Path::new(config_file).exists() {
        return Err("Configuration file does not exist.".to_string());
    }

    let file_content = fs::read_to_string(config_file)
        .map_err(|e| format!("Failed to read config file: {}", e))?;

    let config_data: Value = serde_json::from_str(&file_content)
        .map_err(|e| format!("Failed to parse config file: {}", e))?;

    println!("Current AWS Configurations:");
    for (profile, credentials) in config_data.as_object().unwrap() {
        println!("Profile: {}", profile);
        if let Some(access_key) = credentials.get("aws_access_key_id") {
            println!("  AWS Access Key ID: {}", access_key);
        }
        if let Some(secret_key) = credentials.get("aws_secret_access_key") {
            println!("  AWS Secret Access Key: {}", secret_key);
        }
        if let Some(region) = credentials.get("region") {
            println!("  Region: {}", region);
        }
    }

    Ok(())
}
