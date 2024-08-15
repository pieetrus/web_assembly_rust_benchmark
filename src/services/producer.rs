use std::{fs::File, io::Write};


pub async fn get_traffic_incidents() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let api_key: String = std::env::var("API_KEY")
        .expect("API_KEY must be set in env variables.");

    let base_url = "https://api.tomtom.com/traffic/services/5/incidentDetails";
    let bbox = "4.6541,52.1676,5.1541,52.5676";
    let fields = "{incidents{type,properties{id,iconCategory,magnitudeOfDelay,events{code,description}}}}";
    
    let response = reqwest::get(format!("{base_url}?key={api_key}&bbox={bbox}&fields={fields}"))
        .await?; 

    println!("Status: {}", response.status());

    let body: String = response.text().await?;

    save_response_to_file(&body, "traffic_incidents.json")?;

    Ok(())
}

fn save_response_to_file(content: &str, filename: &str) -> Result<(), std::io::Error> {
    let mut file: File = File::create(filename)?;
    file.write_all(content.as_bytes())?;
    println!("Response saved to {}", filename);
    Ok(())
}