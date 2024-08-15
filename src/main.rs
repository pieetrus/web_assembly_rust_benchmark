mod models;

use std::{error::Error, fs::{self, File}, io::Write};
use models::{Incident, TrafficIncidentResponse};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    get_traffic_incidents().await?;
    
    consume()?;

    Ok(())
}

async fn get_traffic_incidents() -> Result<(), Box<dyn std::error::Error>> {
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

fn consume() -> Result<(), Box<dyn Error>> {
    // Read the JSON file
    let file_content = fs::read_to_string("traffic_incidents.json")?;
    
    // Parse the JSON content
    let json_data: TrafficIncidentResponse = serde_json::from_str(&file_content)?;
    
    // Filter incidents where every event has a code not equal to 701
    let filtered_incidents: Vec<&Incident> = json_data.incidents.iter()
        .filter(|incident| {
            incident.properties.events.iter().all(|event| event.code != 701)
        })
        .collect();
        
    // Print the filtered incidents
    println!("Filtered Incidents (excluding roadworks):");
    for incident in filtered_incidents {
        println!("{:#?}", incident);
    }
    
    Ok(())
}