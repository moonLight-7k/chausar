# Task: resolve_market Instruction

Metadata:
- Dependencies: task-05 (lock_market)
- Provides: anchor/programs/prediction/src/instructions/resolve_market.rs
- Size: Small (1-2 files)

## Implementation Content
Implement the `resolve_market` instruction that:
- Validates caller is the designated oracle
- Validates market status is Locked
- Sets the market result (Yes or No)
- Changes status to Resolved

## Target Files
- [ ] `anchor/programs/prediction/src/instructions/resolve_market.rs`
- [ ] Update `anchor/programs/prediction/src/instructions/mod.rs`

## Implementation Steps (TDD: Red-Green-Refactor)

### 1. Red Phase
- [ ] Write test: create market, lock market, resolve as oracle
- [ ] Write test: attempt resolve as non-oracle (should fail)
- [ ] Verify tests fail (instruction not implemented)

### 2. Green Phase
- [ ] Implement ResolveMarket accounts struct:

```rust
#[derive(Accounts)]
pub struct ResolveMarket<'info> {
    #[account(
        constraint = oracle.key() == market.oracle @ PredictionError::UnauthorizedOracle
    )]
    pub oracle: Signer<'info>,

    #[account(
        mut,
        constraint = market.status == MarketStatus::Locked @ PredictionError::MarketNotLocked,
        constraint = market.result == MarketResult::Undecided @ PredictionError::MarketAlreadyResolved,
    )]
    pub market: Account<'info, Market>,
}
```

- [ ] Implement resolve_market function:
  - Validate oracle is signer
  - Validate market is Locked
  - Set market.result to provided result (Yes or No)
  - Set market.status = MarketStatus::Resolved

### 3. Refactor Phase
- [ ] Add event emission for resolution
- [ ] Run tests and confirm they pass

## Completion Criteria
- [ ] resolve_market instruction compiles
- [ ] Test passes: oracle can resolve market
- [ ] Test passes: non-oracle cannot resolve
- [ ] Operation verified: L2 (tests pass)

## Notes
- Impact scope: Enables claim_winnings instruction
- Constraints: Only designated oracle can resolve
- Cannot resolve to Undecided (must be Yes or No)
