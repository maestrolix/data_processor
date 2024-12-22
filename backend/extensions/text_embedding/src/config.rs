use dotenv_config::EnvConfig;

#[derive(Debug, EnvConfig, Clone)]
pub struct TextualModel {
    #[env_config(name = "TEXTUAL_MODEL_PATH")]
    pub model_path: String,
    #[env_config(name = "TEXTUAL_MODEL_NAME")]
    pub model_name: String,
}
