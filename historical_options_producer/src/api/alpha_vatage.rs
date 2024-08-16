use anyhow::{Result, Context};

pub struct AlphaVantageClient {
    api_key: String,
    base_url: String,
}

impl AlphaVantageClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            base_url: "https://www.alphavantage.co/query".to_string(),
        }
    }

    pub async fn get_historical_options(&self, symbol: &str) -> Result<String> {
        let url = format!(
            "{}?function=HISTORICAL_OPTIONS&symbol={}&apikey={}",
            self.base_url, symbol, self.api_key
        );
        let response = reqwest::get(&url)
            .await
            .context("Failed to send request to Alpha Vantage")?;
        
        println!("Alpha Vantage API Status: {}", response.status());
        
        response.text().await
            .context("Failed to get response body from Alpha Vantage")
    }
}