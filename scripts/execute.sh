#!/bin/bash

# Execute script for Safrimba smart contract
set -e

# Configuration
CHAIN_ID="${CHAIN_ID:-safrochain-testnet-1}"
NODE="${NODE:-https://rpc-testnet.safrochain.com:443}"
DENOM="${DENOM:-usaf}"
GAS_PRICES="${GAS_PRICES:-0.001usaf}"
KEY_NAME="${KEY_NAME:-deployer}"

# Check arguments
if [ $# -ne 1 ]; then
    echo "Usage: $0 <execute_message.json>"
    echo "Example: $0 examples/add_member.json"
    exit 1
fi

EXECUTE_MSG_FILE="$1"

# Check if execute message file exists
if [ ! -f "$EXECUTE_MSG_FILE" ]; then
    echo "❌ Execute message file not found: $EXECUTE_MSG_FILE"
    exit 1
fi

# Load deployment info
DEPLOY_INFO_FILE="deploy/deployment-$CHAIN_ID.json"
if [ ! -f "$DEPLOY_INFO_FILE" ]; then
    echo "❌ Deployment info not found. Please deploy and instantiate the contract first"
    exit 1
fi

CONTRACT_ADDRESS=$(jq -r '.contract_address' $DEPLOY_INFO_FILE)
if [ "$CONTRACT_ADDRESS" = "null" ]; then
    echo "❌ Contract not instantiated. Please run ./scripts/instantiate.sh first"
    exit 1
fi

SENDER=$(safrochaind keys show $KEY_NAME -a)

echo "⚡ Executing transaction on Safrimba contract..."
echo "Contract: $CONTRACT_ADDRESS"
echo "Sender: $SENDER"
echo "Message: $EXECUTE_MSG_FILE"

# Read the execute message
EXECUTE_MSG=$(cat $EXECUTE_MSG_FILE | jq -c .)

# Execute the transaction
echo "📡 Broadcasting transaction..."
TX_RESULT=$(safrochaind tx wasm execute $CONTRACT_ADDRESS "$EXECUTE_MSG" \
    --from $KEY_NAME \
    --gas auto \
    --gas-adjustment 1.3 \
    --gas-prices $GAS_PRICES \
    --chain-id $CHAIN_ID \
    --node $NODE \
    --broadcast-mode block \
    --yes \
    --output json)

echo "✅ Transaction executed successfully!"
echo "TX Hash: $(echo $TX_RESULT | jq -r '.txhash')"
echo "Gas used: $(echo $TX_RESULT | jq -r '.gas_used')"

# Show events if any
if echo $TX_RESULT | jq -e '.logs[0].events' > /dev/null 2>&1; then
    echo "Events:"
    echo $TX_RESULT | jq '.logs[0].events'
fi