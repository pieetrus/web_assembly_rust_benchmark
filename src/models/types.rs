use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TrafficIncidentResponse {
    pub incidents: Vec<Incident>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Incident {
    #[serde(rename = "type")]
    pub incident_type: String,
    pub properties: IncidentProperties,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncidentProperties {
    pub id: String,
    #[serde(rename = "magnitudeOfDelay")]
    pub magnitude_of_delay: u8,
    pub events: Vec<Event>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub code: u16,
    pub description: String,
}