# implementation-sample-api-rust

This is a SaaS implementation sample using the SaaSus SDK

See the documentation [API implementation using SaaS Platform](https://docs.saasus.io/docs/implementing-authentication-using-saasus-platform-apiserver)

## Prerequisites
- docker

## Run Rust API
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
docker compose up -d
docker compose exec app bash
cargo run # Inside the app container
```
