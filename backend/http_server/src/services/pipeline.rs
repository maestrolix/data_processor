use axum::routing::connect;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::{
    db_connection::connection,
    models::{NewPipeline, NewPluginDecoder, Pipeline, PluginDecoder},
};

// pub struct PipelineDetail {
//     pub id: i32,
//     pub title: String,
//     pub description: Option<String>,
//     pub plugins:
// }

pub fn create_pipeline(new_pipeline: NewPipeline) -> Pipeline {
    use crate::schema::{data_types, output_fields, pipelines, plugin_data_links, plugin_decoder};

    let pipeline = diesel::insert_into(pipelines::table)
        .values(new_pipeline)
        .returning(Pipeline::as_returning())
        .get_result(&mut connection())
        .unwrap();

    let plugin_data_links_id: i32 = diesel::insert_into(plugin_data_links::table)
        .default_values()
        .returning(plugin_data_links::id)
        .get_result(&mut connection())
        .unwrap();

    let frame_type_id: i32 = data_types::table
        .filter(data_types::key.eq("frame"))
        .select(data_types::id)
        .get_result(&mut connection())
        .unwrap();

    diesel::insert_into(output_fields::table)
        .values((
            output_fields::plugin_data_links_id.eq(plugin_data_links_id),
            output_fields::data_type_id.eq(frame_type_id),
        ))
        .execute(&mut connection())
        .unwrap();

    let decoder = NewPluginDecoder {
        title: "Example title".to_string(),
        description: Some("Example description".to_string()),
        pipeline_id: pipeline.id,
        plugin_data_links_id,
    };

    let start_plugin: PluginDecoder = diesel::insert_into(plugin_decoder::table)
        .values(decoder)
        .returning(PluginDecoder::as_returning())
        .get_result(&mut connection())
        .unwrap();

    pipeline
}

pub fn get_pipeline(pipeline_id: i32) -> Pipeline {
    use crate::schema::pipelines;

    pipelines::table
        .filter(pipelines::id.eq(pipeline_id))
        .select(Pipeline::as_returning())
        .get_result(&mut connection())
        .unwrap()
}

pub fn add_plugin_to_pipeline() {}
