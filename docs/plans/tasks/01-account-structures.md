# Task: Account Structures and Enums

Metadata:
- Dependencies: None (foundation task)
- Provides: anchor/programs/prediction/src/state.rs
- Size: Small (1 file)

## Implementation Content
Create the core account structures and enums for the prediction market:
- Market account struct with all fields from PRD
- Pool account struct for YES/NO pools
- MarketStatus enum (Open, Locked, Resolved)
- MarketResult enum (Undecided, Yes, No)
- PoolSide enum (Yes, No)

## Target Files
- [x] `anchor/programs/prediction/src/state.rs`

## Implementation Steps (TDD: Red-Green-Refactor)

### 1. Red Phase
- [x] Create new `anchor/programs/prediction/` directory structure
- [x] Create `src/lib.rs` with module declarations
- [x] Create `src/state.rs` with struct/enum definitions
- [x] Verify structs compile with correct Anchor attributes

### 2. Green Phase
- [x] Add `#[account]` attribute to Market and Pool structs
- [x] Add space calculations for account sizing
- [x] Ensure all fields match PRD specification:

```rust
// Market fields:
// - id: u64
// - question: String (max 280 chars)
// - description: String (max 1000 chars)
// - creator: Pubkey
// - oracle: Pubkey
// - end_time: i64
// - resolve_time: i64
// - yes_mint: Pubkey
// - no_mint: Pubkey
// - yes_pool: Pubkey
// - no_pool: Pubkey
// - vault_usdc: Pubkey
// - status: MarketStatus
// - result: MarketResult
// - total_liquidity: u64
// - created_at: i64
// - bump: u8

// Pool fields:
// - market: Pubkey
// - side: PoolSide
// - usdc_reserve: u64
// - token_reserve: u64
// - lp_mint: Pubkey
// - total_lp_supply: u64
// - fee_bps: u16
// - collected_fees: u64
// - bump: u8
```

### 3. Refactor Phase
- [x] Ensure proper documentation comments
- [x] Verify space calculations are accurate
- [x] Run `anchor build` to confirm compilation

## Completion Criteria
- [x] All structs and enums compile without errors
- [x] Operation verified: L3 (build succeeds)
- [x] Space calculations documented for each account type

## Notes
- Impact scope: Foundation for all subsequent smart contract tasks
- Constraints: Must match PRD data model exactly
- String lengths: question 280 chars, description 1000 chars
