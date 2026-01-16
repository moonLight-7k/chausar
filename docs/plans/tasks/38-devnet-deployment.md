# Task: Devnet Deployment

Metadata:
- Dependencies: All phases 1-9 complete
- Provides: Deployed application on devnet
- Size: Small (2-3 files)

## Implementation Content
Deploy the complete prediction market to Solana devnet:
- Deploy Anchor program to devnet
- Configure frontend for devnet
- Set up environment variables
- Test full flow on devnet
- Document deployment process

## Target Files
- [ ] `anchor/Anchor.toml` (update for devnet)
- [ ] `.env.production` (create)
- [ ] `app/lib/config.ts` (update)
- [ ] `docs/deployment.md` (create)

## Implementation Steps

### 1. Program Deployment Preparation
- [ ] Ensure program builds successfully
- [ ] Verify all tests pass
- [ ] Get devnet SOL for deployment

```bash
# Get devnet SOL
solana airdrop 5 --url devnet

# Verify balance
solana balance --url devnet
```

### 2. Update Anchor Configuration
```toml
# anchor/Anchor.toml
[features]
seeds = false
skip-lint = false

[programs.devnet]
prediction = "PROGRAM_ID_WILL_BE_HERE"

[registry]
url = "https://api.apr.dev"

[provider]
cluster = "devnet"
wallet = "~/.config/solana/id.json"

[scripts]
test = "yarn run ts-mocha -p ./tsconfig.json -t 1000000 tests/**/*.ts"
```

### 3. Deploy Program
```bash
# Build program
cd anchor
anchor build

# Deploy to devnet
anchor deploy --provider.cluster devnet

# Note the program ID from output
```

### 4. Update Frontend Configuration
```typescript
// app/lib/config.ts
export const NETWORK = process.env.NEXT_PUBLIC_NETWORK || "devnet";

export const RPC_ENDPOINTS = {
  devnet: process.env.NEXT_PUBLIC_RPC_URL || "https://api.devnet.solana.com",
  mainnet: "https://api.mainnet-beta.solana.com",
};

export const RPC_ENDPOINT = RPC_ENDPOINTS[NETWORK as keyof typeof RPC_ENDPOINTS];

// Program ID from deployment
export const PROGRAM_ID = process.env.NEXT_PUBLIC_PROGRAM_ID || "YOUR_PROGRAM_ID";

// Devnet USDC mint (use wrapped SOL or test USDC)
export const USDC_MINT = process.env.NEXT_PUBLIC_USDC_MINT || "DEVNET_USDC_MINT";

export const MIN_LIQUIDITY = 100_000_000n; // 100 USDC (6 decimals)
export const DEFAULT_SLIPPAGE = 5; // 5%
export const FEE_BPS = 30; // 0.3%
```

### 5. Create Environment Files
```bash
# .env.production
NEXT_PUBLIC_NETWORK=devnet
NEXT_PUBLIC_RPC_URL=https://api.devnet.solana.com
NEXT_PUBLIC_PROGRAM_ID=YOUR_DEPLOYED_PROGRAM_ID
NEXT_PUBLIC_USDC_MINT=DEVNET_USDC_MINT_ADDRESS
```

```bash
# .env.local (for local development against devnet)
NEXT_PUBLIC_NETWORK=devnet
NEXT_PUBLIC_RPC_URL=https://api.devnet.solana.com
NEXT_PUBLIC_PROGRAM_ID=YOUR_DEPLOYED_PROGRAM_ID
NEXT_PUBLIC_USDC_MINT=DEVNET_USDC_MINT_ADDRESS
```

### 6. Regenerate Client with Deployed Program ID
```bash
# Update IDL with program ID
npm run anchor-build

# Regenerate TypeScript client
npm run codama:js
```

### 7. Build and Test Frontend
```bash
# Build frontend
npm run build

# Run in production mode locally
npm run start

# Test all flows manually
```

### 8. Create Deployment Documentation
```markdown
# docs/deployment.md
# Chausar Deployment Guide

## Prerequisites
- Solana CLI installed
- Anchor CLI installed
- Node.js 18+
- Wallet with SOL for deployment

## Devnet Deployment

### 1. Build the Program
\`\`\`bash
cd anchor
anchor build
\`\`\`

### 2. Deploy to Devnet
\`\`\`bash
# Ensure you have devnet SOL
solana airdrop 5 --url devnet

# Deploy
anchor deploy --provider.cluster devnet
\`\`\`

### 3. Update Configuration
1. Copy the program ID from deployment output
2. Update `.env.production` with the program ID
3. Update `anchor/Anchor.toml` with the program ID
4. Regenerate TypeScript client: `npm run setup`

### 4. Deploy Frontend
\`\`\`bash
# Build
npm run build

# Deploy to Vercel/Netlify
vercel --prod
# or
netlify deploy --prod
\`\`\`

## Testing on Devnet

### Get Test Tokens
1. Get devnet SOL: `solana airdrop 2 --url devnet`
2. Get devnet USDC: Use a faucet or create test tokens

### Test Flows
1. Connect wallet (use Phantom in devnet mode)
2. Create a market with 100 USDC
3. Trade YES/NO tokens
4. Add/remove liquidity
5. (For testing resolution: use oracle wallet)

## Mainnet Deployment (Future)

### Security Checklist
- [ ] Program audited
- [ ] Frontend security review
- [ ] Rate limiting configured
- [ ] Monitoring set up
- [ ] Incident response plan

### Deployment Steps
1. Update cluster to mainnet in Anchor.toml
2. Deploy with mainnet wallet
3. Update frontend configuration
4. Gradual rollout with limits
```

## Completion Criteria
- [ ] Program deployed to devnet successfully
- [ ] Program ID configured in frontend
- [ ] Frontend builds without errors
- [ ] Full flow works on devnet:
  - [ ] Create market
  - [ ] Trade YES/NO
  - [ ] Add liquidity
  - [ ] Remove liquidity
  - [ ] Resolve market (as oracle)
  - [ ] Claim winnings
- [ ] Deployment documentation complete
- [ ] Operation verified: L1 (functional on devnet)

## Notes
- Impact scope: Production deployment
- Constraints: Requires devnet SOL and test USDC
- Keep deployment keys secure
- Do not commit .env files with secrets
