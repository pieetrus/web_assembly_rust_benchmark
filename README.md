# Historical Options Analyzer

This Rust application fetches and analyzes historical options data using the Alpha Vantage API and WebAssembly for data processing.

## Features

- Fetches historical options data for a specific stock symbol (currently set to IBM)
- Uses WebAssembly for efficient data filtering
- Demonstrates Rust integration with WebAssembly for data processing tasks

## API Used

This application uses the [Alpha Vantage API](https://www.alphavantage.co/documentation/) to fetch historical options data. Specifically, it uses the `HISTORICAL_OPTIONS` endpoint.

## Data Description

The application processes historical options data, which includes the following information for each option contract:

- Contract ID
- Symbol
- Expiration date
- Strike price
- Option type (call/put)
- Last price
- Mark price
- Bid and ask prices
- Volume and open interest
- Greeks (delta, gamma, theta, vega, rho)
- Implied volatility

## How It Works

1. The application fetches historical options data from Alpha Vantage API.
2. The data is saved to a local JSON file.
3. A WebAssembly module is used to filter the options data (currently filtering options with a strike price above $85.00).
4. The filtered results are displayed in the console.

## Setup

1. Clone the repository
2. Set up your Alpha Vantage API key in a `.env` file:
   ```
   API_KEY=your_alpha_vantage_api_key
   ```
3. Build the WebAssembly module:
   ```
   cd wasm-filter
   cargo build --target wasm32-unknown-unknown --release
   ```
4. Run the main application:
   ```
   cargo run
   ```
