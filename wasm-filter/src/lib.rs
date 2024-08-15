use serde::{Deserialize, Serialize};

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

static mut RESULT: Option<Vec<u8>> = None;

#[no_mangle]
pub extern "C" fn alloc(len: usize) -> *mut u8 {
    let mut buf = Vec::with_capacity(len);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut u8, len: usize) {
    unsafe {
        let _ = Vec::from_raw_parts(ptr, 0, len);
    }
}

#[no_mangle]
pub extern "C" fn filter_incidents(ptr: *const u8, len: usize) -> *const u8 {
    let incidents_json = unsafe { std::slice::from_raw_parts(ptr, len) };
    let incidents: Vec<Incident> = serde_json::from_slice(incidents_json).unwrap();
    
    let filtered_incidents: Vec<&Incident> = incidents.iter()
        .filter(|incident| {
            incident.properties.events.iter().all(|event| event.code != 701)
        })
        .collect();
    
    let result = serde_json::to_vec(&filtered_incidents).unwrap();
    
    unsafe {
        RESULT = Some(result);
        RESULT.as_ref().unwrap().as_ptr()
    }
}

#[no_mangle]
pub extern "C" fn get_result_len() -> usize {
    unsafe {
        RESULT.as_ref().map_or(0, |v| v.len())
    }
}