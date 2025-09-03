# Safrimba Smart Contract

A decentralized **Likelemba/ROSCA (Rotating Savings and Credit Association)** built on **Safrochain**, utilizing the native **usaf** token. The Safrimba smart contract enables community members to form trustless, transparent savings groups, contribute periodically, and receive automated lump-sum payouts in a fair, rule-based manner.

## 🎯 Key Features

- **Trustless Automation**: Smart contract enforces rules without human intervention
- **Transparent Distribution**: Immutable payout calendar ensures fairness
- **Flexible Configuration**: Support for different distribution modes and timing
- **Penalty System**: Automatic late payment penalties and member exclusion
- **Explorer Integration**: Proper query function visibility in blockchain explorers

## 🚀 Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [safrochaind](https://github.com/safrochain/safrochain) CLI tool
- [jq](https://stedolan.github.io/jq/) for JSON processing

### Building the Contract

```bash
# Install Rust target for WASM
rustup target add wasm32-unknown-unknown

# Build the contract
./scripts/build.sh
```

### Deployment

1. **Set up your environment**:
   ```bash
   export CHAIN_ID="safrochain-testnet-1"
   export NODE="https://rpc-testnet.safrochain.com:443"
   export KEY_NAME="your-key-name"
   ```

2. **Deploy the contract**:
   ```bash
   ./scripts/deploy.sh
   ```

3. **Instantiate with your parameters**:
   ```bash
   # Edit examples/init_testnet.json with your parameters
   ./scripts/instantiate.sh examples/init_testnet.json
   ```

## 📚 Contract Interface

### Instantiation Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `name` | String | Name of the tontine group |
| `symbol` | String | Ticker symbol for group identification |
| `admin` | String | Address of the group admin/creator |
| `members` | Array[String] | List of initial member addresses |
| `contribution_amount` | String | Fixed contribution amount in usaf per cycle |
| `total_cycles` | u32 | Total number of payout cycles |
| `cycle_duration` | u64 | Duration of each cycle in seconds |
| `distribution_mode` | Enum | Payout order: `Fifo`, `Random`, or `Custom` |
| `start_mode` | Enum | Start trigger: `Manual` or `Auto` |
| `deposit_deadline` | u64 | Time window for contributions each cycle |
| `grace_seconds` | u32 | Grace period for late deposits |
| `late_penalty_percent` | u8 | Penalty percentage for late contributions |
| `late_strike_limit` | u8 | Maximum late payments before exclusion |
| `allow_member_exit` | bool | Whether members can exit voluntarily |
| `allow_member_add` | bool | Whether new members can be added |
| `max_members` | u32 | Maximum number of members allowed |
| `caution_deposit` | String | Security deposit required from members |

### Execute Messages

#### Membership Management
```json
{
  "action": {
    "AddMember": {
      "addr": "safro1...",
      "pseudo": "MemberName"
    }
  }
}
```

```json
{
  "action": {
    "RemoveMember": {
      "addr": "safro1..."
    }
  }
}
```

#### Tontine Lifecycle
```json
{
  "action": {
    "InitiateStart": {}
  }
}
```

```json
{
  "action": {
    "Deposit": {
      "cycle": 1
    }
  }
}
```

### Query Messages

#### Get Group Information
```json
{
  "GetGroupInfo": {}
}
```

#### Get Member Information  
```json
{
  "GetMemberInfo": {
    "addr": "safro1..."
  }
}
```

#### Get Distribution Calendar
```json
{
  "GetDistributionCalendar": {}
}
```

#### Get Cycle Information
```json
{
  "GetCycleInfo": {
    "cycle": 1
  }
}
```

## 🛠 Development Scripts

### Building
```bash
./scripts/build.sh              # Build optimized WASM
```

### Deployment
```bash
./scripts/deploy.sh             # Deploy to configured network
./scripts/instantiate.sh <msg>  # Instantiate with init message
```

### Interaction
```bash
./scripts/execute.sh <msg>      # Execute transaction
./scripts/query.sh <msg>        # Query contract state
```

### Testing
```bash
cargo test                      # Run unit tests
cargo run --bin schema          # Generate JSON schemas
```

## 📁 Project Structure

```
safrimba/
├── src/
│   ├── contract.rs         # Main contract logic
│   ├── msg.rs             # Message definitions  
│   ├── state.rs           # State structures
│   ├── storage.rs         # Storage helpers
│   ├── schema.rs          # Schema generation
│   └── lib.rs             # Library root
├── scripts/
│   ├── build.sh           # Build script
│   ├── deploy.sh          # Deployment script
│   ├── instantiate.sh     # Instantiation script
│   ├── execute.sh         # Execute script
│   └── query.sh           # Query script
├── examples/
│   ├── init_testnet.json  # Example init message
│   ├── add_member.json    # Example execute messages
│   └── query_*.json       # Example query messages
├── schemas/               # Generated JSON schemas
├── deploy/                # Deployment artifacts
└── artifacts/             # Built WASM files
```

## 🔄 Typical Workflow

1. **Deploy and Instantiate**:
   ```bash
   ./scripts/build.sh
   ./scripts/deploy.sh
   ./scripts/instantiate.sh examples/init_testnet.json
   ```

2. **Add Members** (if allowed):
   ```bash
   ./scripts/execute.sh examples/add_member.json
   ```

3. **Start the Tontine**:
   ```bash
   ./scripts/execute.sh examples/start_tontine.json
   ```

4. **Members Make Deposits**:
   ```bash
   ./scripts/execute.sh examples/deposit_cycle1.json
   ```

5. **Query State**:
   ```bash
   ./scripts/query.sh examples/query_group_info.json
   ./scripts/query.sh examples/query_calendar.json
   ```

## 🔍 Explorer Integration

The contract now includes proper JSON schema generation that makes all query functions visible in blockchain explorers like:

- Safrochain Explorer
- Cosmoscan
- BigDipper
- Ping.pub

Query functions will appear in the "Query" section of contract pages, allowing users to interact with the contract directly through the explorer interface.

## 🔒 Security Considerations

- **Access Control**: Only predefined members can deposit
- **Contribution Validation**: Overpayments/underpayments are configurable
- **Automated Penalties**: Late deposits are penalized automatically
- **Transparent Payouts**: Distribution calendar is immutable once set
- **Auditability**: All actions and states are queryable

## 🐛 Troubleshooting

### Common Issues

1. **"Code ID not found"**: Contract needs to be deployed first
2. **"Contract not instantiated"**: Run instantiate script after deployment
3. **"Insufficient funds"**: Ensure sender has enough usaf tokens
4. **"Member not found"**: Check member addresses are correct

### Getting Help

- Check deployment info in `deploy/deployment-*.json`
- Use query scripts to check contract state
- Review transaction logs for error details

## 📄 License

This project is licensed under the MIT License.

## 🤝 Contributing

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## 📞 Support

For support and questions:
- Create an issue on GitHub
- Contact the development team
- Join our community Discord