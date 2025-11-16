#!/usr/bin/env bash

set -eu

# Ensure WASM target is available
rustup target add wasm32-unknown-unknown

eval "$(linera net helper)"
linera_spawn linera net up --with-faucet

export LINERA_FAUCET_URL=http://localhost:8080
linera wallet init --faucet="$LINERA_FAUCET_URL"
linera wallet request-chain --faucet="$LINERA_FAUCET_URL"

# Build and publish the counter application
echo "Building counter application..."
cargo build --release --target wasm32-unknown-unknown

echo "Publishing counter application..."
COUNTER_APP_ID=$(linera publish-and-create \
    target/wasm32-unknown-unknown/release/counter.wasm \
    --required-application-ids)

echo "Counter application published with ID: $COUNTER_APP_ID"

# Start the service
echo "Starting Linera service..."
linera_spawn linera service --port 9001 --application-id $COUNTER_APP_ID

# Build and run frontend
echo "Setting up frontend..."
cd web
npm install
echo "Starting frontend on port 5173..."
npm run dev