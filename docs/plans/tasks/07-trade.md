# Task: trade Instruction

Metadata:
- Dependencies: task-04 (create_market)
- Provides: anchor/programs/prediction/src/instructions/trade.rs
- Size: Medium (2-3 files)

## Implementation Content
Implement the `trade` instruction that:
- Validates market is Open
- Calculates output using constant product formula
- Deducts 0.3% fee before swap
- Validates slippage (min_amount_out)
- Transfers USDC to pool, mints tokens to user

## Target Files
- [ ] `anchor/programs/prediction/src/instructions/trade.rs`
- [ ] `anchor/programs/prediction/src/utils/amm.rs` (calculation helpers)
- [ ] Update `anchor/programs/prediction/src/instructions/mod.rs`

## Implementation Steps (TDD: Red-Green-Refactor)

### 1. Red Phase
- [ ] Write AMM calculation unit tests first:
  - Test constant product formula: dy = (y * dx) / (x + dx)
  - Test fee deduction: amount_after_fee = amount * 997 / 1000
- [ ] Write trade integration test
- [ ] Verify tests fail

### 2. Green Phase
- [ ] Implement AMM calculation functions:

```rust
// In utils/amm.rs
pub fn calculate_output(
    input_amount: u64,
    input_reserve: u64,
    output_reserve: u64,
    fee_bps: u16,
) -> Result<u64> {
    // Fee deduction: 30 bps = 0.3%
    // amount_after_fee = input_amount * (10000 - fee_bps) / 10000
    let fee_factor = 10000u128 - fee_bps as u128;
    let amount_with_fee = (input_amount as u128)
        .checked_mul(fee_factor)
        .ok_or(PredictionError::MathOverflow)?;

    // dy = (y * dx) / (x + dx)
    let numerator = (output_reserve as u128)
        .checked_mul(amount_with_fee)
        .ok_or(PredictionError::MathOverflow)?;
    let denominator = (input_reserve as u128)
        .checked_mul(10000)
        .ok_or(PredictionError::MathOverflow)?
        .checked_add(amount_with_fee)
        .ok_or(PredictionError::MathOverflow)?;

    let output = numerator
        .checked_div(denominator)
        .ok_or(PredictionError::MathOverflow)?;

    Ok(output as u64)
}
```

- [ ] Implement Trade accounts struct:

```rust
#[derive(Accounts)]
pub struct Trade<'info> {
    #[account(mut)]
    pub trader: Signer<'info>,

    #[account(
        constraint = market.status == MarketStatus::Open @ PredictionError::MarketNotOpen,
    )]
    pub market: Account<'info, Market>,

    #[account(mut)]
    pub pool: Account<'info, Pool>,

    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub trader_usdc_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub trader_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub token_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
}
```

- [ ] Implement trade function:
  - Validate market.status == Open
  - Validate current_time < market.end_time
  - Calculate output tokens using AMM
  - Validate output >= min_amount_out
  - Transfer USDC from trader to vault
  - Mint YES/NO tokens to trader
  - Update pool reserves
  - Update collected_fees

### 3. Refactor Phase
- [ ] Extract AMM logic to separate utility module
- [ ] Add slippage protection checks
- [ ] Run tests and confirm they pass

## Completion Criteria
- [ ] trade instruction compiles
- [ ] AMM calculation tests pass
- [ ] Trade integration test passes
- [ ] Slippage protection test passes
- [ ] Operation verified: L2 (tests pass)

## Notes
- Impact scope: Core trading functionality
- Constraints: Cannot trade on Locked/Resolved markets
- Fee: 0.3% (30 bps) deducted from input
- Formula: dy = (y * dx * 997) / (x * 1000 + dx * 997)
