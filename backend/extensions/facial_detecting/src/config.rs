use dotenv_config::EnvConfig;

#[derive(Debug, EnvConfig, Clone)]
pub struct FacialDetectorModel {
    #[env_config(name = "FACIAL_DETECTOR_MODEL_PATH")]
    pub model_path: String,
    #[env_config(name = "FACIAL_DETECTOR_MODEL_NAME")]
    pub model_name: String,
}
