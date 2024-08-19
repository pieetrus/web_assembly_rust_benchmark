use crate::models::HistoricalOption;

pub fn native_filter_options(options: &[HistoricalOption]) -> Vec<HistoricalOption> {
    options.iter()
        .filter(|option| {
            // Example filter: only keep options with a strike price above 85.00
            option.strike.parse::<f64>().unwrap_or(0.0) > 85.0
        })
        .cloned()
        .collect()
}