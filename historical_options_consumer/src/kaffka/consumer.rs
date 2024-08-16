use anyhow::{Result, Context};
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::Message;

pub struct KafkaConsumer {
    consumer: StreamConsumer,
}

impl KafkaConsumer {
    pub fn new(brokers: &str, group_id: &str, topic: &str) -> Result<Self> {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("group.id", group_id)
            .set("bootstrap.servers", brokers)
            .set("auto.offset.reset", "earliest")
            .create()
            .context("Failed to create Kafka consumer")?;

        consumer.subscribe(&[topic])
            .context("Failed to subscribe to specified topics")?;

        Ok(Self { consumer })
    }

    pub async fn receive_message(&self) -> Result<Vec<u8>> {
        println!("Waiting for message...");
        let message = self.consumer.recv().await
            .context("Failed to receive message")?;
        
        message.payload().map(|p| p.to_vec())
            .context("Empty payload")
    }
}