#!/bin/bash

# Query script for Safrimba smart contract
set -e

# Configuration
CHAIN_ID="${CHAIN_ID:-safrochain-testnet-1}"
NODE="${NODE:-https://rpc-testnet.safrochain.com:443}"

# Check arguments
if [ $# -ne 1 ]; then
    echo "Usage: $0 <query_message.json>"
    echo "Example: $0 examples/query_group_info.json"
    exit 1
fi

QUERY_MSG_FILE="$1"

# Check if query message file exists
if [ ! -f "$QUERY_MSG_FILE" ]; then
    echo "❌ Query message file not found: $QUERY_MSG_FILE"
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

echo "🔍 Querying Safrimba contract..."
echo "Contract: $CONTRACT_ADDRESS"
echo "Query: $QUERY_MSG_FILE"

# Read the query message
QUERY_MSG=$(cat $QUERY_MSG_FILE | jq -c .)

# Execute the query
echo "📡 Executing query..."
RESULT=$(safrochaind query wasm contract-state smart $CONTRACT_ADDRESS "$QUERY_MSG" \
    --node $NODE \
    --output json)

echo "✅ Query result:"
echo $RESULT | jq .