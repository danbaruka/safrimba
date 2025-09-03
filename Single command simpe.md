## Safrimba single command examples (no shell variables)

Replace angle-bracket placeholders like 30, addr_safro1armcr7lme0ac3l59dx6ne3et3pe9mfyv57d8pc9m5tyxtzm64dpquqgxcx, <MEMBER_KEY_NAME> with your actual values before running.

RPC endpoint: https://rpc.testnet.safrochain.com
CHAIN_ID: safro-testnet-1

### 1) Store contract (upload wasm)

```bash
safrochaind tx wasm store artifacts/safrimba.wasm --from mycontractadmin --keyring-backend file --gas auto --gas-prices 0.025usaf --gas-adjustment 1.3 -y --chain-id safro-testnet-1 --node https://rpc.testnet.safrochain.com
```

### 2) Get code list to find CODE_ID

```bash
safrochaind query wasm list-code --output json --node https://rpc.testnet.safrochain.com | jq
```

### 3) Instantiate (minimal; members added later)

```bash
safrochaind tx wasm instantiate 31 '{
  "name":"My ROSCA",
  "symbol":"ROSCA1",
  "admin":"addr_safro18cmxlyj8yllr702kzujhkuwth3rektfpcw8f7p",
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
}' --label safrimba-1 --admin addr_safro18cmxlyj8yllr702kzujhkuwth3rektfpcw8f7p --from mycontractadmin --keyring-backend file --gas auto --gas-prices 0.025usaf --gas-adjustment 1.3 -y --broadcast-mode sync --chain-id safro-testnet-1 --node https://rpc.testnet.safrochain.com
```

### 4) Get contract address from CODE_ID

```bash
safrochaind query wasm list-contract-by-code 31 --output json --node https://rpc.testnet.safrochain.com | jq -r '.contracts[-1]'
```

### 5) Add members (run once per member) BEFORE start

```bash
safrochaind tx wasm execute addr_safro1armcr7lme0ac3l59dx6ne3et3pe9mfyv57d8pc9m5tyxtzm64dpquqgxcx '{"action":{"AddMember":{"addr":"addr_safro1member1...","pseudo":"Alice","profile":"A"}}}' --from mycontractadmin --keyring-backend file --gas auto --gas-prices 0.025usaf --gas-adjustment 1.3 -y --chain-id safro-testnet-1 --node https://rpc.testnet.safrochain.com

safrochaind tx wasm execute addr_safro1armcr7lme0ac3l59dx6ne3et3pe9mfyv57d8pc9m5tyxtzm64dpquqgxcx '{"action":{"AddMember":{"addr":"addr_safro1member2...","pseudo":"Bob","profile":"B"}}}' --from mycontractadmin --keyring-backend file --gas auto --gas-prices 0.025usaf --gas-adjustment 1.3 -y --chain-id safro-testnet-1 --node https://rpc.testnet.safrochain.com

safrochaind tx wasm execute addr_safro1armcr7lme0ac3l59dx6ne3et3pe9mfyv57d8pc9m5tyxtzm64dpquqgxcx '{"action":{"AddMember":{"addr":"addr_safro1member3...","pseudo":"Carol","profile":"C"}}}' --from mycontractadmin --keyring-backend file --gas auto --gas-prices 0.025usaf --gas-adjustment 1.3 -y --chain-id safro-testnet-1 --node https://rpc.testnet.safrochain.com
```

### 6) Compute distribution calendar (allowed until start)

```bash
safrochaind tx wasm execute addr_safro1armcr7lme0ac3l59dx6ne3et3pe9mfyv57d8pc9m5tyxtzm64dpquqgxcx '{"action":{"ComputeDistributionCalendar":{}}}' --from mycontractadmin --keyring-backend file --gas auto --gas-prices 0.025usaf --gas-adjustment 1.3 -y --chain-id safro-testnet-1 --node https://rpc.testnet.safrochain.com
```

### 7) Start (manual)

```bash
safrochaind tx wasm execute addr_safro1armcr7lme0ac3l59dx6ne3et3pe9mfyv57d8pc9m5tyxtzm64dpquqgxcx '{"action":{"InitiateStart":{}}}' --from mycontractadmin --keyring-backend file --gas auto --gas-prices 0.025usaf --gas-adjustment 1.3 -y --chain-id safro-testnet-1 --node https://rpc.testnet.safrochain.com
```

### 8) Start (auto) when conditions are met

```bash
safrochaind tx wasm execute addr_safro1armcr7lme0ac3l59dx6ne3et3pe9mfyv57d8pc9m5tyxtzm64dpquqgxcx '{"action":{"AutoStart":{}}}' --from mycontractadmin --keyring-backend file --gas auto --gas-prices 0.025usaf --gas-adjustment 1.3 -y --chain-id safro-testnet-1 --node https://rpc.testnet.safrochain.com
```

### 9) Member deposit for cycle N (replace with member's local key)

```bash
safrochaind tx wasm execute addr_safro1armcr7lme0ac3l59dx6ne3et3pe9mfyv57d8pc9m5tyxtzm64dpquqgxcx '{"action":{"Deposit":{"cycle":1}}}' --from <MEMBER_KEY_NAME> --keyring-backend file --amount 1000000usaf --gas auto --gas-prices 0.025usaf --gas-adjustment 1.3 -y --chain-id safro-testnet-1 --node https://rpc.testnet.safrochain.com
```

### 10) Trigger payout (admin)

```bash
safrochaind tx wasm execute addr_safro1armcr7lme0ac3l59dx6ne3et3pe9mfyv57d8pc9m5tyxtzm64dpquqgxcx '{"action":{"TriggerPayout":{"cycle":1}}}' --from mycontractadmin --keyring-backend file --gas auto --gas-prices 0.025usaf --gas-adjustment 1.3 -y --chain-id safro-testnet-1 --node https://rpc.testnet.safrochain.com
```

### 11) Distribute penalties (carry to next payout)

```bash
safrochaind tx wasm execute addr_safro1armcr7lme0ac3l59dx6ne3et3pe9mfyv57d8pc9m5tyxtzm64dpquqgxcx '{"action":{"DistributePenalties":{}}}' --from mycontractadmin --keyring-backend file --gas auto --gas-prices 0.025usaf --gas-adjustment 1.3 -y --chain-id safro-testnet-1 --node https://rpc.testnet.safrochain.com
```

### 12) Exit flow

```bash
# Member requests exit
safrochaind tx wasm execute addr_safro1armcr7lme0ac3l59dx6ne3et3pe9mfyv57d8pc9m5tyxtzm64dpquqgxcx '{"action":{"RequestExit":{}}}' --from <MEMBER_KEY_NAME> --keyring-backend file --gas auto --gas-prices 0.025usaf --gas-adjustment 1.3 -y --chain-id safro-testnet-1 --node https://rpc.testnet.safrochain.com

# Admin processes exits
safrochaind tx wasm execute addr_safro1armcr7lme0ac3l59dx6ne3et3pe9mfyv57d8pc9m5tyxtzm64dpquqgxcx '{"action":{"ProcessExit":{}}}' --from mycontractadmin --keyring-backend file --gas auto --gas-prices 0.025usaf --gas-adjustment 1.3 -y --chain-id safro-testnet-1 --node https://rpc.testnet.safrochain.com
```

### 13) Queries

```bash
# Group info
safrochaind query wasm contract-state smart addr_safro1armcr7lme0ac3l59dx6ne3et3pe9mfyv57d8pc9m5tyxtzm64dpquqgxcx '{"query":{"GetGroupInfo":{}}}' --output json --node https://rpc.testnet.safrochain.com | jq

# Member info
safrochaind query wasm contract-state smart addr_safro1armcr7lme0ac3l59dx6ne3et3pe9mfyv57d8pc9m5tyxtzm64dpquqgxcx '{"query":{"GetMemberInfo":{"addr":"addr_safro1member1..."}}}' --output json --node https://rpc.testnet.safrochain.com | jq

# Cycle info
safrochaind query wasm contract-state smart addr_safro1armcr7lme0ac3l59dx6ne3et3pe9mfyv57d8pc9m5tyxtzm64dpquqgxcx '{"query":{"GetCycleInfo":{"cycle":1}}}' --output json --node https://rpc.testnet.safrochain.com | jq

# Distribution calendar
safrochaind query wasm contract-state smart addr_safro1armcr7lme0ac3l59dx6ne3et3pe9mfyv57d8pc9m5tyxtzm64dpquqgxcx '{"query":{"GetDistributionCalendar":{}}}' --output json --node https://rpc.testnet.safrochain.com | jq
```
