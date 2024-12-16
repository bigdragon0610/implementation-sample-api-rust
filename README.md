# SaaSus SDK for Rust - Sample API

This is a sample application that demonstrates how to use the [SaaSus SDK for Rust](https://github.com/bigdragon0610/saasus-sdk-rust).

## Prerequisites

This sample application is designed to be used together with the frontend sample application:
- [Frontend Sample for React](https://github.com/saasus-platform/implementation-sample-front-react/)

## Run the API
```sh
# Run in any directory
git clone git@github.com:bigdragon0610/implementation-sample-api-rust.git
cd implementation-sample-api-rust

cp .env.example .env
vi .env

# Set Env for SaaSus Platform API
# Get it in the SaaSus Admin Console
SAASUS_SAAS_ID="xxxxxxxxxx"
SAASUS_API_KEY="xxxxxxxxxx"
SAASUS_SECRET_KEY="xxxxxxxxxx"

# Save and exit
```

```sh
# API will run on http://localhost
cargo run
```
