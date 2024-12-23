use axum::{
    routing::{get, post},
    Json, Router,
};

use crate::models::{NewPipeline, Pipeline};
use crate::services::pipeline;

pub async fn router() -> Router {
    Router::new()
        .route("/", post(create_pipeline))
        .route("/", get(get_pipeline))
}

#[utoipa::path(
    post,
    path = "/pipeline",
    request_body = NewPipeline,
    responses(
        (status = 201, description = "Create album", body = Pipeline)
    )
)]
pub async fn create_pipeline(Json(new_pipeline): Json<NewPipeline>) -> Json<Pipeline> {
    Json(pipeline::create_pipeline(new_pipeline))
}

#[utoipa::path(
    get,
    path = "/pipeline/{id}",
    params(("id" = i32, Path, description = "")),
    responses(
        (status = 200, description = "", body = Pipeline)
    )
)]
pub async fn get_pipeline(Json(new_pipeline): Json<NewPipeline>) -> Json<Pipeline> {
    Json(pipeline::create_pipeline(new_pipeline))
}

// #[utoipa::path(
//     delete,
//     path = "/api/album/{album_id}",
//     params(("album_id" = i32, Path, description = "Todo database id")),
//     responses(
//         (status = 201, description = "Create album")
//     )
// )]
// pub async fn delete_album(Path(album_id): Path<i32>) {}

// #[utoipa::path(
//     get,
//     path = "/api/album/{album_id}",
//     params(("album_id" = i32, Path, description = "Id of album")),
//     responses(
//         (status = 200, description = "Detail info about album", body = Album)
//     )
// )]
// pub async fn get_album(Path(album_id): Path<i32>) {}

// #[utoipa::path(
//     get,
//     path = "/api/album",
//     responses(
//         (status = 200, description = "Detail info about album", body = Vec<Album>)
//     )
// )]
// pub async fn get_albums() {}
