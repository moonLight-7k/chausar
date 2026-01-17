# Chausar Implementation Todo

## Progress Summary
| Phase | Status | Tasks |
|-------|--------|-------|
| Phase 1: Smart Contract Foundation | 🟡 In Progress | 2/3 complete |
| Phase 2: Smart Contract Instructions | ⬜ Pending | 0/7 |
| Phase 3: Smart Contract Testing | ⬜ Pending | 0/6 |
| Phase 4: Client Generation | ⬜ Pending | 0/3 |
| Phase 5: Frontend Infrastructure | ⬜ Pending | 0/8 |
| Phase 6: Frontend Pages | ⬜ Pending | 0/4 |
| Phase 7: Frontend Components | ⬜ Pending | 0/10 |
| Phase 8: Wallet Integration | ⬜ Pending | 0/4 |
| Phase 9: Quality Assurance | ⬜ Pending | 0/6 |
| Phase 10: Deployment | ⬜ Pending | 0/4 |

---

## Phase 1: Smart Contract Foundation ✅ IN PROGRESS

### Core Account Structures ✅ COMPLETE
- [x] Create Market account struct (id, question, description, creator, oracle, timestamps, mints, pools, vault, status, result)
- [x] Create Pool account struct (market, side, reserves, lp_mint, fees)
- [x] Create MarketStatus enum (Open, Locked, Resolved)
- [x] Create MarketResult enum (Undecided, Yes, No)
- [x] Create PoolSide enum (Yes, No)
- [ ] Define custom error types

### PDA Derivation ✅ COMPLETE
- [x] Market PDA (seeds: "market", market_id)
- [x] Pool PDAs (seeds: "pool", market, side)
- [x] Vault PDA (seeds: "vault", market)
- [x] YES/NO mint PDAs
- [x] LP mint PDAs

## Phase 2: Smart Contract Instructions

### Market Lifecycle
- [ ] `create_market` - Create market with initial liquidity
  - Validate question (max 280 chars), description (max 1000 chars)
  - Validate end_time > now, resolve_time > end_time
  - Require minimum 100 USDC initial liquidity
  - Initialize YES/NO token mints
  - Initialize YES/NO pools with liquidity
  - Create USDC vault

- [ ] `lock_market` - Lock market when end_time reached
  - Validate current_time >= end_time
  - Change status to Locked
  - Disable trading

- [ ] `resolve_market` - Oracle resolves outcome
  - Validate caller is oracle
  - Validate status is Locked
  - Set result (Yes/No)
  - Change status to Resolved

### Trading
- [ ] `trade` - Buy YES or NO tokens via AMM
  - Validate market is Open
  - Calculate output using constant product formula: dy = (y * dx) / (x + dx)
  - Deduct 0.3% fee before swap
  - Validate slippage (min_amount_out)
  - Transfer USDC to pool, mint tokens to user

### Liquidity
- [ ] `add_liquidity` - Deposit USDC to pool
  - Validate market is Open
  - Calculate LP tokens to mint proportionally
  - Transfer USDC to pool
  - Mint LP tokens to user

- [ ] `remove_liquidity` - Withdraw from pool
  - Validate LP token balance
  - Calculate proportional USDC + position tokens
  - Burn LP tokens
  - Transfer assets to user

### Payouts
- [ ] `claim_winnings` - Claim payout after resolution
  - Validate market is Resolved
  - Validate user holds winning tokens
  - Burn winning tokens
  - Transfer 1 USDC per token to user

## Phase 3: Smart Contract Testing

- [ ] Unit tests for AMM calculations
- [ ] Unit tests for each instruction
- [ ] Integration test: create market -> trade -> resolve -> claim
- [ ] Integration test: add liquidity -> remove liquidity
- [ ] Edge case tests (zero liquidity, dust amounts, max slippage)
- [ ] Access control tests (only oracle can resolve, etc.)

## Phase 4: Client Generation

- [ ] Build Anchor program (`npm run anchor-build`)
- [ ] Generate TypeScript client with Codama (`npm run codama:js`)
- [ ] Verify generated types match expected interfaces

## Phase 5: Frontend - Core Infrastructure

### Providers & Config
- [ ] Update `app/components/providers.tsx` with proper RPC config
- [ ] Add program ID configuration for devnet
- [ ] Set up USDC mint address for devnet

### Hooks
- [ ] `useMarkets` - Fetch all markets
- [ ] `useMarket` - Fetch single market by ID
- [ ] `useTrade` - Execute trade transaction
- [ ] `useLiquidity` - Add/remove liquidity transactions
- [ ] `usePortfolio` - Fetch user positions and LP balances

### Utilities
- [ ] `app/lib/amm.ts` - AMM price calculation helpers
- [ ] `app/lib/formatters.ts` - Price, time, amount formatters

## Phase 6: Frontend - Pages

### Market List Page (`app/markets/page.tsx`)
- [ ] Display all markets in card grid
- [ ] Show question, YES/NO prices, time remaining, liquidity, status
- [ ] Sort by: ending soon, volume, newest
- [ ] Basic search/filter

### Market Detail Page (`app/markets/[id]/page.tsx`)
- [ ] Display full market info (question, description, timestamps)
- [ ] Trading panel (YES/NO buttons, amount input, price impact)
- [ ] Liquidity panel (add/remove)
- [ ] Current prices display
- [ ] Market status indicator

### Create Market Page (`app/create/page.tsx`)
- [ ] Form: question, description, end time, resolve time
- [ ] Initial liquidity input (min 100 USDC)
- [ ] Oracle address input
- [ ] Form validation
- [ ] Submit transaction

### Portfolio Page (`app/portfolio/page.tsx`)
- [ ] Active positions (YES/NO token balances per market)
- [ ] LP positions (LP token balances per pool)
- [ ] Claimable winnings (resolved markets)
- [ ] Claim button for each resolved position

## Phase 7: Frontend - Components

- [ ] `MarketCard` - Market summary card for list view
- [ ] `TradingPanel` - Buy YES/NO interface
- [ ] `LiquidityPanel` - Add/remove liquidity interface
- [ ] `PositionCard` - Display user position
- [ ] `CreateMarketForm` - Market creation form
- [ ] `Navigation` - Header with wallet connect and nav links
- [ ] `MarketStatus` - Status badge (Open/Locked/Resolved)
- [ ] `PriceDisplay` - YES/NO price display
- [ ] `AmountInput` - USDC amount input with validation
- [ ] `SlippageSettings` - Slippage tolerance config (default 5%)

## Phase 8: Wallet Integration

- [ ] Wallet connection (Phantom, Solflare)
- [ ] Persist wallet state across sessions
- [ ] Display connected wallet address
- [ ] Handle disconnect

## Phase 9: Quality Assurance

- [ ] Frontend component tests
- [ ] E2E test: full flow (create -> trade -> resolve -> claim)
- [ ] Error handling and user-friendly error messages
- [ ] Loading states
- [ ] Transaction confirmation feedback
- [ ] Mobile responsive design check

## Phase 10: Deployment

- [ ] Deploy Anchor program to devnet
- [ ] Configure frontend for devnet
- [ ] Test full flow on devnet
- [ ] Documentation updates

---

## Notes

- **Minimum liquidity**: 100 USDC per market
- **Trading fee**: 0.3% (30 bps)
- **Default slippage**: 5%
- **AMM formula**: x * y = k (constant product)
- **Price calculation**: Price_YES = USDC_yes / (USDC_yes + USDC_no)

## Out of Scope (MVP)

- Governance tokens/DAO
- Multi-outcome markets
- Dispute resolution
- Mobile native apps
- Limit orders
- Social features
- Market categories/tags
