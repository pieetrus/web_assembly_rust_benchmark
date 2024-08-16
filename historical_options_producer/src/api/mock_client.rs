use anyhow::Result;

pub struct MockServerClient {
    client: reqwest::Client
}

impl MockServerClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_historical_options(&self) -> Result<String> {
        let url = "http://localhost:5000/query?function=HISTORICAL_OPTIONS";

        let response = self.client.get(url).send().await?;
        let body = response.text().await?;
        Ok(body)
    }
}
