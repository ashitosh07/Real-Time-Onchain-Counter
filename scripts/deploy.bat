@echo off

echo Building Counter Application...

cd contracts
cargo build --release --target wasm32-unknown-unknown
cd ..

echo Publishing application to Conway testnet...

linera publish-and-create --wait-for-outgoing-messages contracts/target/wasm32-unknown-unknown/release/counter.wasm --required-application-ids

echo Application deployed successfully!
echo Note: Save the application ID from the output above for frontend configuration.