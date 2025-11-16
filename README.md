# Real-Time Onchain Counter

A simple microchain-based counter application built for Linera Buildathon Wave 2. This application demonstrates real-time state synchronization across all clients using Linera's microchain architecture and GraphQL subscriptions.

## Features

- **Real-time Updates**: Counter changes are instantly synchronized across all connected clients
- **Microchain Architecture**: Leverages Linera's fast finality and low latency
- **GraphQL Interface**: Modern API with subscriptions for live updates
- **Simple UI**: Clean interface with increment/decrement buttons
- **Conway Testnet Ready**: Configured for deployment on Linera's Conway testnet

## Project Structure

```
├── contracts/          # Linera smart contract
│   ├── src/
│   │   └── lib.rs     # Main contract logic
│   └── Cargo.toml     # Contract dependencies
├── frontend/           # Web frontend
│   ├── public/
│   │   └── index.html # Main UI
│   ├── server.js      # Simple HTTP server
│   └── package.json   # Frontend dependencies
├── scripts/            # Deployment scripts
│   ├── deploy.sh      # Unix deployment
│   ├── deploy.bat     # Windows deployment
│   ├── run-service.sh # Unix service runner
│   └── run-service.bat# Windows service runner
├── Cargo.toml         # Workspace configuration
└── README.md          # This file
```

## Prerequisites

1. **Rust**: Install from [rustup.rs](https://rustup.rs/)
2. **Visual Studio Build Tools** (Windows only): Install from [Visual Studio Downloads](https://visualstudio.microsoft.com/downloads/) - select "Build Tools for Visual Studio" and include "C++ build tools"
3. **Linera CLI**: Install the Linera command-line tools
4. **Node.js**: For running the frontend server
5. **Conway Testnet Access**: Ensure you have access to Conway testnet

### Install Linera CLI

**Automated Setup (Recommended):**
```bash
# Unix/Linux/macOS
chmod +x scripts/setup.sh
./scripts/setup.sh

# Windows
scripts\setup.bat
```

**Manual Setup:**
```bash
# Clone and install Linera CLI from source
git clone https://github.com/linera-io/linera-protocol.git
cd linera-protocol
cargo install --path linera-cli

# Add WASM target
rustup target add wasm32-unknown-unknown
```

## Quick Start

### 1. Build the Contract

```bash
cd contracts
cargo build --release --target wasm32-unknown-unknown
```

**Windows Quick Fix (if build fails):**
```cmd
# Use GNU toolchain instead of MSVC
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu

# Or use the provided script
scripts\build-windows-gnu.bat
```

### 2. Deploy to Conway Testnet

**Unix/Linux/macOS:**
```bash
chmod +x scripts/deploy.sh
./scripts/deploy.sh
```

**Windows:**
```cmd
scripts\deploy.bat
```

**Manual Deployment:**
```bash
linera publish-and-create \
    --wait-for-outgoing-messages \
    contracts/target/wasm32-unknown-unknown/release/counter.wasm \
    --required-application-ids
```

Save the application ID from the deployment output!

### 3. Start the Service

Replace `YOUR_APP_ID` with the actual application ID from deployment:

**Unix/Linux/macOS:**
```bash
chmod +x scripts/run-service.sh
./scripts/run-service.sh YOUR_APP_ID
```

**Windows:**
```cmd
scripts\run-service.bat YOUR_APP_ID
```

**Manual Service Start:**
```bash
linera service --port 8080 --application-id YOUR_APP_ID
```

### 4. Run the Frontend

```bash
cd frontend
npm install
npm start
```

The frontend will be available at `http://localhost:3001`

## GraphQL API

The application exposes a GraphQL endpoint at `http://localhost:8080/graphql` with the following schema:

### Query
```graphql
query {
  counter {
    value
  }
}
```

### Mutations
```graphql
# Increment counter
mutation {
  increment {
    value
  }
}

# Decrement counter
mutation {
  decrement {
    value
  }
}
```

### Subscription
```graphql
subscription {
  counterUpdates {
    value
  }
}
```

## Example Usage

### Using GraphQL Playground

1. Open `http://localhost:8080/graphql` in your browser
2. Try the following queries:

**Get current value:**
```graphql
{
  counter {
    value
  }
}
```

**Increment counter:**
```graphql
mutation {
  increment {
    value
  }
}
```

**Subscribe to updates:**
```graphql
subscription {
  counterUpdates {
    value
  }
}
```

### Using curl

**Query current value:**
```bash
curl -X POST http://localhost:8080/graphql \
  -H "Content-Type: application/json" \
  -d '{"query": "{ counter { value } }"}'
```

**Increment counter:**
```bash
curl -X POST http://localhost:8080/graphql \
  -H "Content-Type: application/json" \
  -d '{"query": "mutation { increment { value } }"}'
```

## Development

### Contract Development

The main contract logic is in `contracts/src/lib.rs`. Key components:

- **ApplicationState**: Stores the counter value
- **Operation**: Defines Increment/Decrement operations
- **CounterContract**: Handles state mutations
- **CounterService**: Provides GraphQL interface

### Frontend Development

The frontend is a simple HTML page with embedded JavaScript that:

- Connects to the GraphQL service via WebSocket for subscriptions
- Sends mutations for increment/decrement operations
- Updates the UI in real-time when the counter changes

### Testing Locally

1. Build and deploy the contract
2. Start the service
3. Open multiple browser tabs to see real-time synchronization
4. Click increment/decrement in one tab and watch updates in others

## Troubleshooting

### Common Issues

**Build Errors:**
- Ensure you have the `wasm32-unknown-unknown` target installed
- **Windows**: Install Visual Studio Build Tools with C++ support if you get `link.exe` not found error
- **Windows Alternative**: Use `rustup toolchain install stable-x86_64-pc-windows-gnu` and `rustup default stable-x86_64-pc-windows-gnu` to use GNU toolchain instead of MSVC
- Check that all dependencies are properly specified

**Deployment Errors:**
- Verify you're connected to Conway testnet
- Ensure you have sufficient balance for deployment

**Service Connection Issues:**
- Check that the service is running on port 8080
- Verify the application ID is correct
- Ensure no firewall is blocking the connection

**Frontend Issues:**
- Check browser console for JavaScript errors
- Verify the GraphQL endpoint is accessible
- Ensure WebSocket connections are allowed

### Logs and Debugging

**View service logs:**
```bash
linera service --port 8080 --application-id YOUR_APP_ID --verbose
```

**Check contract state:**
```bash
linera query-application YOUR_APP_ID
```

## Conway Testnet Configuration

This application is configured for Conway testnet. Ensure your Linera CLI is configured correctly:

```bash
# Check current configuration
linera wallet show

# If needed, configure for Conway testnet
linera net up --testing-prng-seed 37
```

## Contributing

This is a Buildathon submission, but contributions and improvements are welcome:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## License

MIT License - see LICENSE file for details.

## Buildathon Submission

**Project**: Real-Time Onchain Counter  
**Wave**: Linera Buildathon Wave 2  
**Category**: MVP Application  
**Features**: Real-time state sync, GraphQL subscriptions, Conway testnet deployment

This application demonstrates Linera's core capabilities:
- Fast finality through microchains
- Real-time state synchronization
- Modern GraphQL interface
- Simple deployment process

## Support

For issues related to this application, please open a GitHub issue.
For Linera-specific questions, refer to the [Linera Documentation](https://docs.linera.io/).