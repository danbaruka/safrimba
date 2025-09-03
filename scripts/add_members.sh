#!/usr/bin/env bash
set -euo pipefail

# Config: set parameters here (not via command args)
CLI=safrochaind
CHAIN_ID=safro-testnet-1
NODE=https://rpc.testnet.safrochain.com
ADMIN_NAME=mycontractadmin
GAS_PRICES=0.025usaf
GAS_ADJ=1.3
# Contract address from last deploy (set it here)
CONTRACT_ADDR="addr_safro1v8e4j5l0an5tyhegs26redyajvhr0rsltj6s2jzjr7uefma2qp2q5h4yzg"
# Caution deposit (micro usaf) that must match instantiate value
CAUTION=1000000
# Members to add and their pseudonyms
MEMBERS=(
  "addr_safro1zxs95zht04vv3r6rj8ahcx3eg9gs43un640sp5" "Member2"
  "addr_safro128t7dz2qwefww6zkkauktdyhhvgeatr3tf9rnw" "Member3"
  "addr_safro1zxs95zht04vv3r6rj8ahcx3eg9gs43un640sp5" "Member4"
)

if [ -z "$CONTRACT_ADDR" ]; then
  echo "Please set CONTRACT_ADDR in this file." >&2
  exit 1
fi

validate_addr() {
  local a="$1"
  if [[ "$a" =~ [[:space:]] ]]; then
    echo "Invalid address (contains whitespace): $a" >&2
    exit 1
  fi
  if [[ "$a" == *.* ]]; then
    echo "Invalid address (contains dots/ellipsis): $a" >&2
    exit 1
  fi
  if [[ "$a" != addr_safro1* ]]; then
    echo "Invalid address (wrong prefix): $a" >&2
    exit 1
  fi
}

validate_addr "$CONTRACT_ADDR"

add_member() {
  local addr="$1" pseudo="$2"
  # Build correct ExecuteMsg shape: {"action":{"AddMember":{...}}}
  local EXEC_JSON
  EXEC_JSON=$(jq -n --arg a "$addr" --arg p "$pseudo" '{action:{AddMember:{addr:$a,pseudo:$p}}}')
  echo "Adding member $addr ($pseudo) with caution ${CAUTION}usaf..."
  echo "JSON: $EXEC_JSON"
  $CLI tx wasm execute "$CONTRACT_ADDR" "$EXEC_JSON" \
    --from "$ADMIN_NAME" --keyring-backend file \
    --amount ${CAUTION}usaf \
    --gas auto --gas-prices "$GAS_PRICES" --gas-adjustment "$GAS_ADJ" -y \
    --broadcast-mode sync \
    --chain-id "$CHAIN_ID" --node "$NODE"
}

len=${#MEMBERS[@]}
if (( len % 2 != 0 )); then
  echo "MEMBERS array must contain pairs: <addr> <pseudo>" >&2
  exit 1
fi

for ((i=0; i<len; i+=2)); do
  addr="${MEMBERS[i]}"; pseudo="${MEMBERS[i+1]}"
  validate_addr "$addr"
  add_member "$addr" "$pseudo"
  sleep 2
done

echo "Members added to $CONTRACT_ADDR"
