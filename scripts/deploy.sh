#!/bin/bash

# Deploy Counter Application to Linera Testnet Conway

echo "Building Counter Application..."

# Build the contract
cd contracts
cargo build --release --target wasm32-unknown-unknown
cd ..

echo "Publishing application to Conway testnet..."

# Publish the application
linera publish-and-create \
    --wait-for-outgoing-messages \
    contracts/target/wasm32-unknown-unknown/release/counter.wasm \
    --required-application-ids

echo "Application deployed successfully!"
echo "Note: Save the application ID from the output above for frontend configuration."