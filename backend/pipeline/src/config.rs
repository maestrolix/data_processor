use dotenv_config::EnvConfig;

#[derive(Debug, EnvConfig, Clone)]
pub struct TextualModel {
    #[env_config(name = "TEXTUAL_MODEL_PATH")]
    pub model_path: String,
    #[env_config(name = "TEXTUAL_MODEL_NAME")]
    pub model_name: String,
}

#[derive(Debug, EnvConfig, Clone)]
pub struct VisualModel {
    #[env_config(name = "VISUAL_MODEL_PATH")]
    pub model_path: String,
    #[env_config(name = "VISUAL_MODEL_NAME")]
    pub model_name: String,
}

#[derive(Debug, EnvConfig, Clone)]
pub struct FacialDetectorModel {
    #[env_config(name = "FACIAL_DETECTOR_MODEL_PATH")]
    pub model_path: String,
    #[env_config(name = "FACIAL_DETECTOR_MODEL_NAME")]
    pub model_name: String,
}

#[derive(Debug, EnvConfig, Clone)]
pub struct FacialRecognizerModel {
    #[env_config(name = "FACIAL_RECOGNIZER_MODEL_PATH")]
    pub model_path: String,
    #[env_config(name = "FACIAL_RECOGNIZER_MODEL_NAME")]
    pub model_name: String,
}

#[derive(Debug, EnvConfig, Clone)]
pub struct Search {
    pub visual: VisualModel,
    pub textual: TextualModel,
}

#[derive(Debug, EnvConfig, Clone)]
pub struct FacialProcessing {
    pub detector: FacialDetectorModel,
    pub recognizer: FacialRecognizerModel,
}

#[derive(Debug, EnvConfig, Clone)]
pub struct Model {
    pub facial_processing: FacialProcessing,
    pub search: Search,
}

#[derive(Debug, EnvConfig, Clone)]
pub struct Kafka {
    #[env_config(name = "KAFKA_HOST")]
    pub host: String,
    #[env_config(name = "KAFKA_PORT")]
    pub port: u16,
    #[env_config(name = "KAFKA_TOPIC_INPUT")]
    pub topic_input: String,
    #[env_config(name = "KAFKA_TOPIC_OUTPUT")]
    pub topic_output: String,
}

#[derive(Debug, EnvConfig, Clone)]
pub struct Config {
    pub kafka: Kafka,
    pub model: Model,
}
