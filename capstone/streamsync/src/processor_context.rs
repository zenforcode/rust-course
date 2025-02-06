pub struct ProcessorContext {
    pub processor_name: String,
    pub config: std::collections::HashMap<String, String>,
}

impl ProcessorContext {
    pub fn new(processor_name: &str) -> Self {
        Self {
            processor_name: processor_name.to_string(),
            config: std::collections::HashMap::new(),
        }
    }

    // Add a method to set configuration properties
    pub fn set_property(&mut self, key: &str, value: &str) {
        self.config.insert(key.to_string(), value.to_string());
    }

    // Get a property from the configuration
    pub fn get_property(&self, key: &str) -> Option<&String> {
        self.config.get(key)
    }
}
