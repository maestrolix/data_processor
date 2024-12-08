use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ModelData {
    pub model_path: String,
    pub model_name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Search {
    pub visual: ModelData,
    pub textual: ModelData,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FacialProcessing {
    pub detector: ModelData,
    pub recognizer: ModelData,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Model {
    pub facial_processing: FacialProcessing,
    pub search: Search,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Kafka {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub kafka: Kafka,
    pub model: Model,
}

impl Config {
    pub fn new(config_file_name: &str) -> Self {
        let config_file = std::env::var("CONFIG_FILE").unwrap_or(config_file_name.into());
        let config_file = std::path::Path::new(&config_file);
        let config = std::fs::read_to_string(config_file).unwrap();
        let config: Config = toml::from_str(&config).unwrap();
        config
    }
}
