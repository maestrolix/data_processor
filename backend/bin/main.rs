use dotenvy::dotenv;
use pipeline::config::Config;
use pipeline::models::{InputTask, MashineLearning};
use pipeline::services::pipeline::run_pipeline;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use rdkafka::{ClientConfig, Message};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let config = Config::init().unwrap();

    let kafka_addr = format!("{}:{}", config.kafka.host, config.kafka.port);
    let ml = MashineLearning::from_config(&config.model);

    let producer = create_producer(&kafka_addr).await;
    let consumer = create_consumer(&kafka_addr).await;
    consumer
        .subscribe(&[config.kafka.topic_input.as_ref()])
        .unwrap();

    loop {
        tokio::select! {
            message = consumer.recv() => {
                let message  = message.expect("Failed to read message").detach();
                let key = message.key().ok_or_else(|| "no key for message").unwrap();
                let payload = message.payload().ok_or_else(|| "no payload for message").unwrap();

                let task = InputTask::from_slice(key, payload).await;
                let output_task = run_pipeline(task, &ml).await;

                producer
                    .send(
                        FutureRecord::to(config.kafka.topic_output.as_ref())
                            .key("task")
                            .payload(&serde_json::to_string(&output_task).unwrap()), Timeout::Never
                    ).await
                    .map_err(|(e, _)| format!("Failed to produce: {:?}", e)).unwrap();
            }
        }
    }
}

async fn create_producer(bootstrap_server: &str) -> FutureProducer {
    ClientConfig::new()
        .set("bootstrap.servers", bootstrap_server)
        .set("message.timeout.ms", "5000")
        .create()
        .unwrap()
}

async fn create_consumer(bootstrap_server: &str) -> StreamConsumer {
    ClientConfig::new()
        .set("bootstrap.servers", bootstrap_server)
        .set("enable.partition.eof", "false")
        .set("group.id", format!("{}", Uuid::default()))
        .create()
        .expect("Failed to create client")
}
