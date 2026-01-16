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
- [ ] `anchor/programs/prediction/src/seeds.rs`
- [ ] Update `anchor/programs/prediction/src/lib.rs` to include module

## Implementation Steps (TDD: Red-Green-Refactor)

### 1. Red Phase
- [ ] Create `src/seeds.rs` with seed constants
- [ ] Define PDA derivation patterns

### 2. Green Phase
- [ ] Implement seed constants:

```rust
pub const MARKET_SEED: &[u8] = b"market";
pub const POOL_SEED: &[u8] = b"pool";
pub const VAULT_SEED: &[u8] = b"vault";
pub const YES_MINT_SEED: &[u8] = b"yes_mint";
pub const NO_MINT_SEED: &[u8] = b"no_mint";
pub const LP_MINT_SEED: &[u8] = b"lp_mint";
```

- [ ] Add module to lib.rs

### 3. Refactor Phase
- [ ] Add documentation for each seed constant
- [ ] Verify consistent naming convention
- [ ] Run `anchor build` to confirm compilation

## Completion Criteria
- [ ] All seed constants defined and accessible
- [ ] Operation verified: L3 (build succeeds)
- [ ] Seeds match PRD specification

## Notes
- Impact scope: All instructions will use these seeds
- Constraints: Seeds must be unique and collision-free
- Market PDA uses market_id (u64 as le_bytes)
- Pool PDAs use market pubkey + side enum
