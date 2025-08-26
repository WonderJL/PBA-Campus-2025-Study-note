# Polkadot JSON-RPC Examples

This directory contains examples demonstrating how to interact with Polkadot mainnet using JSON-RPC over WebSocket.

## Examples

### 01_polkadot_header_subscription.rs

This example demonstrates how to:
- Connect to Polkadot mainnet via WebSocket
- Subscribe to new block headers using `chain_subscribeNewHeads`
- Display header payload information in real-time

#### Features

- **Demo Mode (Default)**: Shows JSON-RPC subscription format and example header structure without requiring a live connection
- **Live Mode**: Connects to Polkadot mainnet and subscribes to real-time block headers
- **Header Payload Display**: Shows key header information including:
  - Parent Hash
  - Block Number
  - State Root
  - Extrinsics Root
  - Digest information
- **Field Decoding**: Detailed explanation of each header field:
  - Hex values and their meanings
  - Data types and purposes
  - Consensus engine identification
- **Header Hash Computation**: Computes the block header hash using Blake2b-256:
  - Shows input data structure
  - Demonstrates hash algorithm
  - Provides computed hash value
- **Connection Management**: Handles WebSocket connection lifecycle with timeout
- **Error Handling**: Robust error handling for network and parsing issues
- **Environment Configuration**: Use `DEMO_MODE=false` to enable live connection mode

#### Running the Example

From the project root directory:

```bash
# Run in demo mode (default) - shows JSON-RPC format and example header
cargo run -p json-rpc-examples --example 01_polkadot_header_subscription

# Or using make commands
make run-06        # Demo mode
make run-06-live   # Live connection mode

# Direct cargo commands
DEMO_MODE=true cargo run -p json-rpc-examples --example 01_polkadot_header_subscription
DEMO_MODE=false cargo run -p json-rpc-examples --example 01_polkadot_header_subscription
```

**Note**: This example is part of the `json-rpc-examples` package in the workspace.

#### Expected Output (Demo Mode)

```
🔗 Polkadot Header Subscription Example
=====================================
🎭 Running in DEMO mode
📝 This demonstrates the JSON-RPC subscription format and header structure
💡 To connect to a real node, set DEMO_MODE=false and ensure you have a valid RPC endpoint

📡 JSON-RPC Subscription Message:
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "chain_subscribeNewHeads",
  "params": []
}

📦 Example Header Payload:
🆕 New Block Header Received!
==================================================
Parent Hash: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
Block Number: "0x1234567"
State Root: "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890"
Extrinsics Root: "0x9876543210fedcba9876543210fedcba9876543210fedcba9876543210fedcba"
Digest: {"logs":["0x0642414245b5010100000000","0x05424142450101"]}
==================================================

🔧 To run with a real connection:
   DEMO_MODE=false cargo run --example 01_polkadot_header_subscription

🌐 Available RPC endpoints:
   - wss://rpc.polkadot.io
   - wss://polkadot-rpc-tn.dwellir.com
   - wss://polkadot.api.onfinality.io/public-ws
```

#### JSON-RPC Method Details

The example uses the `chain_subscribeNewHeads` method:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "chain_subscribeNewHeads",
  "params": []
}
```

This method:
- Subscribes to new block headers
- Returns a subscription ID
- Sends notifications whenever a new block header is available
- The header payload contains all the block header information

#### Header Payload Structure

The header payload includes:
- `parentHash`: Hash of the parent block
- `number`: Block number (hex string)
- `stateRoot`: Root hash of the state trie
- `extrinsicsRoot`: Root hash of the extrinsics trie
- `digest`: Block digest containing logs and other metadata

#### Network Endpoints

The example connects to `wss://rpc.polkadot.io`, which is a public RPC endpoint. You can replace this with:
- Your own Polkadot node
- Other public endpoints
- Local development node

#### Dependencies

- `tokio`: Async runtime
- `tokio-tungstenite`: WebSocket client
- `futures-util`: Async stream utilities
- `serde_json`: JSON serialization/deserialization
- `url`: URL parsing

## Future Examples

Planned examples for this directory:
- Block body subscription
- Storage subscription
- Runtime version subscription
- Custom RPC method calls
- Error handling patterns
