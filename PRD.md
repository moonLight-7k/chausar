# Product Requirements Document (PRD)
## Solana Decentralized Prediction Market - MVP

---

## 1. Executive Summary

Build a **trustless, permissionless prediction market** on Solana where users can create binary (Yes/No) markets, trade positions, and receive automatic payouts based on oracle-resolved outcomes.

**Core Value Proposition:**
- No custody - users control funds via wallet
- No KYC/geographical restrictions
- Automatic settlement via smart contracts
- Transparent, on-chain liquidity pools

---

## 2. Product Goals

### Primary Goals
1. Enable anyone to create a binary prediction market
2. Provide instant trading via AMM (no orderbook matching delays)
3. Ensure trustless, automatic payouts upon resolution`
4. Maintain low transaction costs (<$0.01 per trade)

### Non-Goals (Out of Scope for MVP)
- Governance tokens or DAO
- Multi-outcome markets (only Yes/No)
- Dispute resolution mechanism
- Mobile native apps
- Limit orders
- Social features (comments, profiles)
- Advanced market types (scalar, categorical)

---

## 3. User Personas

### Persona 1: Market Creator
- Wants to create prediction markets on topics they care about
- Willing to provide initial liquidity
- Expects fair oracle resolution

### Persona 2: Trader
- Wants to bet on outcomes
- Expects clear pricing and instant execution
- Wants to withdraw winnings without friction

### Persona 3: Liquidity Provider
- Wants to earn fees from trading activity
- Willing to take on market-making risk
- Expects transparent LP token accounting

---

## 4. System Architecture

### 4.1 Technology Stack

**Blockchain:**
- Solana mainnet
- Anchor framework for smart contracts

**Smart Contracts:**
- Rust/Anchor program for core logic
- SPL Token program for YES/NO tokens

**Frontend:**
- React + TypeScript
- @solana/web3.js
- @project-serum/anchor
- Wallet adapter (Phantom, Solflare)

**Backend (Optional for MVP):**
- Simple indexer for market discovery
- Could be replaced by on-chain queries initially

### 4.2 On-Chain Architecture

```
┌─────────────────────────────────────────┐
│         Prediction Protocol             │
│          (Anchor Program)               │
└─────────────────────────────────────────┘
              │
              ├── Market Accounts
              ├── Pool Accounts (YES/NO)
              ├── Vault Accounts (USDC)
              └── Oracle Account
```

---

## 5. Core Data Models

### 5.1 Market Account

```rust
pub struct Market {
    pub id: u64,                    // Unique market ID
    pub question: String,           // Max 280 chars
    pub description: String,        // Max 1000 chars
    pub creator: Pubkey,            // Market creator
    pub oracle: Pubkey,             // Resolution authority
    
    pub end_time: i64,              // Unix timestamp - trading ends
    pub resolve_time: i64,          // Expected resolution time
    
    pub yes_mint: Pubkey,           // YES token mint
    pub no_mint: Pubkey,            // NO token mint
    
    pub yes_pool: Pubkey,           // YES AMM pool
    pub no_pool: Pubkey,            // NO AMM pool
    
    pub vault_usdc: Pubkey,         // USDC vault
    
    pub status: MarketStatus,       // Open | Locked | Resolved
    pub result: MarketResult,       // Undecided | Yes | No
    
    pub total_liquidity: u64,       // Total USDC locked
    pub created_at: i64,            // Creation timestamp
}

pub enum MarketStatus {
    Open,       // Trading active
    Locked,     // Trading ended, awaiting resolution
    Resolved,   // Outcome determined
}

pub enum MarketResult {
    Undecided,
    Yes,
    No,
}
```

### 5.2 Pool Account (YES and NO)

```rust
pub struct Pool {
    pub market: Pubkey,             // Parent market
    pub side: PoolSide,             // Yes or No
    
    pub usdc_reserve: u64,          // USDC in pool
    pub token_reserve: u64,         // YES/NO tokens in pool
    
    pub lp_mint: Pubkey,            // LP token mint
    pub total_lp_supply: u64,       // Total LP tokens
    
    pub fee_bps: u16,               // Fee in basis points (e.g., 30 = 0.3%)
    pub collected_fees: u64,        // Accumulated fees
}

pub enum PoolSide {
    Yes,
    No,
}
```

---

## 6. Core User Flows

### 6.1 Create Market Flow

**Actor:** Market Creator

**Preconditions:**
- User has connected Solana wallet
- User has sufficient USDC for initial liquidity (minimum 100 USDC)

**Steps:**
1. User fills market creation form:
   - Question (required)
   - Description (optional)
   - End time (must be future)
   - Resolve time (must be after end time)
   - Initial liquidity amount
   - Oracle address (or use default multisig)

2. User approves transaction
3. Program executes:
   - Create Market account
   - Create YES token mint
   - Create NO token mint
   - Create YES pool
   - Create NO pool
   - Deposit USDC to vault
   - Mint initial YES/NO tokens to pools
   - Mint LP tokens to creator

4. Market goes live immediately

**Success Criteria:**
- Market is visible in market list
- Initial price for both YES/NO is ~50%
- Creator receives LP tokens

**Error Cases:**
- Insufficient USDC → Show error, don't execute
- Invalid timestamps → Reject transaction
- Duplicate market → Allow (no uniqueness constraint in MVP)

---

### 6.2 Trade (Buy YES/NO) Flow

**Actor:** Trader

**Preconditions:**
- Market status = Open
- Current time < end_time
- User has USDC

**Steps:**
1. User selects market
2. User chooses YES or NO
3. User enters USDC amount to spend
4. UI shows:
   - Estimated tokens received
   - Current price
   - Price impact
   - Estimated probability after trade

5. User confirms transaction
6. Program executes:
   - Transfer USDC from user to pool
   - Calculate tokens out using constant product formula
   - Transfer YES/NO tokens to user
   - Update pool reserves

**Success Criteria:**
- User receives expected tokens (±0.1% slippage tolerance)
- Pool reserves updated correctly
- Transaction cost < $0.01

**Error Cases:**
- Slippage too high → Show warning, allow override
- Insufficient USDC → Reject with clear message
- Market locked → Disable trade button

---

### 6.3 Provide Liquidity Flow

**Actor:** Liquidity Provider

**Preconditions:**
- Market status = Open
- User has USDC

**Steps:**
1. User selects "Add Liquidity"
2. User chooses pool (YES or NO)
3. User enters USDC amount
4. UI calculates:
   - YES/NO tokens needed for balanced deposit
   - LP tokens to receive
   - Current pool share

5. User confirms
6. Program executes:
   - Transfer USDC from user
   - Mint YES/NO tokens to user first
   - User deposits both to pool
   - Mint LP tokens to user

**Success Criteria:**
- User receives LP tokens
- Pool reserves increase proportionally

---

### 6.4 Resolution Flow

**Actor:** Oracle (Multisig)

**Preconditions:**
- Market status = Locked
- Current time >= resolve_time

**Steps:**
1. Oracle determines real-world outcome
2. Oracle calls `resolve_market` instruction
3. Passes market ID and result (Yes/No)
4. Program executes:
   - Verify caller is oracle
   - Set market.result
   - Set market.status = Resolved
   - Emit resolution event

**Success Criteria:**
- Market result is set
- Payouts become available

---

### 6.5 Claim Payout Flow

**Actor:** Winning Trader

**Preconditions:**
- Market status = Resolved
- User holds winning tokens (YES if result=Yes, NO if result=No)

**Steps:**
1. User clicks "Claim Winnings"
2. UI shows claimable amount
3. User confirms
4. Program executes:
   - Burn user's winning tokens
   - Transfer equivalent USDC to user (1 token = 1 USDC)

**Success Criteria:**
- User receives USDC
- Tokens are burned

---

## 7. Technical Specifications

### 7.1 AMM Pricing Model

Use **constant product formula** for each pool independently:

```
x * y = k

where:
x = USDC reserve
y = Token reserve (YES or NO)
k = constant
```

**Price calculation:**
```
Price of YES = USDC_yes / (USDC_yes + USDC_no)
Price of NO = USDC_no / (USDC_yes + USDC_no)

Always: Price_yes + Price_no ≈ 1.0
```

**Token output calculation:**
```
Given USDC input (dx):
dy = (y * dx) / (x + dx)

where:
dy = tokens out
y = current token reserve
x = current USDC reserve
dx = USDC input
```

**Fee structure:**
- Trading fee: 0.3% (30 bps)
- Fees accrue to liquidity providers
- Deducted from input amount before swap calculation

### 7.2 Smart Contract Instructions

**create_market**
```rust
pub fn create_market(
    ctx: Context<CreateMarket>,
    question: String,
    description: String,
    end_time: i64,
    resolve_time: i64,
    initial_liquidity: u64,
) -> Result<()>
```

**trade**
```rust
pub fn trade(
    ctx: Context<Trade>,
    side: PoolSide,        // Yes or No
    amount_in: u64,         // USDC to spend
    min_amount_out: u64,    // Slippage protection
) -> Result<()>
```

**add_liquidity**
```rust
pub fn add_liquidity(
    ctx: Context<AddLiquidity>,
    side: PoolSide,
    usdc_amount: u64,
) -> Result<()>
```

**remove_liquidity**
```rust
pub fn remove_liquidity(
    ctx: Context<RemoveLiquidity>,
    side: PoolSide,
    lp_amount: u64,
) -> Result<()>
```

**lock_market**
```rust
pub fn lock_market(
    ctx: Context<LockMarket>,
) -> Result<()>
// Anyone can call when end_time reached
```

**resolve_market**
```rust
pub fn resolve_market(
    ctx: Context<ResolveMarket>,
    result: MarketResult,
) -> Result<()>
// Only oracle can call
```

**claim_winnings**
```rust
pub fn claim_winnings(
    ctx: Context<ClaimWinnings>,
) -> Result<()>
// Burn winning tokens, receive USDC
```

### 7.3 Security Requirements

**Access Controls:**
- Only oracle can resolve markets
- Anyone can lock market after end_time
- Market creator has no special privileges post-creation

**Validation:**
- end_time must be in future
- resolve_time must be after end_time
- Slippage checks on all trades
- Prevent trades when market locked
- Prevent double-resolution

**Economic Security:**
- Minimum liquidity: 100 USDC
- Maximum slippage: 5% default (user can override)
- No market creator fee (for MVP)

---

## 8. Frontend Requirements

### 8.1 Market List Page

**Components:**
- Search/filter markets
- Sort by: ending soon, highest volume, newest
- Market cards showing:
  - Question
  - Current YES/NO prices
  - Time remaining
  - Total liquidity
  - Status badge

### 8.2 Market Detail Page

**Components:**
- Question and description
- Price chart (YES/NO over time)
- Trading interface:
  - YES/NO tabs
  - Amount input
  - Expected output display
  - Buy button
- Pool stats:
  - YES pool reserves
  - NO pool reserves
  - Total volume
  - Your position
- Activity feed (recent trades)

### 8.3 Portfolio Page

**Components:**
- Active positions (markets you've traded)
- LP positions
- Claimable winnings
- Trade history

### 8.4 Create Market Page

**Form fields:**
- Question (textarea)
- Description (textarea)
- End date/time (datetime picker)
- Resolution date/time (datetime picker)
- Initial liquidity (number input)
- Oracle selection (dropdown, default to protocol multisig)

---

## 9. Testing Requirements

### 9.1 Smart Contract Tests

**Unit Tests:**
- Market creation
- Pool initialization
- AMM calculations
- Token minting/burning
- Access control

**Integration Tests:**
- Full trade flow
- Liquidity provision and removal
- Resolution and payout
- Edge cases (zero liquidity, dust amounts)

**Scenario Tests:**
- Multiple traders in same market
- LP providing then removing liquidity
- Market that resolves YES vs NO
- Expired market cannot be traded

### 9.2 Frontend Tests

**Component Tests:**
- Form validation
- Wallet connection
- Transaction signing
- Error handling

**E2E Tests:**
- Create market → Trade → Resolve → Claim
- Add liquidity → Remove liquidity

---

## 10. Launch Criteria

### Must Have (Blockers)
- [ ] Smart contracts deployed to mainnet
- [ ] Contracts audited (at minimum: internal review)
- [ ] Frontend can create markets
- [ ] Frontend can trade YES/NO
- [ ] Resolution mechanism works
- [ ] Payouts work
- [ ] Basic error handling

### Should Have (High Priority)
- [ ] Market search/filter
- [ ] Price charts
- [ ] Transaction history
- [ ] Wallet connection to Phantom + Solflare
- [ ] Slippage protection

### Nice to Have (Post-MVP)
- [ ] Market categories/tags
- [ ] Share market links
- [ ] Email notifications for resolution
- [ ] Advanced charts

---

## 11. Risks & Mitigations

| Risk                        | Impact   | Likelihood | Mitigation                                         |
| --------------------------- | -------- | ---------- | -------------------------------------------------- |
| Oracle manipulation         | High     | Medium     | Use 3-of-5 multisig, plan dispute mechanism for v2 |
| Low liquidity kills markets | High     | High       | Set minimum liquidity requirement, incentivize LPs |
| Smart contract bug          | Critical | Low        | Audit, testnet testing, start with low limits      |
| Poor UX → no adoption       | High     | Medium     | User testing, clear error messages, tutorials      |
| Solana network congestion   | Medium   | Low        | Implement retry logic, show clear pending states   |

---

## 12. Success Metrics

**MVP Success (30 days post-launch):**
- 50+ markets created
- 500+ trades executed
- $10,000+ total value locked
- <1% transaction failure rate
- 10+ active liquidity providers

**Growth Metrics (3 months):**
- 500+ markets
- 5,000+ unique wallets
- $100,000+ TVL
- 50+ daily active users

---

## 13. Development Phases

### Phase 1: Smart Contracts (Weeks 1-3)
- Set up Anchor project
- Implement core instructions
- Write tests
- Deploy to devnet

### Phase 2: Frontend Shell (Weeks 2-4)
- Set up React app
- Wallet integration
- Basic UI components
- Connect to devnet

### Phase 3: Integration (Weeks 4-5)
- Wire frontend to contracts
- End-to-end testing
- Bug fixes

### Phase 4: Polish & Launch (Week 6)
- UI/UX improvements
- Documentation
- Deploy to mainnet
- Announce

---

## 14. Open Questions

1. **Oracle design:** Should we use Chainlink, Pyth, or simple multisig for MVP?
   - **Decision:** Multisig for MVP, integrate Chainlink in v2

2. **Market creation fee:** Should creators pay a fee or provide minimum liquidity?
   - **Decision:** Minimum liquidity only (100 USDC)

3. **Invalid markets:** What if question is ambiguous?
   - **Decision:** Oracle can resolve as "Invalid" and return funds proportionally (post-MVP)

4. **Time zones:** UTC for all timestamps?
   - **Decision:** Yes, display in user's local time in UI

---

## 15. Appendix

### A. Example Markets

1. "Will Bitcoin close above $100,000 on January 31, 2026?"
2. "Will it rain in San Francisco on February 14, 2026?"
3. "Will the S&P 500 be above 6,000 on March 1, 2026?"

### B. Glossary

- **AMM:** Automated Market Maker - algorithm for token swaps
- **LP:** Liquidity Provider - user who deposits to pools
- **Oracle:** Trusted entity that resolves market outcomes
- **SPL:** Solana Program Library - token standard
- **TVL:** Total Value Locked - sum of all USDC in protocol

### C. References

- Solana documentation: https://docs.solana.com
- Anchor framework: https://www.anchor-lang.com
- Kalshi (inspiration): https://kalshi.com
- Polymarket (comparison): https://polymarket.com

---

**Document Version:** 1.0  
**Last Updated:** January 16, 2026  
**Owner:** Development Team  
**Status:** Ready for Implementation