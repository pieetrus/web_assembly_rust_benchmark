<img src="https://www.rust-lang.org/static/images/wasm-ferris.png" alt="Alt text" height="100px" align="right">

# Web Assembly Rust Benchmark

This distributed Rust application benchmarks the processing of historical options data using WebAssembly. It consists of separate producer and consumer applications, communicating through Apache Kafka, with a mock server providing simulated options data.

## Features

- Uses a mock server to generate random historical options data
- Implements WebAssembly for efficient data filtering
- Demonstrates Rust integration with WebAssembly for data processing tasks
- Implements a distributed system with separate producer and consumer applications
- Uses Apache Kafka for message passing between producer and consumer
- Includes benchmarking using Criterion

## Architecture

The system consists of four main components:

1. Mock Options Server: Generates random options data.
2. Producer Application: Fetches data from the mock server and sends it to Kafka.
3. Consumer Application: Reads data from Kafka and processes it using WebAssembly.
4. WebAssembly Module: Filters the options data (currently filtering options with a strike price above $85.00).

## Data Description

The application processes mock historical options data, which includes the following information for each option contract:

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

1. The mock server generates random historical options data.
2. The producer application fetches data from the mock server.
3. The producer sends the data to a Kafka topic.
4. The consumer application reads the data from the Kafka topic.
5. A WebAssembly module in the consumer application filters the options data.
6. The filtered results are displayed in the console.

## Benchmarking

The benchmark focuses on the processing time of the WebAssembly module. It uses sample data from `sample_data.json` to ensure consistent results across runs. Criterion is used for precise and reliable benchmarking.

## Setup

1. Clone the repository:
   ```
   git clone <repository-url>
   cd wasm-rust-benchmark
   ```

2. Start Kafka, Zookeeper, and the mock server using Docker Compose:
   ```
   docker-compose up -d
   ```

3. Build the WebAssembly module:
   ```
   cd wasm-filter
   cargo build --target wasm32-unknown-unknown --release
   ```

4. Build and run the producer application:
   ```
   cd ../historical_options_producer
   cargo run
   ```

5. In a separate terminal, build and run the consumer application:
   ```
   cd ../historical_options_consumer
   cargo run
   ```

6. To run the benchmark:
   ```
   cargo bench
   ```

## Dependencies

- Rust 1.55 or later
- wasm-pack
- Docker and Docker Compose (for running Kafka, Zookeeper, and the mock server)
- Criterion (for benchmarking)

## Mock Server

The mock server is a Flask application that generates random options data. It's containerized and runs alongside Kafka and Zookeeper in the Docker Compose setup.

## Note

This benchmark focuses on the processing performance of the WebAssembly module. The `sample_data.json` file is used for consistent benchmark results, bypassing the full flow with message queues to isolate the processing time.