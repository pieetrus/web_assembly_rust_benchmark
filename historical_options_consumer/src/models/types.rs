use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HistoricalOptionsResponse {
    pub endpoint: String,
    pub message: String,
    pub data: Vec<HistoricalOption>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    pub rho: String,
}