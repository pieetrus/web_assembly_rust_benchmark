use std::{error::Error, fs::{self}};

use crate::models::types::{Incident, TrafficIncidentResponse};

pub fn consume() -> Result<(), Box<dyn Error>> {
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