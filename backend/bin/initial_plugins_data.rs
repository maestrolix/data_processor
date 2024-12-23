use core::models::{DataTypeConfig, PluginTypeConfig};
use http_server::services::plugin::{updating_data_types, updating_plugin_types};
use std::fs;

fn main() {
    let data_types: Vec<DataTypeConfig> =
        serde_json::from_str(&fs::read_to_string("core/types.json").unwrap()).unwrap();

    updating_data_types(data_types);

    let mut plugin_types: Vec<PluginTypeConfig> = vec![];

    let plugins_paths = fs::read_dir("extensions").unwrap();
    for plugin_path in plugins_paths {
        let mut config_file = plugin_path.unwrap().path();
        config_file.push("config.json");
        let plugin_type: PluginTypeConfig =
            serde_json::from_str(&fs::read_to_string(config_file).unwrap()).unwrap();

        plugin_types.push(plugin_type);
    }
    updating_plugin_types(plugin_types);
}
