#!/bin/bash

# Deployment script for Safrimba smart contract
set -e

# Configuration
CHAIN_ID="${CHAIN_ID:-safrochain-testnet-1}"
NODE="${NODE:-https://rpc-testnet.safrochain.com:443}"
DENOM="${DENOM:-usaf}"
GAS_PRICES="${GAS_PRICES:-0.001usaf}"
KEY_NAME="${KEY_NAME:-deployer}"

# Contract parameters
CONTRACT_NAME="safrimba"
WASM_FILE="artifacts/safrimba.wasm"

echo "🚀 Deploying Safrimba to $CHAIN_ID"
echo "Node: $NODE"
echo "Key: $KEY_NAME"

# Check if WASM file exists
if [ ! -f "$WASM_FILE" ]; then
    echo "❌ WASM file not found. Please run ./scripts/build.sh first"
    exit 1
fi

# Check if key exists
if ! safrochaind keys show $KEY_NAME > /dev/null 2>&1; then
    echo "❌ Key '$KEY_NAME' not found. Please create or import a key first"
    echo "Example: safrochaind keys add $KEY_NAME"
    exit 1
fi

# Get deployer address
DEPLOYER=$(safrochaind keys show $KEY_NAME -a)
echo "Deployer address: $DEPLOYER"

# Check balance
echo "Checking balance..."
safrochaind query bank balances $DEPLOYER --node $NODE

# Store the contract
echo "📤 Storing contract..."
STORE_TX=$(safrochaind tx wasm store $WASM_FILE \
    --from $KEY_NAME \
    --gas auto \
    --gas-adjustment 1.3 \
    --gas-prices $GAS_PRICES \
    --chain-id $CHAIN_ID \
    --node $NODE \
    --broadcast-mode block \
    --yes \
    --output json)

# Extract code ID
CODE_ID=$(echo $STORE_TX | jq -r '.logs[0].events[] | select(.type=="store_code") | .attributes[] | select(.key=="code_id") | .value')
echo "✅ Contract stored with Code ID: $CODE_ID"

# Save deployment info
DEPLOY_INFO_FILE="deploy/deployment-$CHAIN_ID.json"
mkdir -p deploy

# Create deployment info
cat > $DEPLOY_INFO_FILE << EOF
{
  "chain_id": "$CHAIN_ID",
  "code_id": "$CODE_ID",
  "deployer": "$DEPLOYER",
  "contract_name": "$CONTRACT_NAME",
  "store_tx_hash": "$(echo $STORE_TX | jq -r '.txhash')",
  "deployed_at": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "node": "$NODE"
}
EOF

echo "📝 Deployment info saved to $DEPLOY_INFO_FILE"
echo "🎉 Contract deployment complete!"
echo ""
echo "Next steps:"
echo "1. Create an instantiate message (see examples/ directory)"
echo "2. Instantiate the contract with: ./scripts/instantiate.sh <init_message.json>"