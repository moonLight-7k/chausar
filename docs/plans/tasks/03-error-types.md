# Task: Custom Error Types

Metadata:
- Dependencies: task-01 (account structures)
- Provides: anchor/programs/prediction/src/errors.rs
- Size: Small (1 file)

## Implementation Content
Define comprehensive error types for the prediction market program covering:
- Market creation validation errors
- Trading errors (slippage, market status)
- Liquidity errors
- Resolution and claim errors
- Access control errors

## Target Files
- [ ] `anchor/programs/prediction/src/errors.rs`
- [ ] Update `anchor/programs/prediction/src/lib.rs` to include module

## Implementation Steps (TDD: Red-Green-Refactor)

### 1. Red Phase
- [ ] Create `src/errors.rs` with error enum
- [ ] Define error codes with descriptive messages

### 2. Green Phase
- [ ] Implement error enum with Anchor's #[error_code]:

```rust
#[error_code]
pub enum PredictionError {
    // Market Creation Errors
    #[msg("Question exceeds maximum length of 280 characters")]
    QuestionTooLong,
    #[msg("Description exceeds maximum length of 1000 characters")]
    DescriptionTooLong,
    #[msg("End time must be in the future")]
    EndTimeInPast,
    #[msg("Resolve time must be after end time")]
    ResolveTimeBeforeEnd,
    #[msg("Initial liquidity must be at least 100 USDC")]
    InsufficientInitialLiquidity,

    // Trading Errors
    #[msg("Market is not open for trading")]
    MarketNotOpen,
    #[msg("Market has ended")]
    MarketEnded,
    #[msg("Slippage tolerance exceeded")]
    SlippageExceeded,
    #[msg("Insufficient input amount")]
    InsufficientInput,

    // Liquidity Errors
    #[msg("Insufficient LP token balance")]
    InsufficientLpBalance,
    #[msg("Zero liquidity not allowed")]
    ZeroLiquidity,

    // Resolution Errors
    #[msg("Only oracle can resolve market")]
    UnauthorizedOracle,
    #[msg("Market must be locked to resolve")]
    MarketNotLocked,
    #[msg("Market already resolved")]
    MarketAlreadyResolved,
    #[msg("Market cannot be locked yet")]
    MarketNotReadyToLock,

    // Claim Errors
    #[msg("Market not resolved")]
    MarketNotResolved,
    #[msg("No winning tokens to claim")]
    NoWinningTokens,

    // Math Errors
    #[msg("Math overflow")]
    MathOverflow,
}
```

- [ ] Add module to lib.rs

### 3. Refactor Phase
- [ ] Ensure error messages are user-friendly
- [ ] Group related errors with comments
- [ ] Run `anchor build` to confirm compilation

## Completion Criteria
- [ ] All error types compile without errors
- [ ] Operation verified: L3 (build succeeds)
- [ ] Error messages are clear and actionable

## Notes
- Impact scope: All instructions will use these errors
- Constraints: Error messages should be understandable to end users
- Each instruction should have specific error conditions covered
