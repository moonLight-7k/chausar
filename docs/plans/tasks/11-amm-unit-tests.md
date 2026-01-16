# Task: AMM Calculation Unit Tests

Metadata:
- Dependencies: task-07 (trade instruction with AMM utils)
- Provides: Comprehensive AMM test coverage
- Size: Small (1-2 files)

## Implementation Content
Write comprehensive unit tests for AMM calculations:
- Constant product formula validation
- Fee deduction accuracy
- Edge cases (small amounts, large amounts)
- Price calculation accuracy

## Target Files
- [ ] `anchor/programs/prediction/src/utils/amm.rs` (add tests module)
- [ ] Or separate test file if preferred

## Implementation Steps (TDD: Red-Green-Refactor)

### 1. Red Phase
- [ ] Write test cases for calculate_output function
- [ ] Write test cases for price calculation
- [ ] Identify edge cases

### 2. Green Phase
- [ ] Implement comprehensive tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_output_basic() {
        // 1000 USDC in, 1000 USDC reserve, 1000 token reserve
        // dy = (1000 * 1000 * 997) / (1000 * 1000 + 1000 * 997)
        // dy = 997000000 / 1997000 = 499.24...
        let output = calculate_output(1000, 1000, 1000, 30).unwrap();
        assert!(output > 490 && output < 510);
    }

    #[test]
    fn test_calculate_output_fee_deduction() {
        // Verify 0.3% fee is deducted
        let with_fee = calculate_output(10000, 100000, 100000, 30).unwrap();
        let without_fee = calculate_output(10000, 100000, 100000, 0).unwrap();
        assert!(with_fee < without_fee);
    }

    #[test]
    fn test_calculate_output_small_amount() {
        // Small input amounts
        let output = calculate_output(1, 1000000, 1000000, 30).unwrap();
        assert!(output > 0);
    }

    #[test]
    fn test_calculate_output_large_amount() {
        // Large input amounts (test overflow protection)
        let output = calculate_output(1_000_000_000, 10_000_000_000, 10_000_000_000, 30);
        assert!(output.is_ok());
    }

    #[test]
    fn test_calculate_output_price_impact() {
        // Larger trades should have higher price impact
        let small_trade = calculate_output(100, 10000, 10000, 30).unwrap();
        let large_trade = calculate_output(1000, 10000, 10000, 30).unwrap();

        // Price per unit should be worse for larger trade
        let small_rate = small_trade as f64 / 100.0;
        let large_rate = large_trade as f64 / 1000.0;
        assert!(small_rate > large_rate);
    }

    #[test]
    fn test_price_calculation() {
        // Price of YES = USDC_yes / (USDC_yes + USDC_no)
        let yes_usdc = 6000u64;
        let no_usdc = 4000u64;
        // YES price should be 0.6, NO price should be 0.4
        let yes_price = (yes_usdc as f64) / ((yes_usdc + no_usdc) as f64);
        assert!((yes_price - 0.6).abs() < 0.001);
    }
}
```

### 3. Refactor Phase
- [ ] Add more edge case tests
- [ ] Add documentation for expected behavior
- [ ] Run `cargo test` and ensure all pass

## Completion Criteria
- [ ] All AMM calculation tests pass
- [ ] Edge cases covered (small, large, zero)
- [ ] Overflow protection verified
- [ ] Operation verified: L2 (tests pass)

## Notes
- Impact scope: Validates trading accuracy
- Formula: dy = (y * dx * (10000 - fee)) / (x * 10000 + dx * (10000 - fee))
- Use u128 for intermediate calculations
