# Task: Edge Case and Access Control Tests

Metadata:
- Dependencies: task-13 (integration tests)
- Provides: Comprehensive edge case coverage
- Size: Small (1 file)

## Implementation Content
Test edge cases and security scenarios:
- Zero liquidity handling
- Dust amount handling
- Maximum slippage scenarios
- Access control for all instructions
- Double-resolution prevention

## Target Files
- [ ] `anchor/programs/prediction/tests/edge_cases.rs`

## Implementation Steps (TDD: Red-Green-Refactor)

### 1. Red Phase
- [ ] Identify all edge cases from PRD
- [ ] Write failing tests for each

### 2. Green Phase
- [ ] Implement edge case tests:

```rust
mod edge_case_tests {
    #[test]
    fn test_zero_liquidity_trade_fails() {
        // Create market, remove all liquidity, try to trade
    }

    #[test]
    fn test_dust_amount_trade() {
        // Trade with 1 unit (smallest possible)
    }

    #[test]
    fn test_max_slippage_exceeded() {
        // Large trade with tight slippage tolerance
    }

    #[test]
    fn test_trade_entire_pool() {
        // Attempt to buy all tokens in pool
    }

    #[test]
    fn test_minimum_liquidity_enforced() {
        // Try to create market with < 100 USDC
    }
}

mod access_control_tests {
    #[test]
    fn test_only_oracle_can_resolve() {
        // Non-oracle attempts to resolve
    }

    #[test]
    fn test_anyone_can_lock_after_end_time() {
        // Random account locks market
    }

    #[test]
    fn test_double_resolution_fails() {
        // Resolve same market twice
    }

    #[test]
    fn test_trade_after_lock_fails() {
        // Attempt trade on locked market
    }

    #[test]
    fn test_claim_before_resolution_fails() {
        // Attempt claim on open market
    }

    #[test]
    fn test_claim_wrong_tokens_fails() {
        // Claim with losing token type
    }
}

mod math_safety_tests {
    #[test]
    fn test_large_amounts_no_overflow() {
        // Trade with very large amounts
    }

    #[test]
    fn test_small_amounts_no_underflow() {
        // Very small trades don't cause issues
    }

    #[test]
    fn test_fee_calculation_accuracy() {
        // Verify 0.3% fee is exact
    }
}
```

### 3. Refactor Phase
- [ ] Group related tests
- [ ] Add clear failure message expectations
- [ ] Run all tests and ensure pass

## Completion Criteria
- [ ] All edge cases tested
- [ ] Access control verified
- [ ] Math safety verified
- [ ] Operation verified: L2 (tests pass)

## Notes
- Focus on security-critical paths
- Test both boundaries (min and max values)
- Verify error messages match expectations
