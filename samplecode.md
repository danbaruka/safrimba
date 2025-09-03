## Safrimba sample code and CLI snippets

### Environment variables (recommended)

```bash
# CLI and network
export CLI=safrochaind
export CHAIN_ID=safro-testnet-1
export NODE=https://rpc.testnet.safrochain.com

# Admin key and address (from your keyring)
export ADMIN_NAME=mycontractadmin
export ADMIN_ADDR=addr_safro18cmxlyj8yllr702kzujhkuwth3rektfpcw8f7p

# Token / fees
export DENOM=usaf
export GAS_PRICES=0.025${DENOM}
export GAS_ADJ=1.3
```

### Build optimized wasm

```bash
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  cosmwasm/rust-optimizer:0.14.0
```

### Store the contract on chain

```bash
$CLI tx wasm store artifacts/safrimba.wasm \
  --from $ADMIN_NAME --keyring-backend file \
  --gas auto --gas-prices $GAS_PRICES --gas-adjustment $GAS_ADJ -y \
  --chain-id $CHAIN_ID --node $NODE

# Get the code id (JSON output)
$CLI query wasm list-code --output json --node $NODE | jq
```

### Minimal instantiate (members later)

```json
{
  "name": "My ROSCA",
  "symbol": "ROSCA1",
  "admin": "addr_safro18cmxlyj8yllr702kzujhkuwth3rektfpcw8f7p",
  "members": [],
  "member_profiles": {},
  "contribution_amount": "1000000",
  "total_cycles": 5,
  "cycle_duration": 2592000,
  "distribution_mode": "Fifo",
  "start_mode": "Auto",
  "start_condition_auto": { "MembersReached": 5 },
  "deposit_deadline": 604800,
  "grace_seconds": 86400,
  "late_penalty_percent": 5,
  "late_strike_limit": 2,
  "distribution_calendar": [],
  "allow_member_exit": true,
  "allow_member_add": true,
  "early_withdrawal_penalty": 10,
  "forbid_overpay": true,
  "forbid_underpay": true
}
```

### Instantiate via CLI (uses admin and pays fees)

```bash
CODE_ID=<code_id>
INIT='{
  "name":"My ROSCA",
  "symbol":"ROSCA1",
  "admin":"'"$ADMIN_ADDR"'",
  "members":[],
  "member_profiles":{},
  "contribution_amount":"1000000",
  "total_cycles":5,
  "cycle_duration":2592000,
  "distribution_mode":"Fifo",
  "start_mode":"Auto",
  "start_condition_auto":{"MembersReached":5},
  "deposit_deadline":604800,
  "grace_seconds":86400,
  "late_penalty_percent":5,
  "late_strike_limit":2,
  "distribution_calendar":[],
  "allow_member_exit":true,
  "allow_member_add":true,
  "early_withdrawal_penalty":10,
  "forbid_overpay":true,
  "forbid_underpay":true
}'

# Broadcast and wait for inclusion (block mode)
$CLI tx wasm instantiate $CODE_ID "$INIT" \
  --label safrimba-1 \
  --admin $ADMIN_ADDR \
  --from $ADMIN_NAME --keyring-backend file \
  --gas auto --gas-prices $GAS_PRICES --gas-adjustment $GAS_ADJ -y \
  --broadcast-mode block \
  --chain-id $CHAIN_ID --node $NODE

# Optionally, if you captured TXHASH, query it:
# $CLI query tx <TXHASH> --output json --node $NODE | jq

# Get the contract address (last contract for this code id)
$CLI query wasm list-contract-by-code $CODE_ID --output json --node $NODE | jq -r '.contracts[-1]'
CONTRACT=$($CLI query wasm list-contract-by-code $CODE_ID --output json --node $NODE | jq -r '.contracts[-1]')
```

### Add members (before start)

```bash
$CLI tx wasm execute $CONTRACT '{"action":{"AddMember":{"addr":"addr_safro1m1...","pseudo":"Alice","profile":"A"}}}' \
  --from $ADMIN_NAME --keyring-backend file \
  --gas auto --gas-prices $GAS_PRICES --gas-adjustment $GAS_ADJ -y \
  --chain-id $CHAIN_ID --node $NODE

$CLI tx wasm execute $CONTRACT '{"action":{"AddMember":{"addr":"addr_safro1m2...","pseudo":"Bob","profile":"B"}}}' \
  --from $ADMIN_NAME --keyring-backend file \
  --gas auto --gas-prices $GAS_PRICES --gas-adjustment $GAS_ADJ -y \
  --chain-id $CHAIN_ID --node $NODE

$CLI tx wasm execute $CONTRACT '{"action":{"AddMember":{"addr":"addr_safro1m3...","pseudo":"Carol","profile":"C"}}}' \
  --from $ADMIN_NAME --keyring-backend file \
  --gas auto --gas-prices $GAS_PRICES --gas-adjustment $GAS_ADJ -y \
  --chain-id $CHAIN_ID --node $NODE
```

### Compute distribution calendar (locks at start)

```bash
$CLI tx wasm execute $CONTRACT '{"action":{"ComputeDistributionCalendar":{}}}' \
  --from $ADMIN_NAME --keyring-backend file \
  --gas auto --gas-prices $GAS_PRICES --gas-adjustment $GAS_ADJ -y \
  --chain-id $CHAIN_ID --node $NODE
```

### Start the tontine

```bash
# Manual start
$CLI tx wasm execute $CONTRACT '{"action":{"InitiateStart":{}}}' \
  --from $ADMIN_NAME --keyring-backend file \
  --gas auto --gas-prices $GAS_PRICES --gas-adjustment $GAS_ADJ -y \
  --chain-id $CHAIN_ID --node $NODE

# Or auto start (once conditions are met)
$CLI tx wasm execute $CONTRACT '{"action":{"AutoStart":{}}}' \
  --from $ADMIN_NAME --keyring-backend file \
  --gas auto --gas-prices $GAS_PRICES --gas-adjustment $GAS_ADJ -y \
  --chain-id $CHAIN_ID --node $NODE
```

### Member deposit for cycle N

```bash
# Replace <member_key> with the member's local key name (in your keyring)
N=1
$CLI tx wasm execute $CONTRACT '{"action":{"Deposit":{"cycle":'$N'}}}' \
  --from <member_key> --keyring-backend file \
  --amount 1000000${DENOM} \
  --gas auto --gas-prices $GAS_PRICES --gas-adjustment $GAS_ADJ -y \
  --chain-id $CHAIN_ID --node $NODE
```

### Record late payment (manual, optional)

```bash
$CLI tx wasm execute $CONTRACT '{"action":{"RecordLatePayment":{"member":"addr_safro1m2..."}}}' \
  --from $ADMIN_NAME --keyring-backend file \
  --gas auto --gas-prices $GAS_PRICES --gas-adjustment $GAS_ADJ -y \
  --chain-id $CHAIN_ID --node $NODE
```

### Trigger payout (admin)

```bash
N=1
$CLI tx wasm execute $CONTRACT '{"action":{"TriggerPayout":{"cycle":'$N'}}}' \
  --from $ADMIN_NAME --keyring-backend file \
  --gas auto --gas-prices $GAS_PRICES --gas-adjustment $GAS_ADJ -y \
  --chain-id $CHAIN_ID --node $NODE
```

### Distribute penalties (carry-over to next payout)

```bash
$CLI tx wasm execute $CONTRACT '{"action":{"DistributePenalties":{}}}' \
  --from $ADMIN_NAME --keyring-backend file \
  --gas auto --gas-prices $GAS_PRICES --gas-adjustment $GAS_ADJ -y \
  --chain-id $CHAIN_ID --node $NODE
```

### Exit flow

```bash
# Member requests exit
$CLI tx wasm execute $CONTRACT '{"action":{"RequestExit":{}}}' \
  --from <member_key> --keyring-backend file \
  --gas auto --gas-prices $GAS_PRICES --gas-adjustment $GAS_ADJ -y \
  --chain-id $CHAIN_ID --node $NODE

# Admin processes exits
$CLI tx wasm execute $CONTRACT '{"action":{"ProcessExit":{}}}' \
  --from $ADMIN_NAME --keyring-backend file \
  --gas auto --gas-prices $GAS_PRICES --gas-adjustment $GAS_ADJ -y \
  --chain-id $CHAIN_ID --node $NODE
```

### Queries

```bash
# Group info
$CLI query wasm contract-state smart $CONTRACT '{"query":{"GetGroupInfo":{}}}' --output json --node $NODE | jq

# Member info
$CLI query wasm contract-state smart $CONTRACT '{"query":{"GetMemberInfo":{"addr":"addr_safro1m1..."}}}' --output json --node $NODE | jq

# Cycle info
$CLI query wasm contract-state smart $CONTRACT '{"query":{"GetCycleInfo":{"cycle":1}}}' --output json --node $NODE | jq

# Distribution calendar
$CLI query wasm contract-state smart $CONTRACT '{"query":{"GetDistributionCalendar":{}}}' --output json --node $NODE | jq
```

### Maintenance / governance

```bash
# Remove member (admin)
$CLI tx wasm execute $CONTRACT '{"action":{"RemoveMember":{"addr":"addr_safro1m2..."}}}' \
  --from $ADMIN_NAME --keyring-backend file \
  --gas auto --gas-prices $GAS_PRICES --gas-adjustment $GAS_ADJ -y \
  --chain-id $CHAIN_ID --node $NODE

# Replace member (admin)
$CLI tx wasm execute $CONTRACT '{"action":{"ReplaceMember":{"old_addr":"addr_safro1m2...","new_addr":"addr_safro1m4..."}}}' \
  --from $ADMIN_NAME --keyring-backend file \
  --gas auto --gas-prices $GAS_PRICES --gas-adjustment $GAS_ADJ -y \
  --chain-id $CHAIN_ID --node $NODE

# Update profile
$CLI tx wasm execute $CONTRACT '{"action":{"UpdateProfile":{"addr":"addr_safro1m3...","pseudo":"Carol2","profile":"Updated"}}}' \
  --from $ADMIN_NAME --keyring-backend file \
  --gas auto --gas-prices $GAS_PRICES --gas-adjustment $GAS_ADJ -y \
  --chain-id $CHAIN_ID --node $NODE
```
