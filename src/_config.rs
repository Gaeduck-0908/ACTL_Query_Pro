use std::collections::HashMap;

pub struct Config {
    pub selected_profile: Option<String>,
    pub profiles: HashMap<String, serde_json::Value>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            selected_profile: None,
            profiles: HashMap::new(),
        }
    }

    pub fn load_profiles(&mut self, config_file: &str) -> Result<(), String> {
        let file_content = std::fs::read_to_string(config_file)
            .map_err(|e| format!("Failed to read config file: {}", e))?;

        let profiles: HashMap<String, serde_json::Value> = serde_json::from_str(&file_content)
            .map_err(|e| format!("Failed to parse config file: {}", e))?;

        self.profiles = profiles;
        Ok(())
    }

    pub fn set_selected_profile(&mut self, profile_name: &str) {
        self.selected_profile = Some(profile_name.to_string());
    }

    pub fn get_selected_profile(&self) -> String {
        match &self.selected_profile {
            Some(profile) => format!("[{}]", profile),
            None => String::from("[None]"),
        }
    }
}
