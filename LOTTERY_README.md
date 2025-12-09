# 🎰 Chainlink VRF Lottery - Scaffold Stylus

A decentralized lottery system built with **Arbitrum Stylus** (Rust/WASM) and **Chainlink VRF** for verifiable random winner selection.

## 📋 Overview

This project demonstrates how to integrate **Chainlink VRF (Verifiable Random Function)** with smart contracts written in Rust using Stylus. The application includes:

- ✅ **Lottery Smart Contract** in Rust/Stylus
- ✅ Complete **Web Interface** React/Next.js
- ✅ **Chainlink VRF Integration** Direct Funding
- ✅ **Automatic multi-round system**
- ✅ **Automatic payment** to winner

## 🏗️ Architecture

### Smart Contracts (Rust/Stylus)

#### 1. VRF Consumer (`packages/stylus/vrf-consumer/`)

Base contract that implements Chainlink VRF V2+ integration:

- Requests verifiable random numbers
- Receives callback from VRF Wrapper
- Manages payments in native ETH

#### 2. Lottery Contract (`packages/stylus/lottery/`)

Main lottery contract that uses VRF:

- **Player entry** with fee payment
- **Draw initiation** by owner
- **Random winner selection** via VRF
- **Automatic prize payment**
- **History** of previous rounds

### Frontend (Next.js/React)

#### Lottery Interface (`packages/nextjs/app/lottery/`)

- Real-time lottery status
- Player entry
- Administrator controls
- Winner history
- Prize pool visualization

## 🚀 How to Use

### Prerequisites

```bash
# Node.js 18+
node --version

# Yarn
yarn --version

# Rust and Cargo Stylus
rustup --version
cargo stylus --version
```

### 1. Installation

```bash
# Clone the repository
git clone <your-repo>
cd vrf-stylus

# Install dependencies
yarn install
```

### 2. Configuration

Create `.env` file in `packages/stylus/`:

```env
# Arbitrum Sepolia (Testnet)
RPC_URL_SEPOLIA=https://sepolia-rollup.arbitrum.io/rpc
PRIVATE_KEY_SEPOLIA=your_private_key
ACCOUNT_ADDRESS_SEPOLIA=your_address

# Or Arbitrum One (Mainnet)
RPC_URL_MAINNET=https://arb1.arbitrum.io/rpc
PRIVATE_KEY_MAINNET=your_private_key
ACCOUNT_ADDRESS_MAINNET=your_address
```

### 3. Deploy Contracts

```bash
# Deploy on Arbitrum Sepolia
cd packages/stylus
yarn deploy --network sepolia

# Note the deployed contract addresses
# VRF Consumer: 0x...
# Lottery: 0x...
```

Contracts will be saved in `packages/stylus/deployments/`.

### 4. Configure Frontend

Update `packages/nextjs/scaffold.config.ts`:

```typescript
const scaffoldConfig = {
  targetNetworks: [chains.arbitrumSepolia], // or arbitrumOne
  // ...
};
```

### 5. Start Frontend

```bash
cd packages/nextjs
yarn start
```

Access `http://localhost:3000/lottery`

## 🎮 How to Play

### For Players

1. **Connect Wallet**

   - Click "Connect Wallet"
   - Select your wallet (MetaMask, etc.)
   - Make sure you're on Arbitrum Sepolia network

2. **Get Test ETH**

   - Visit [Chainlink Faucet](https://faucets.chain.link/arbitrum-sepolia)
   - Or [QuickNode Faucet](https://faucet.quicknode.com/arbitrum/sepolia)

3. **Enter Lottery**

   - Check the entry fee (Entry Fee)
   - Click "Enter Lottery"
   - Confirm transaction
   - Wait for confirmation

4. **Wait for Draw**
   - Administrator will start the draw when there are enough players
   - Chainlink VRF will return a random number (3-5 blocks)
   - Winner will be selected automatically
   - Prize will be transferred automatically

### For Administrators (Owner)

1. **Fund Contract**

   ```
   - Click "Fund Contract"
   - Send ETH to pay VRF fees
   - Recommended: 0.05 ETH or more
   ```

2. **Start Draw**

   ```
   - Wait for at least 1 player
   - Click "Start Draw"
   - Confirm transaction
   - VRF will be requested automatically
   ```

3. **Update Entry Fee** (Optional)
   ```solidity
   // Call set_entry_fee(new_fee) via Debug Contracts
   ```

## 📊 Contract Features

### Main Functions

#### `enter_lottery()`

```rust
// Player enters lottery by paying fee
// Requires: msg.value >= entry_fee
// Emits: LotteryEntered
```

#### `start_draw()`

```rust
// Owner starts the draw
// Requires: lottery_open && players.len() > 0
// Requests randomness from VRF
// Emits: DrawStarted, VRFRequestSent
```

#### `raw_fulfill_random_words(request_id, random_words)`

```rust
// VRF Wrapper callback
// Selects winner using modulo
// Transfers prize automatically
// Resets for next round
// Emits: WinnerSelected, VRFRequestFulfilled
```

### View Functions

```rust
get_entry_fee() -> U256
get_current_lottery_id() -> U256
is_lottery_open() -> bool
get_players_count() -> U256
get_prize_pool() -> U256
get_lottery_round(lottery_id) -> (lottery_id, prize_pool, winner, random_word, players_count, timestamp, completed)
```

## 🔧 Development

### Compile Contracts

```bash
cd packages/stylus/vrf-consumer
cargo build --target wasm32-unknown-unknown --release

cd ../lottery
cargo build --target wasm32-unknown-unknown --release
```

### Export ABIs

```bash
cd packages/stylus
yarn export-abi --contract vrf-consumer
yarn export-abi --contract lottery
```

### Tests

```bash
# Frontend
cd packages/nextjs
yarn test

# Contracts (when implemented)
cd packages/stylus/lottery
cargo test
```

## 📝 Events

### LotteryEntered

```solidity
event LotteryEntered(address indexed player, uint256 indexed lotteryId, uint256 entryFee);
```

### DrawStarted

```solidity
event DrawStarted(uint256 indexed lotteryId, uint256 indexed requestId, uint256 playersCount);
```

### WinnerSelected

```solidity
event WinnerSelected(uint256 indexed lotteryId, address indexed winner, uint256 prizeAmount, uint256 randomWord);
```

### VRFRequestSent

```solidity
event VRFRequestSent(uint256 indexed requestId, uint32 numWords);
```

### VRFRequestFulfilled

```solidity
event VRFRequestFulfilled(uint256 indexed requestId, uint256[] randomWords, uint256 payment);
```

## 🛡️ Security

### Security Features

1. **Verifiable Randomness**

   - Uses Chainlink VRF to ensure cryptographically secure randomness
   - Impossible to predict or manipulate the outcome

2. **Controlled Access**

   - `start_draw()` function restricted to owner
   - `raw_fulfill_random_words()` callback accepts only from VRF Wrapper

3. **Validations**

   - Verifies minimum entry fee
   - Verifies lottery state (open/closed)
   - Verifies number of players before draw

4. **Transparency**
   - All events are emitted on-chain
   - Round history stored permanently
   - Random numbers publicly verifiable

## 🌐 Network Addresses

### Arbitrum Sepolia (Testnet)

- **VRF V2 Plus Wrapper**: `0x29576aB8152A09b9DC634804e4aDE73dA1f3a3CC`
- **Chain ID**: `421614`
- **Explorer**: https://sepolia.arbiscan.io/

### Arbitrum One (Mainnet)

- **VRF V2 Plus Wrapper**: `TBD` (check Chainlink documentation)
- **Chain ID**: `42161`
- **Explorer**: https://arbiscan.io/

## 📚 Resources

- [Stylus Documentation](https://docs.arbitrum.io/stylus/gentle-introduction)
- [Scaffold-Stylus Docs](https://arb-stylus.github.io/scaffold-stylus-docs/)
- [Chainlink VRF Docs](https://docs.chain.link/vrf)
- [OpenZeppelin Stylus](https://github.com/OpenZeppelin/rust-contracts-stylus)

## 🤝 Contributing

Contributions are welcome! Please:

1. Fork the project
2. Create a branch for your feature (`git checkout -b feature/NewFeature`)
3. Commit your changes (`git commit -m 'Add NewFeature'`)
4. Push to the branch (`git push origin feature/NewFeature`)
5. Open a Pull Request

## 📄 License

This project is under the MIT license. See the [LICENSE](LICENSE) file for more details.

## 🆘 Support

If you encounter problems:

1. Check if you're on the correct network (Arbitrum Sepolia/One)
2. Make sure you have enough ETH for gas fees
3. Verify the contract is funded (for VRF payments)
4. Check browser logs for specific errors

## 🎯 Next Steps

Ideas for expansion:

- [ ] Multiple winners per round
- [ ] Ticket system (multiple entries per player)
- [ ] Accumulated jackpot between rounds
- [ ] NFTs for winners
- [ ] Referral/affiliate system
- [ ] ERC20 token integration
- [ ] Advanced administration interface
- [ ] Analytics and statistics

---

**Built with ❤️ using Scaffold-Stylus, Arbitrum Stylus and Chainlink VRF**
