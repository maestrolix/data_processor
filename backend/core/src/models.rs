use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DataTypeConfig {
    pub key: String,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct FieldConfig {
    pub key: String,
    pub is_required: bool,
}

#[derive(Debug, Deserialize)]
pub struct PluginTypeConfig {
    pub key: String,
    pub title: String,
    pub description: String,
    pub input_fields: Vec<FieldConfig>,
    pub output_fields: Vec<FieldConfig>,
}
