mod pipeline;

use axum::extract::DefaultBodyLimit;
use axum::Router;
use tower_cookies::CookieManagerLayer;
use tower_http::{cors::CorsLayer, services::ServeDir};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub async fn craete_app() -> Router {
    #[derive(OpenApi)]
    #[openapi(
        paths(

        ),
        components(
            schemas()
        ),
        tags(
            (name = "users", description = "Управление пользователями"),
            (name = "albums", description = "Управление альбомами"),
            (name = "photos", description = "Управления фотографиями")
        )
    )]
    struct ApiDoc;

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .nest("/pipeline", pipeline::router().await)
        .nest_service("/storage", ServeDir::new("storage"))
        .layer(CookieManagerLayer::new())
        .layer(DefaultBodyLimit::max(100000000))
        .layer(CorsLayer::permissive())
}
