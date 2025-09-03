#!/usr/bin/env bash
set -euo pipefail

# Config
CLI=safrochaind
CHAIN_ID=safro-testnet-1
NODE=https://rpc.testnet.safrochain.com
ADMIN_NAME=mycontractadmin
ADMIN_ADDR=addr_safro18cmxlyj8yllr702kzujhkuwth3rektfpcw8f7p
GAS_PRICES=0.025usaf
GAS_ADJ=1.3
WASM_PATH="contract-optimized.wasm"

if [ ! -f "$WASM_PATH" ]; then
  echo "Wasm file not found at $WASM_PATH. Build or provide the optimized wasm first." >&2
  exit 1
fi


echo "Choose action:"
echo "1) Deploy and instantiate new contract"
echo "2) Instantiate existing contract code (skip deploy)"
read -p "Enter 1 or 2: " CHOICE

if [[ "$CHOICE" == "1" ]]; then
  # Store wasm
  echo "Storing wasm..."
  STORE_OUT=$($CLI tx wasm store "$WASM_PATH" \
    --from "$ADMIN_NAME" --keyring-backend file \
    --gas auto --gas-prices "$GAS_PRICES" --gas-adjustment "$GAS_ADJ" -y \
    --broadcast-mode sync \
    --chain-id "$CHAIN_ID" --node "$NODE" \
    --output json)
  TXHASH=$(echo "$STORE_OUT" | jq -r '.txhash')
  echo "Store txhash: $TXHASH"
  sleep 4
  # Find latest code id (assumes last is ours)
  CODE_ID=$($CLI query wasm list-code --output json --node "$NODE" | jq -r '.code_infos[-1].code_id')
  echo "CODE_ID: $CODE_ID"
elif [[ "$CHOICE" == "2" ]]; then
  read -p "Enter existing CODE_ID: " CODE_ID
  echo "Using CODE_ID: $CODE_ID"
else
  echo "Invalid choice. Exiting."
  exit 1
fi

# Build INIT JSON (portable heredoc)
INIT=$(cat <<'JSON'
{
  "name":"My ROSCA",
  "symbol":"ROSCA1",
  "admin":"addr_safro18cmxlyj8yllr702kzujhkuwth3rektfpcw8f7p",
  "members":[],
  "member_profiles":{},
  "contribution_amount":"1000000",
  "total_cycles":5,
  "cycle_duration":2592000,
  "distribution_mode":"Fifo",
  "start_mode":"Manual",
  "start_condition_auto":null,
  "deposit_deadline":604800,
  "grace_seconds":86400,
  "late_penalty_percent":5,
  "late_strike_limit":2,
  "distribution_calendar":[],
  "allow_member_exit":true,
  "allow_member_add":true,
  "early_withdrawal_penalty":10,
  "forbid_overpay":true,
  "forbid_underpay":true,
  "max_members": 20,
  "caution_deposit": "1000000"
}
JSON
)

# Instantiate
echo "Instantiating..."
$CLI tx wasm instantiate "$CODE_ID" "$INIT" \
  --label safrimba-1 \
  --admin "$ADMIN_ADDR" \
  --from "$ADMIN_NAME" --keyring-backend file \
  --gas auto --gas-prices "$GAS_PRICES" --gas-adjustment "$GAS_ADJ" -y \
  --broadcast-mode sync \
  --chain-id "$CHAIN_ID" --node "$NODE"

# Wait for the contract to be indexed
sleep 4

# Get contract address (last for this code id)
CONTRACT_ADDR=$($CLI query wasm list-contract-by-code "$CODE_ID" --output json --node "$NODE" | jq -r '.contracts[-1]')
if [ -z "$CONTRACT_ADDR" ] || [ "$CONTRACT_ADDR" = "null" ]; then
  echo "Failed to resolve CONTRACT_ADDR. Check the instantiate transaction and try again." >&2
  exit 1
fi

echo "CONTRACT_ADDR: $CONTRACT_ADDR"
