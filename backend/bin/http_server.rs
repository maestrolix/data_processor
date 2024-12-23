use http_server::routes;

#[tokio::main]
async fn main() {
    env_logger::init();

    let app = routes::craete_app().await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
