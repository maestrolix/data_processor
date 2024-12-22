use dotenv_config::EnvConfig;

#[derive(Debug, EnvConfig, Clone)]
pub struct Kafka {
    #[env_config(name = "KAFKA_HOST")]
    pub host: String,
    #[env_config(name = "KAFKA_PORT")]
    pub port: String,
    #[env_config(name = "KAFKA_TOPIC_INPUT")]
    pub topic_input: String,
    #[env_config(name = "KAFKA_TOPIC_OUTPUT")]
    pub topic_output: String,
}
