use serde_json::Value;
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn delete() -> Result<(), String> {
    let config_file = "configure/aws_config.json";

    if !Path::new(config_file).exists() {
        return Err("Configuration file does not exist.".to_string());
    }

    let profile_name = prompt_user("Enter the profile name to delete: ")?;

    let file_content = fs::read_to_string(config_file)
        .map_err(|e| format!("Failed to read config file: {}", e))?;

    let mut config_data: Value = serde_json::from_str(&file_content)
        .map_err(|e| format!("Failed to parse config file: {}", e))?;

    if config_data.as_object().unwrap().contains_key(&profile_name) {
        config_data.as_object_mut().unwrap().remove(&profile_name);
        save_to_config_file(config_file, config_data)?;
        println!("Profile '{}' deleted successfully.", profile_name);
    } else {
        println!("Profile '{}' does not exist.", profile_name);
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

fn save_to_config_file(config_file: &str, config_data: Value) -> Result<(), String> {
    let file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(config_file)
        .map_err(|e| format!("Failed to open config file: {}", e))?;

    serde_json::to_writer(file, &config_data).map_err(|e| format!("Failed to write to config file: {}", e))?;
    Ok(())
}
