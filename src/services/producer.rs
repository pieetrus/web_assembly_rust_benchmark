use std::{fs::File, io::Write};


pub async fn get_historical_options_information() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let api_key: String = std::env::var("API_KEY")
        .expect("API_KEY must be set in env variables.");

    let base_url = "https://www.alphavantage.co/query";
    let function = "HISTORICAL_OPTIONS";
    let symbol = "IBM";
    
    let response = reqwest::get(format!("{base_url}?function={function}&symbol={symbol}&apikey={api_key}"))
        .await?; 

    println!("Status: {}", response.status());

    let body: String = response.text().await?;

    save_response_to_file(&body, "historical_options_information.json")?;

    Ok(())
}

fn save_response_to_file(content: &str, filename: &str) -> Result<(), std::io::Error> {
    let mut file: File = File::create(filename)?;
    file.write_all(content.as_bytes())?;
    println!("Response saved to {}", filename);
    Ok(())
}