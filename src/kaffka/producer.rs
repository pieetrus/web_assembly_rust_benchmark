use anyhow::{Result, Context};
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;

pub struct KafkaProducer {
    producer: FutureProducer,
}

impl KafkaProducer {
    pub fn new(brokers: &str) -> Result<Self> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("message.timeout.ms", "5000")
            .create()
            .context("Failed to create Kafka producer")?;
        
        Ok(Self { producer })
    }

    pub async fn send_message(&self, topic: &str, key: &str, payload: &str) -> Result<()> {
        let record = FutureRecord::to(topic)
            .payload(payload)
            .key(key);

        self.producer.send(record, Duration::from_secs(0))
            .await
            .map(|delivery| {
                println!("Sent message to Kafka: {:?}", delivery);
            })
            .map_err(|(e, _)| anyhow::anyhow!("Error sending message to Kafka: {:?}", e))?;

        Ok(())
    }
}