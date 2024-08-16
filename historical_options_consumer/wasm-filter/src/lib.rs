use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoricalOption {
    #[serde(rename = "contractID")]
    pub contract_id: String,
    pub symbol: String,
    pub expiration: String,
    pub strike: String,
    #[serde(rename = "type")]
    pub option_type: String,
    pub last: String,
    pub mark: String,
    pub bid: String,
    pub bid_size: String,
    pub ask: String,
    pub ask_size: String,
    pub volume: String,
    pub open_interest: String,
    pub date: String,
    pub implied_volatility: String,
    pub delta: String,
    pub gamma: String,
    pub theta: String,
    pub vega: String,
    pub rho: String
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
pub extern "C" fn filter_historical_options(ptr: *const u8, len: usize) -> *const u8 {
    let options_json = unsafe { std::slice::from_raw_parts(ptr, len) };
    let options: Vec<HistoricalOption> = serde_json::from_slice(options_json).unwrap();

    let filtered_options: Vec<&HistoricalOption> = options.iter()
        .filter(|option| {
            // Example filter: only keep options with a strike price above 85.00
            option.strike.parse::<f64>().unwrap_or(0.0) > 85.00
        })
        .collect();

    let result = serde_json::to_vec(&filtered_options).unwrap();
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