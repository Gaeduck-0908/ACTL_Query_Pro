use std::collections::HashMap;

pub struct Config {
    pub selected_profile: Option<String>,
    pub profiles: HashMap<String, serde_json::Value>,
    pub queries: HashMap<String, serde_json::Value>,
}

impl Config {
    // Update the new method to return Result
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            selected_profile: None,
            profiles: HashMap::new(),
            queries: HashMap::new(),
        })
    }

    // Load profiles from the specified config file
    pub fn load_profiles(&mut self, config_file: &str) -> Result<(), String> {
        let file_content = std::fs::read_to_string(config_file)
            .map_err(|e| format!("Failed to read config file: {}", e))?;

        let profiles: HashMap<String, serde_json::Value> = serde_json::from_str(&file_content)
            .map_err(|e| format!("Failed to parse config file: {}", e))?;

        self.profiles = profiles;
        Ok(())
    }

    // Load queries from the specified query file
    pub fn load_queries(&mut self, query_file: &str) -> Result<(), String> {
        let file_content = std::fs::read_to_string(query_file)
            .map_err(|e| format!("Failed to read query file: {}", e))?;

        let queries: HashMap<String, serde_json::Value> = serde_json::from_str(&file_content)
            .map_err(|e| format!("Failed to parse query file: {}", e))?;

        self.queries = queries;
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

    // Add or update a profile
    pub fn add_profile(&mut self, profile_name: &str, profile_data: serde_json::Value) {
        self.profiles.insert(profile_name.to_string(), profile_data);
    }

    // Add or update a query
    pub fn add_query(&mut self, query_name: &str, query_data: serde_json::Value) {
        self.queries.insert(query_name.to_string(), query_data);
    }

    // Get query by name
    pub fn get_query(&self, query_name: &str) -> Option<&serde_json::Value> {
        self.queries.get(query_name)
    }
}