# Task: PDA Derivation and Seeds Constants

Metadata:
- Dependencies: task-01 (account structures)
- Provides: anchor/programs/prediction/src/seeds.rs
- Size: Small (1 file)

## Implementation Content
Define PDA seeds constants and helper functions for:
- Market PDA (seeds: "market", market_id)
- Pool PDAs (seeds: "pool", market, side)
- Vault PDA (seeds: "vault", market)
- YES/NO mint PDAs

## Target Files
- [x] `anchor/programs/prediction/src/seeds.rs`
- [x] Update `anchor/programs/prediction/src/lib.rs` to include module

## Implementation Steps (TDD: Red-Green-Refactor)

### 1. Red Phase
- [x] Create `src/seeds.rs` with seed constants
- [x] Define PDA derivation patterns

### 2. Green Phase
- [x] Implement seed constants:

```rust
pub const MARKET_SEED: &[u8] = b"market";
pub const POOL_SEED: &[u8] = b"pool";
pub const VAULT_SEED: &[u8] = b"vault";
pub const YES_MINT_SEED: &[u8] = b"yes_mint";
pub const NO_MINT_SEED: &[u8] = b"no_mint";
pub const LP_MINT_SEED: &[u8] = b"lp_mint";
```

- [x] Add module to lib.rs

### 3. Refactor Phase
- [x] Add documentation for each seed constant
- [x] Verify consistent naming convention
- [x] Run `anchor build` to confirm compilation

## Completion Criteria
- [x] All seed constants defined and accessible
- [x] Operation verified: L3 (build succeeds)
- [x] Seeds match PRD specification

## Notes
- Impact scope: All instructions will use these seeds
- Constraints: Seeds must be unique and collision-free
- Market PDA uses market_id (u64 as le_bytes)
- Pool PDAs use market pubkey + side enum
