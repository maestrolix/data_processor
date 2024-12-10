use dotenv_config::EnvConfig;

#[derive(Debug, EnvConfig)]
pub struct TextualModel {
    #[env_config(name = "textual_model_path")]
    pub model_path: String,
    #[env_config(name = "textual_model_name")]
    pub model_name: String,
}

#[derive(Debug, EnvConfig)]
pub struct VisualModel {
    #[env_config(name = "visual_model_path")]
    pub model_path: String,
    #[env_config(name = "visual_model_name")]
    pub model_name: String,
}

#[derive(Debug, EnvConfig)]
pub struct FacialDetectorModel {
    #[env_config(name = "facial_detector_model_path")]
    pub model_path: String,
    #[env_config(name = "facial_detector_model_name")]
    pub model_name: String,
}

#[derive(Debug, EnvConfig)]
pub struct FacialRecognizerModel {
    #[env_config(name = "facial_recognizer_model_path")]
    pub model_path: String,
    #[env_config(name = "facial_recognizer_model_name")]
    pub model_name: String,
}

#[derive(Debug, EnvConfig)]
pub struct Search {
    pub visual: VisualModel,
    pub textual: TextualModel,
}

#[derive(Debug, EnvConfig)]
pub struct FacialProcessing {
    pub detector: FacialDetectorModel,
    pub recognizer: FacialRecognizerModel,
}

#[derive(Debug, EnvConfig)]
pub struct Model {
    pub facial_processing: FacialProcessing,
    pub search: Search,
}

#[derive(Debug, EnvConfig)]
pub struct Kafka {
    #[env_config(name = "kafka_host")]
    pub host: String,
    #[env_config(name = "kafka_port")]
    pub port: u16,
}

#[derive(Debug, EnvConfig)]
pub struct Config {
    pub kafka: Kafka,
    pub model: Model,
}
