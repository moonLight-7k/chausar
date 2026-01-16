# Task: lock_market Instruction

Metadata:
- Dependencies: task-04 (create_market)
- Provides: anchor/programs/prediction/src/instructions/lock_market.rs
- Size: Small (1-2 files)

## Implementation Content
Implement the `lock_market` instruction that:
- Checks if current_time >= end_time
- Changes market status from Open to Locked
- Disables further trading

## Target Files
- [ ] `anchor/programs/prediction/src/instructions/lock_market.rs`
- [ ] Update `anchor/programs/prediction/src/instructions/mod.rs`

## Implementation Steps (TDD: Red-Green-Refactor)

### 1. Red Phase
- [ ] Write test: create market, advance clock, call lock_market
- [ ] Verify test fails (instruction not implemented)

### 2. Green Phase
- [ ] Implement LockMarket accounts struct:

```rust
#[derive(Accounts)]
pub struct LockMarket<'info> {
    #[account(
        mut,
        constraint = market.status == MarketStatus::Open @ PredictionError::MarketNotOpen,
    )]
    pub market: Account<'info, Market>,
}
```

- [ ] Implement lock_market function:
  - Get current timestamp from Clock sysvar
  - Validate current_time >= market.end_time
  - Set market.status = MarketStatus::Locked
  - Emit event (optional for MVP)

### 3. Refactor Phase
- [ ] Ensure idempotency (calling on locked market is no-op or error)
- [ ] Add clock sysvar to context if needed
- [ ] Run test and confirm it passes

## Completion Criteria
- [ ] lock_market instruction compiles
- [ ] Test passes: market status changes to Locked
- [ ] Operation verified: L2 (test passes)
- [ ] Test: cannot lock before end_time

## Notes
- Impact scope: Affects trading (trade instruction checks status)
- Constraints: Anyone can call this once end_time is reached
- This is a permissionless operation
