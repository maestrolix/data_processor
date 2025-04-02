use core::models::{DataTypeConfig, PluginTypeConfig};
use diesel::{insert_into, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use std::collections::HashMap;

use crate::{
    db_connection::connection,
    models::{
        DataType, NewDataType, NewPluginType, NewPluginTypeDataType, UpdateDataType,
        UpdatePluginType,
    },
};

pub fn updating_data_types(types: Vec<DataTypeConfig>) {
    use crate::schema::data_types;

    let keys_db: Vec<String> = data_types::table
        .select(data_types::key)
        .get_results(&mut connection())
        .unwrap();

    let mut new_types = vec![];

    for data_type in types {
        if !keys_db.contains(&data_type.key) {
            new_types.push(NewDataType {
                key: data_type.key,
                description: Some(data_type.description),
                title: data_type.title,
            });
        } else {
            diesel::update(data_types::table)
                .filter(data_types::key.eq(data_type.key))
                .set(&UpdateDataType {
                    description: Some(data_type.description),
                    title: data_type.title,
                })
                .execute(&mut connection())
                .unwrap();
        }
    }

    insert_into(data_types::table)
        .values(new_types)
        .execute(&mut connection())
        .unwrap();
}

pub fn updating_plugin_types(types: Vec<PluginTypeConfig>) {
    use crate::schema::{data_types, plugin_types, plugin_types_data_types};

    let data_types_list: Vec<DataType> = data_types::table
        .select(DataType::as_select())
        .get_results(&mut connection())
        .unwrap();
    let hash_map_data_types: HashMap<String, i32> =
        HashMap::from_iter(data_types_list.iter().map(|dt| (dt.key.clone(), dt.id)));

    let keys_db: Vec<String> = plugin_types::table
        .select(plugin_types::key)
        .get_results(&mut connection())
        .unwrap();

    for plugin_type in types {
        if !keys_db.contains(&plugin_type.key) {
            let new_plugin_type = NewPluginType {
                key: plugin_type.key,
                description: Some(plugin_type.description),
                title: plugin_type.title,
            };
            let plugin_type_id: i32 = insert_into(plugin_types::table)
                .values(new_plugin_type)
                .returning(plugin_types::id)
                .get_result(&mut connection())
                .unwrap();

            let mut plugin_fields = vec![];

            for field in plugin_type.input_fields {
                plugin_fields.push(NewPluginTypeDataType {
                    is_input: true,
                    is_required: true,
                    plugin_type_id,
                    data_type_id: *hash_map_data_types.get(&field.key).unwrap(),
                });
            }

            for field in plugin_type.output_fields {
                plugin_fields.push(NewPluginTypeDataType {
                    is_input: false,
                    is_required: true,
                    plugin_type_id,
                    data_type_id: *hash_map_data_types.get(&field.key).unwrap(),
                });
            }

            insert_into(plugin_types_data_types::table)
                .values(plugin_fields)
                .execute(&mut connection())
                .unwrap();
        } else {
            let plugin_type_id = diesel::update(plugin_types::table)
                .filter(plugin_types::key.eq(plugin_type.key))
                .set(&UpdatePluginType {
                    description: Some(plugin_type.description),
                    title: plugin_type.title,
                })
                .returning(plugin_types::id)
                .get_result(&mut connection())
                .unwrap();

            diesel::delete(plugin_types_data_types::table)
                .filter(plugin_types_data_types::plugin_type_id.eq(plugin_type_id))
                .execute(&mut connection())
                .unwrap();

            let mut plugin_fields = vec![];

            for field in plugin_type.input_fields {
                plugin_fields.push(NewPluginTypeDataType {
                    is_input: true,
                    is_required: true,
                    plugin_type_id,
                    data_type_id: *hash_map_data_types.get(&field.key).unwrap(),
                });
            }

            for field in plugin_type.output_fields {
                plugin_fields.push(NewPluginTypeDataType {
                    is_input: false,
                    is_required: true,
                    plugin_type_id,
                    data_type_id: *hash_map_data_types.get(&field.key).unwrap(),
                });
            }

            insert_into(plugin_types_data_types::table)
                .values(plugin_fields)
                .execute(&mut connection())
                .unwrap();
        }
    }
}
