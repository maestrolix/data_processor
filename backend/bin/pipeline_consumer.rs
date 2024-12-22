use pipeline_consumer::run_consumer;

#[tokio::main]
async fn main() {
    run_consumer().await
}
