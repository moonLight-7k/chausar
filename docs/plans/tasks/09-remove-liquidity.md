# Task: remove_liquidity Instruction

Metadata:
- Dependencies: task-08 (add_liquidity)
- Provides: anchor/programs/prediction/src/instructions/remove_liquidity.rs
- Size: Small (1-2 files)

## Implementation Content
Implement the `remove_liquidity` instruction that:
- Validates LP token balance
- Calculates proportional USDC + position tokens to return
- Burns LP tokens
- Transfers assets to user

## Target Files
- [ ] `anchor/programs/prediction/src/instructions/remove_liquidity.rs`
- [ ] Update `anchor/programs/prediction/src/instructions/mod.rs`

## Implementation Steps (TDD: Red-Green-Refactor)

### 1. Red Phase
- [ ] Write test: add liquidity then remove
- [ ] Write test: partial removal
- [ ] Verify tests fail

### 2. Green Phase
- [ ] Implement withdrawal calculation:

```rust
// USDC to return = (lp_amount / total_lp_supply) * pool_usdc_reserve
// Tokens to return = (lp_amount / total_lp_supply) * pool_token_reserve
pub fn calculate_withdrawal(
    lp_amount: u64,
    pool_usdc_reserve: u64,
    pool_token_reserve: u64,
    total_lp_supply: u64,
) -> Result<(u64, u64)> {
    let usdc_out = (lp_amount as u128)
        .checked_mul(pool_usdc_reserve as u128)
        .ok_or(PredictionError::MathOverflow)?
        .checked_div(total_lp_supply as u128)
        .ok_or(PredictionError::MathOverflow)?;

    let tokens_out = (lp_amount as u128)
        .checked_mul(pool_token_reserve as u128)
        .ok_or(PredictionError::MathOverflow)?
        .checked_div(total_lp_supply as u128)
        .ok_or(PredictionError::MathOverflow)?;

    Ok((usdc_out as u64, tokens_out as u64))
}
```

- [ ] Implement RemoveLiquidity accounts struct:

```rust
#[derive(Accounts)]
pub struct RemoveLiquidity<'info> {
    #[account(mut)]
    pub provider: Signer<'info>,

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
    pub provider_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub token_mint: Account<'info, Mint>,

    #[account(mut)]
    pub lp_mint: Account<'info, Mint>,

    #[account(mut)]
    pub provider_lp_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
```

- [ ] Implement remove_liquidity function:
  - Validate LP balance >= lp_amount
  - Calculate USDC and tokens to return
  - Burn LP tokens from provider
  - Transfer USDC from vault to provider
  - Mint position tokens to provider (or transfer from pool)
  - Update pool reserves

### 3. Refactor Phase
- [ ] Handle edge case: removing all liquidity
- [ ] Ensure rounding favors the pool
- [ ] Run tests and confirm they pass

## Completion Criteria
- [ ] remove_liquidity instruction compiles
- [ ] Test passes: correct amounts returned
- [ ] Test passes: LP tokens burned
- [ ] Operation verified: L2 (tests pass)

## Notes
- Impact scope: Enables LP withdrawal
- Constraints: Cannot remove more than balance
- Returns both USDC and position tokens proportionally
