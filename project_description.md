# Safrimba Tontine Smart Contract

A decentralized **Likelemba/ROSCA (Rotating Savings and Credit Association)** built on **Safrochain**, utilizing the native **usaf** token. The Safrimba smart contract enables community members to form trustless, transparent savings groups, contribute periodically, and receive automated lump-sum payouts in a fair, rule-based manner.

---

## 📖 Overview

Safrimba brings the traditional ROSCA model to the blockchain, offering a secure, transparent, and automated solution for community-based savings and payouts. Designed for accessibility and fairness, it ensures that contributions are enforced, payouts are distributed according to a predefined schedule, and penalties are applied automatically for non-compliance. The contract is ideal for communities seeking a decentralized alternative to traditional tontine systems.

---

## 🏷️ Key Information

- **Token**: usaf (native token of Safrochain)
- **Address Prefix**: `addr_safro`
- **Contract Name**: `safrimba`
- **Type**: Group Savings & Distribution (ROSCA/Tontine)

---

## 🌍 Motivation

Traditional ROSCAs rely on trust and manual coordination, which can lead to disputes, mismanagement, or exclusion. Safrimba leverages blockchain technology to:

- Automate contributions and payouts, reducing human error.
- Ensure transparency through an immutable distribution calendar.
- Enforce rules fairly with smart contract logic.
- Empower communities with a decentralized, trustless savings mechanism.

---

## ⚙️ Initialization Parameters

The contract is initialized with the following parameters to define the tontine's structure and rules:

| Parameter                  | Type     | Description                                                         |
| -------------------------- | -------- | ------------------------------------------------------------------- |
| `name`                     | String   | Name of the tontine group.                                          |
| `symbol`                   | String   | Ticker symbol for group identification.                             |
| `admin`                    | Addr     | Address of the group admin/creator.                                 |
| `members`                  | [Addr]   | List of initial member addresses.                                   |
| `member_profiles`          | Map      | Optional metadata for members (e.g., pseudonym, description).       |
| `contribution_amount`      | Uint128  | Fixed contribution amount in usaf per cycle per member.             |
| `total_cycles`             | u32      | Total number of payout cycles.                                      |
| `cycle_duration`           | Duration | Duration of each cycle (e.g., 30 days).                             |
| `distribution_mode`        | Enum     | Payout order: `fifo` (join order), `random`, or `custom`.           |
| `start_mode`               | Enum     | Start trigger: `manual` (admin starts) or `auto` (condition-based). |
| `start_condition_auto`     | Enum     | If `auto`: either `members_reached=N` or `start_date=Timestamp`.    |
| `deposit_deadline`         | Duration | Time window for contributions each cycle.                           |
| `grace_seconds`            | u32      | Grace period for late deposits (in seconds).                        |
| `late_penalty_percent`     | u8       | Penalty percentage for late contributions.                          |
| `late_strike_limit`        | u8       | Maximum late payments before member exclusion.                      |
| `distribution_calendar`    | [Addr]   | Precomputed payout order (generated at initialization).             |
| `allow_member_exit`        | bool     | Whether members can exit voluntarily before completion.             |
| `allow_member_add`         | bool     | Whether new members can be added mid-cycle.                         |
| `early_withdrawal_penalty` | u8       | Penalty percentage for early member exit.                           |
| `forbid_overpay`           | bool     | Rejects deposits exceeding `contribution_amount`.                   |
| `forbid_underpay`          | bool     | Rejects deposits below `contribution_amount`.                       |

---

## 📦 Core Functions

### 🔹 Membership Management

- `add_member {addr, pseudo, profile}`: Adds a new member (if `allow_member_add` is true).
- `remove_member {addr}`: Removes a member (by admin or governance rules).
- `update_profile {addr, pseudo, profile}`: Updates a member’s metadata.

### 🔹 Tontine Lifecycle

- `initiate_start {}`: Manually starts the tontine (if `start_mode = manual`).
- `auto_start {}`: Automatically starts when conditions are met (e.g., member threshold or start date).
- `compute_distribution_calendar {}`: Generates and locks the payout schedule based on `distribution_mode`.

### 🔹 Contributions

- `deposit {cycle}`: Allows a member to deposit `contribution_amount` in usaf for the specified cycle.
- `record_late_payment {member}`: Automatically applies penalties for late deposits.
- `apply_penalty {member, percent}`: Deducts the specified penalty percentage from a member’s contribution.

### 🔹 Payouts

- `trigger_payout {cycle}`: Transfers the total contributions to the scheduled recipient for the cycle.
- `distribute_penalties {}`: Distributes collected penalties to a group reserve or the next payout.
- `view_calendar {}`: Returns the full payout schedule.

### 🔹 Exit & Recovery

- `request_exit {}`: Allows a member to request an exit (if `allow_member_exit` is true).
- `process_exit {}`: Processes the exit, applying penalties and refunding remaining contributions.
- `replace_member {old_addr, new_addr}`: Replaces a non-compliant member with a new one.

### 🔹 Queries

- `get_group_info {}`: Returns tontine metadata (name, symbol, admin, etc.).
- `get_member_info {addr}`: Returns a member’s contributions, strikes, and profile.
- `get_cycle_info {cycle}`: Returns cycle-specific details (status, recipient, penalties).
- `get_distribution_calendar {}`: Returns the full payout schedule.

---

## 🔄 Automated Workflow

1. **Cycle Start**:

   - The tontine begins based on the `start_mode` (manual or auto).
   - For `auto`, the contract checks if conditions (`members_reached` or `start_date`) are met.

2. **Contribution Enforcement**:

   - Members must deposit exactly `contribution_amount` before the `deposit_deadline`.
   - A `grace_seconds` period allows late deposits with penalties.
   - Non-compliance triggers automatic penalties or exclusion.

3. **Penalty System**:

   - Late deposits incur a `late_penalty_percent` deduction.
   - Each late payment adds a strike; exceeding `late_strike_limit` results in exclusion.
   - Penalties are stored in a reserve or distributed per the contract’s rules.

4. **Payouts**:

   - At the end of each cycle, the contract transfers the total contributions to the recipient in the `distribution_calendar`.
   - Payouts only occur if all contributions (or penalties) are processed.

5. **Exit Management**:

   - If `allow_member_exit` is true, members can exit with an `early_withdrawal_penalty`.
   - Exits are rejected if disallowed or if conditions are unmet.

6. **Completion**:
   - After `total_cycles`, the contract marks itself as `Completed`.
   - Remaining funds (e.g., penalties or reserves) are distributed proportionally or to the final recipient.

---

## 📊 Example Workflow (FIFO, 5 Members)

1. **Initialization**:

   - Members: A, B, C, D, E
   - Contribution: 100 usaf per cycle
   - Total cycles: 5
   - Distribution mode: FIFO

2. **Cycle 1**:

   - All members deposit 100 usaf → Total = 500 usaf
   - A receives 500 usaf
   - Next recipient: B

3. **Cycle 2**:

   - All members deposit 100 usaf → Total = 500 usaf
   - B receives 500 usaf

4. **Subsequent Cycles**:
   - Continues until Cycle 5, when E receives 500 usaf.
   - Contract marks itself as `Completed`.

---

## ✅ Security & Fairness

- **Restricted Access**: Only predefined members can deposit.
- **Contribution Validation**: Overpayments or underpayments are rejected if `forbid_overpay` or `forbid_underpay` is true.
- **Automated Penalties**: Late deposits are penalized automatically, ensuring fairness.
- **Transparent Payouts**: The `distribution_calendar` is immutable once set, guaranteeing a fair payout order.
- **No Partial Payouts**: Payouts are only executed when all contributions (or penalties) are collected.
- **Auditability**: All actions and states are queryable, ensuring transparency.

---

## 🚀 Getting Started

To deploy or interact with the Safrimba contract:

1. Deploy the contract on Safrochain with the desired initialization parameters.
2. Add members and configure the tontine rules.
3. Start the tontine (manually or automatically).
4. Use the provided functions to manage contributions, payouts, and queries.

For API access or integration, visit [xAI API](https://x.ai/api).

---

## 📝 Notes

- Ensure all members have sufficient usaf tokens before participating.
- The admin should carefully configure parameters like `late_penalty_percent` and `late_strike_limit` to balance fairness and flexibility.
- The `distribution_calendar` is generated at initialization and cannot be altered, ensuring trustless execution.

docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")\_cache",target=/code/target \
 cosmwasm/rust-optimizer:0.14.0
