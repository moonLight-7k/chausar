# Task: add_liquidity Instruction

Metadata:
- Dependencies: task-04 (create_market), task-07 (trade - for AMM utils)
- Provides: anchor/programs/prediction/src/instructions/add_liquidity.rs
- Size: Small (1-2 files)

## Implementation Content
Implement the `add_liquidity` instruction that:
- Validates market is Open
- Calculates LP tokens to mint proportionally
- Transfers USDC from user to pool
- Mints LP tokens to user

## Target Files
- [ ] `anchor/programs/prediction/src/instructions/add_liquidity.rs`
- [ ] Update `anchor/programs/prediction/src/instructions/mod.rs`

## Implementation Steps (TDD: Red-Green-Refactor)

### 1. Red Phase
- [ ] Write test: add liquidity to existing pool
- [ ] Write test: LP token calculation is proportional
- [ ] Verify tests fail

### 2. Green Phase
- [ ] Implement LP token calculation:

```rust
// LP tokens to mint = (usdc_amount / total_pool_usdc) * total_lp_supply
// For first LP: LP = sqrt(usdc_amount * token_amount)
pub fn calculate_lp_tokens(
    usdc_amount: u64,
    pool_usdc_reserve: u64,
    total_lp_supply: u64,
) -> Result<u64> {
    if total_lp_supply == 0 {
        // First liquidity provider
        Ok(usdc_amount) // 1:1 for simplicity
    } else {
        // Proportional
        let lp_tokens = (usdc_amount as u128)
            .checked_mul(total_lp_supply as u128)
            .ok_or(PredictionError::MathOverflow)?
            .checked_div(pool_usdc_reserve as u128)
            .ok_or(PredictionError::MathOverflow)?;
        Ok(lp_tokens as u64)
    }
}
```

- [ ] Implement AddLiquidity accounts struct:

```rust
#[derive(Accounts)]
pub struct AddLiquidity<'info> {
    #[account(mut)]
    pub provider: Signer<'info>,

    #[account(
        constraint = market.status == MarketStatus::Open @ PredictionError::MarketNotOpen,
    )]
    pub market: Account<'info, Market>,

    #[account(
        mut,
        constraint = pool.market == market.key(),
    )]
    pub pool: Account<'info, Pool>,

    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub provider_usdc_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub lp_mint: Account<'info, Mint>,

    #[account(mut)]
    pub provider_lp_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
```

- [ ] Implement add_liquidity function:
  - Validate market is Open
  - Calculate LP tokens to mint
  - Transfer USDC from provider to vault
  - Mint LP tokens to provider
  - Update pool.usdc_reserve
  - Update pool.total_lp_supply

### 3. Refactor Phase
- [ ] Ensure proper rounding (round down LP tokens)
- [ ] Run tests and confirm they pass

## Completion Criteria
- [ ] add_liquidity instruction compiles
- [ ] Test passes: LP tokens minted correctly
- [ ] Test passes: pool reserves updated
- [ ] Operation verified: L2 (tests pass)

## Notes
- Impact scope: Enables liquidity provision
- Constraints: Market must be Open
- LP calculation: proportional to existing pool share
