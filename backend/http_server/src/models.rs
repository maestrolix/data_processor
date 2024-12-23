use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Clone, Debug)]
#[diesel(table_name = crate::schema::data_types)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DataType {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub key: String,
}

#[derive(Insertable, Serialize, Deserialize, Clone, Debug)]
#[diesel(table_name = crate::schema::data_types)]
pub struct NewDataType {
    pub title: String,
    pub description: Option<String>,
    pub key: String,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::data_types)]
pub struct UpdateDataType {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Clone, Debug)]
#[diesel(table_name = crate::schema::plugin_types)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PluginType {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub key: String,
}

#[derive(Insertable, Serialize, Deserialize, Clone, Debug)]
#[diesel(table_name = crate::schema::plugin_types)]
pub struct NewPluginType {
    pub title: String,
    pub description: Option<String>,
    pub key: String,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::plugin_types)]
pub struct UpdatePluginType {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Clone, Debug)]
#[diesel(table_name = crate::schema::plugin_types_data_types)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PluginTypeDataType {
    pub id: i32,
    pub is_input: bool,
    pub is_required: bool,
    pub plugin_type_id: i32,
    pub data_type_id: i32,
}

#[derive(Insertable, Serialize, Deserialize, Clone, Debug)]
#[diesel(table_name = crate::schema::plugin_types_data_types)]
pub struct NewPluginTypeDataType {
    pub is_input: bool,
    pub is_required: bool,
    pub plugin_type_id: i32,
    pub data_type_id: i32,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::plugin_types_data_types)]
pub struct UpdatePluginTypeDataType {
    pub is_required: bool,
}
