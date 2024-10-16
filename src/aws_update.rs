use crate::_config::Config;
use std::io::{self, Write};

pub fn update() -> Result<(), String> {
    let config_file = "configure/aws_config.json";
    let mut config = Config::new();

    config.load_profiles(config_file)?;

    println!("Available AWS Profiles:");
    for (profile_name, _) in &config.profiles {
        println!("- {}", profile_name);
    }

    let selected_profile = prompt_user("Enter the profile name to select (or leave empty for None): ")?;

    if !selected_profile.is_empty() {
        if config.profiles.contains_key(&selected_profile) {
            config.set_selected_profile(&selected_profile);
            println!("Selected Profile: {}", config.get_selected_profile());
        } else {
            println!("Profile '{}' does not exist. No profile selected.", selected_profile);
            config.set_selected_profile(""); // Set to None
        }
    } else {
        config.set_selected_profile(""); // Set to None
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
