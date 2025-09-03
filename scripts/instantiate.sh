#!/bin/bash

# Instantiation script for Safrimba smart contract
set -e

# Configuration
CHAIN_ID="${CHAIN_ID:-safrochain-testnet-1}"
NODE="${NODE:-https://rpc-testnet.safrochain.com:443}"
DENOM="${DENOM:-usaf}"
GAS_PRICES="${GAS_PRICES:-0.001usaf}"
KEY_NAME="${KEY_NAME:-deployer}"

# Check arguments
if [ $# -ne 1 ]; then
    echo "Usage: $0 <init_message.json>"
    echo "Example: $0 examples/init_testnet.json"
    exit 1
fi

INIT_MSG_FILE="$1"

# Check if init message file exists
if [ ! -f "$INIT_MSG_FILE" ]; then
    echo "❌ Init message file not found: $INIT_MSG_FILE"
    exit 1
fi

# Load deployment info
DEPLOY_INFO_FILE="deploy/deployment-$CHAIN_ID.json"
if [ ! -f "$DEPLOY_INFO_FILE" ]; then
    echo "❌ Deployment info not found. Please deploy the contract first with ./scripts/deploy.sh"
    exit 1
fi

CODE_ID=$(jq -r '.code_id' $DEPLOY_INFO_FILE)
DEPLOYER=$(safrochaind keys show $KEY_NAME -a)

echo "🏗️  Instantiating Safrimba contract..."
echo "Code ID: $CODE_ID"
echo "Admin: $DEPLOYER"
echo "Init message: $INIT_MSG_FILE"

# Read the init message
INIT_MSG=$(cat $INIT_MSG_FILE | jq -c .)

# Instantiate the contract
INSTANTIATE_TX=$(safrochaind tx wasm instantiate $CODE_ID "$INIT_MSG" \
    --from $KEY_NAME \
    --label "Safrimba Tontine v1.0" \
    --admin $DEPLOYER \
    --gas auto \
    --gas-adjustment 1.3 \
    --gas-prices $GAS_PRICES \
    --chain-id $CHAIN_ID \
    --node $NODE \
    --broadcast-mode block \
    --yes \
    --output json)

# Extract contract address
CONTRACT_ADDRESS=$(echo $INSTANTIATE_TX | jq -r '.logs[0].events[] | select(.type=="instantiate") | .attributes[] | select(.key=="contract_address") | .value')
echo "✅ Contract instantiated at: $CONTRACT_ADDRESS"

# Update deployment info
jq --arg addr "$CONTRACT_ADDRESS" --arg tx "$(echo $INSTANTIATE_TX | jq -r '.txhash')" --arg time "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
    '. + {contract_address: $addr, instantiate_tx_hash: $tx, instantiated_at: $time}' \
    $DEPLOY_INFO_FILE > tmp.json && mv tmp.json $DEPLOY_INFO_FILE

echo "📝 Contract address saved to $DEPLOY_INFO_FILE"
echo "🎉 Contract instantiation complete!"
echo ""
echo "Contract address: $CONTRACT_ADDRESS"
echo "Next steps:"
echo "1. Add members: ./scripts/execute.sh add_member.json"
echo "2. Start the tontine: ./scripts/execute.sh start_tontine.json"
echo "3. Query contract state: ./scripts/query.sh group_info.json"