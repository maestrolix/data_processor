// @generated automatically by Diesel CLI.

diesel::table! {
    data_types (id) {
        id -> Int4,
        #[max_length = 50]
        title -> Varchar,
        description -> Nullable<Text>,
        #[max_length = 50]
        key -> Varchar,
    }
}

diesel::table! {
    input_fields (id) {
        id -> Int4,
        plugin_data_links_id -> Int4,
        data_type_id -> Int4,
        from_output_field_id -> Int4,
    }
}

diesel::table! {
    output_fields (id) {
        id -> Int4,
        plugin_data_links_id -> Int4,
        data_type_id -> Int4,
    }
}

diesel::table! {
    pipelines (id) {
        id -> Int4,
        #[max_length = 50]
        title -> Varchar,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    plugin_data_links (id) {
        id -> Int4,
    }
}

diesel::table! {
    plugin_facial_detectings (id) {
        id -> Int4,
        #[max_length = 50]
        title -> Varchar,
        description -> Nullable<Text>,
        plugin_data_links_id -> Int4,
        pipeline_id -> Int4,
    }
}

diesel::table! {
    plugin_facial_recognitions (id) {
        id -> Int4,
        #[max_length = 50]
        title -> Varchar,
        description -> Nullable<Text>,
        plugin_data_links_id -> Int4,
        pipeline_id -> Int4,
    }
}

diesel::table! {
    plugin_image_recognitions (id) {
        id -> Int4,
        #[max_length = 50]
        title -> Varchar,
        description -> Nullable<Text>,
        plugin_data_links_id -> Int4,
        pipeline_id -> Int4,
    }
}

diesel::table! {
    plugin_types (id) {
        id -> Int4,
        #[max_length = 50]
        title -> Varchar,
        description -> Nullable<Text>,
        #[max_length = 50]
        key -> Varchar,
    }
}

diesel::table! {
    plugin_types_data_types (id) {
        id -> Int4,
        is_input -> Bool,
        is_required -> Bool,
        plugin_type_id -> Int4,
        data_type_id -> Int4,
    }
}

diesel::joinable!(input_fields -> data_types (data_type_id));
diesel::joinable!(input_fields -> output_fields (from_output_field_id));
diesel::joinable!(input_fields -> plugin_data_links (plugin_data_links_id));
diesel::joinable!(output_fields -> data_types (data_type_id));
diesel::joinable!(output_fields -> plugin_data_links (plugin_data_links_id));
diesel::joinable!(plugin_facial_detectings -> pipelines (pipeline_id));
diesel::joinable!(plugin_facial_detectings -> plugin_data_links (plugin_data_links_id));
diesel::joinable!(plugin_facial_recognitions -> pipelines (pipeline_id));
diesel::joinable!(plugin_facial_recognitions -> plugin_data_links (plugin_data_links_id));
diesel::joinable!(plugin_image_recognitions -> pipelines (pipeline_id));
diesel::joinable!(plugin_image_recognitions -> plugin_data_links (plugin_data_links_id));
diesel::joinable!(plugin_types_data_types -> data_types (data_type_id));
diesel::joinable!(plugin_types_data_types -> plugin_types (plugin_type_id));

diesel::allow_tables_to_appear_in_same_query!(
    data_types,
    input_fields,
    output_fields,
    pipelines,
    plugin_data_links,
    plugin_facial_detectings,
    plugin_facial_recognitions,
    plugin_image_recognitions,
    plugin_types,
    plugin_types_data_types,
);
