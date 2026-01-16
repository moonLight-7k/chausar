# Task: Instruction Unit Tests

Metadata:
- Dependencies: Phase 2 completion (all instructions)
- Provides: Individual instruction test coverage
- Size: Medium (1 file with multiple test modules)

## Implementation Content
Write unit tests for each instruction:
- create_market validation tests
- lock_market timing tests
- resolve_market access control tests
- trade slippage tests
- add/remove liquidity proportional tests
- claim_winnings payout tests

## Target Files
- [ ] `anchor/programs/prediction/src/tests.rs` (or tests/ directory)

## Implementation Steps (TDD: Red-Green-Refactor)

### 1. Red Phase
- [ ] Set up test harness with LiteSVM
- [ ] Create test utilities for common setup

### 2. Green Phase
- [ ] Implement test setup:

```rust
#[cfg(test)]
mod tests {
    use litesvm::LiteSVM;
    use anchor_lang::prelude::*;

    fn setup_test() -> (LiteSVM, Keypair) {
        let svm = LiteSVM::new();
        let payer = Keypair::new();
        svm.airdrop(&payer.pubkey(), 10_000_000_000);
        (svm, payer)
    }

    mod create_market_tests {
        #[test]
        fn test_create_market_success() { }

        #[test]
        fn test_create_market_question_too_long() { }

        #[test]
        fn test_create_market_insufficient_liquidity() { }

        #[test]
        fn test_create_market_invalid_timestamps() { }
    }

    mod lock_market_tests {
        #[test]
        fn test_lock_market_after_end_time() { }

        #[test]
        fn test_lock_market_before_end_time_fails() { }
    }

    mod resolve_market_tests {
        #[test]
        fn test_resolve_by_oracle() { }

        #[test]
        fn test_resolve_by_non_oracle_fails() { }

        #[test]
        fn test_resolve_unlocked_market_fails() { }
    }

    mod trade_tests {
        #[test]
        fn test_trade_yes_tokens() { }

        #[test]
        fn test_trade_no_tokens() { }

        #[test]
        fn test_trade_slippage_exceeded() { }

        #[test]
        fn test_trade_locked_market_fails() { }
    }

    mod liquidity_tests {
        #[test]
        fn test_add_liquidity() { }

        #[test]
        fn test_remove_liquidity() { }

        #[test]
        fn test_lp_token_proportional() { }
    }

    mod claim_tests {
        #[test]
        fn test_claim_winning_yes() { }

        #[test]
        fn test_claim_winning_no() { }

        #[test]
        fn test_claim_losing_fails() { }
    }
}
```

### 3. Refactor Phase
- [ ] Extract common setup to helper functions
- [ ] Add assertions for all account state changes
- [ ] Run all tests and ensure pass

## Completion Criteria
- [ ] Each instruction has at least 2 tests (success + failure)
- [ ] All tests pass
- [ ] Test coverage > 80%
- [ ] Operation verified: L2 (tests pass)

## Notes
- Use LiteSVM for fast local testing
- Mock USDC mint for testing
- Test both happy path and error cases
