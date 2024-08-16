use services::produce_historical_options;
use anyhow::Result;
use tokio::time::{interval, Duration};

mod services;
mod api;
mod kaffka;

#[tokio::main]
async fn main() -> Result<()> {
    let mut interval = interval(Duration::from_secs(2));
    loop {
        interval.tick().await;
        if let Err(e) = produce_historical_options().await {
            eprintln!("Error producing message: {}", e);
        }
    }
}