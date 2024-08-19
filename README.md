# Web Assembly Rust Benchmark

This distributed Rust application fetches and analyzes historical options data using the Alpha Vantage API and WebAssembly for data processing. It consists of separate producer and consumer applications, communicating through Apache Kafka.

## Features

- Fetches historical options data for a specific stock symbol (currently set to IBM)
- Uses WebAssembly for efficient data filtering
- Demonstrates Rust integration with WebAssembly for data processing tasks
- Implements a distributed system with separate producer and consumer applications
- Uses Apache Kafka for message passing between producer and consumer

## Architecture

The system consists of three main components:

1. **Producer Application**: Fetches data from Alpha Vantage API and sends it to Kafka.
2. **Consumer Application**: Reads data from Kafka and processes it using WebAssembly.
3. **WebAssembly Module**: Filters the options data (currently filtering options with a strike price above $85.00).

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

1. The producer application fetches historical options data from Alpha Vantage API.
2. The producer sends the data to a Kafka topic.
3. The consumer application reads the data from the Kafka topic.
4. A WebAssembly module in the consumer application filters the options data.
5. The filtered results are displayed in the console.

## Setup

1. Clone the repository:
   ```
   git clone <repository-url>
   cd historical-options-analyzer
   ```

2. Set up your Alpha Vantage API key in a `.env` file in the `historical_options_producer` directory:
   ```
   API_KEY=your_alpha_vantage_api_key
   ```

3. Start Kafka and Zookeeper using Docker Compose:
   ```
   docker-compose up -d
   ```

4. Build the WebAssembly module:
   ```
   cd wasm-filter
   cargo build --target wasm32-unknown-unknown --release
   ```

5. Build and run the producer application:
   ```
   cd ../historical_options_producer
   cargo run
   ```

6. In a separate terminal, build and run the consumer application:
   ```
   cd ../historical_options_consumer
   cargo run
   ```

## Dependencies

- Rust 1.55 or later
- wasm-pack
- Docker and Docker Compose (for running Kafka and Zookeeper)
- Alpha Vantage API key
