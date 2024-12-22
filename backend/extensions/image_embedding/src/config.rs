use dotenv_config::EnvConfig;

#[derive(Debug, EnvConfig, Clone)]
pub struct VisualModel {
    #[env_config(name = "VISUAL_MODEL_PATH")]
    pub model_path: String,
    #[env_config(name = "VISUAL_MODEL_NAME")]
    pub model_name: String,
}
