# 🚀 Quick Start - VRF Lottery

## ⚡ Quick Start (5 minutes)

### 1. Setup Environment

```bash
# Install dependencies
yarn install

# Configure environment variables
cd packages/stylus
cp .env.example .env
# Edit .env with your keys
```

### 2. Deploy Contracts

```bash
# In packages/stylus/
yarn deploy --network sepolia
```

**Note**: Write down the deployed contract addresses!

### 3. Configure Network

Edit `packages/nextjs/scaffold.config.ts`:

```typescript
import * as chains from "./utils/scaffold-stylus/supportedChains";

const scaffoldConfig = {
  targetNetworks: [chains.arbitrumSepolia],
  // ...
};
```

### 4. Start Frontend

```bash
cd packages/nextjs
yarn start
```

Access: **http://localhost:3000/lottery**

### 5. Test the Lottery

1. **Connect your wallet** (MetaMask on Arbitrum Sepolia)
2. **Get test ETH**: https://faucets.chain.link/arbitrum-sepolia
3. **Fund the contract** (as owner): 0.05 ETH
4. **Enter the lottery**: Pay entry fee (0.01 ETH)
5. **Start the draw** (as owner)
6. **Wait for VRF**: 3-5 blocks (~15-30 seconds)
7. **Winner selected**: Prize transferred automatically! 🎉

## 🎯 Project Structure

```
vrf-stylus/
├── packages/
│   ├── stylus/
│   │   ├── vrf-consumer/    # Base VRF contract
│   │   ├── lottery/          # Lottery contract
│   │   └── scripts/          # Deploy scripts
│   └── nextjs/
│       ├── app/
│       │   ├── lottery/      # Lottery UI ✨
│       │   └── vrf/          # VRF UI
│       └── components/       # React components
└── LOTTERY_README.md         # Complete documentation
```

## 🎮 How It Works

### Lottery Flow

```
1. ENTRY            2. DRAW              3. VRF              4. WINNER
   ↓                    ↓                   ↓                    ↓
Players          Owner starts         Chainlink VRF        Winner selected
pay fee       →  start_draw()    →   returns random   →   Prize paid!
                                       number (3-5 blocks)
```

### Lifecycle

```rust
// State: OPEN
lottery_open = true
players = []

// Players enter
enter_lottery() // payable

// Owner starts
start_draw() // closes lottery, calls VRF

// VRF responds (automatic)
raw_fulfill_random_words(requestId, randomWords)
  → selects winner
  → pays prize
  → resets for next round

// State: OPEN again
lottery_id++
players = []
lottery_open = true
```

## 💡 Important Tips

### For Players

- ✅ Use Arbitrum Sepolia for testing
- ✅ Keep enough ETH for gas + entry fee
- ✅ Wait for draw to be started by owner
- ✅ Winner receives prize automatically

### For Owners/Developers

- ✅ **ALWAYS fund the contract first** (0.05+ ETH)
- ✅ VRF requires payment in native ETH
- ✅ Wait for at least 1 player before start_draw()
- ✅ VRF takes 3-5 blocks to respond (~15-30s)
- ✅ Can adjust entry_fee via set_entry_fee()

## 🔍 Debug and Troubleshooting

### Error: "Insufficient Entry Fee"

**Solution**: Send at least the entry fee (`get_entry_fee()`)

### Error: "Lottery Not Open"

**Solution**: Wait for current round to finish

### Error: "No Players in Lottery"

**Solution**: Need at least 1 player to draw

### Error: Transaction Failed (VRF)

**Solution**: Make sure contract has ETH to pay VRF

### VRF doesn't respond

**Solution**:

- Wait 3-5 blocks (may take up to 1 minute)
- Check if you're on Arbitrum Sepolia/One
- Confirm contract has balance

## 📊 Monitoring

### Via Frontend

- Lottery status (Open/Closed)
- Number of players
- Current prize pool
- Winner history

### Via Block Explorer

1. Go to https://sepolia.arbiscan.io/
2. Paste Lottery contract address
3. See transactions and events

### Important Events

```solidity
LotteryEntered   // Someone entered
DrawStarted      // Draw started
VRFRequestSent   // VRF requested
VRFRequestFulfilled  // VRF responded
WinnerSelected   // Winner chosen! 🏆
```

## 🎨 Customization

### Change Entry Fee

```typescript
// Via Debug Contracts or direct call
await lottery.set_entry_fee(parseEther("0.02")); // 0.02 ETH
```

### Configure VRF

Edit constructor in `lottery/src/lib.rs`:

```rust
self.callback_gas_limit.set(U32::from(200000)); // Gas
self.request_confirmations.set(U16::from(3));   // Confirmations
self.num_words.set(U32::from(1));               // Words
```

## 📱 Testing on Mobile

1. Use MetaMask Mobile
2. Connect to Arbitrum Sepolia
3. Access via ngrok or deploy

```bash
# Install ngrok
npm install -g ngrok

# Expose localhost
ngrok http 3000
```

## 🚢 Production Deploy

### 1. Deploy Arbitrum One (Mainnet)

```bash
# Configure .env with mainnet keys
yarn deploy --network mainnet
```

### 2. Deploy Frontend (Vercel)

```bash
cd packages/nextjs
vercel deploy
```

### 3. Update Settings

- Change `targetNetworks` to `arbitrumOne`
- Update contract addresses
- Configure environment variables in Vercel

## 📈 Next Features

- [ ] Complete round history in UI
- [ ] Notifications when draw starts
- [ ] Multiple tickets system
- [ ] Accumulated prize pool
- [ ] The Graph integration for queries
- [ ] Mobile app (React Native)

## 🤝 Community

- **Discord**: [Arbitrum](https://discord.gg/arbitrum)
- **Telegram**: [Stylus Developers]
- **Twitter**: [@arbitrum](https://twitter.com/arbitrum)

## 📚 Learn More

- [Stylus Docs](https://docs.arbitrum.io/stylus)
- [Chainlink VRF](https://docs.chain.link/vrf)
- [Scaffold-Stylus](https://arb-stylus.github.io/scaffold-stylus-docs/)
- [Rust Book](https://doc.rust-lang.org/book/)

---

**Questions?** Open an issue on GitHub or ask in the Arbitrum community!

**Good luck in the lottery! 🍀**
