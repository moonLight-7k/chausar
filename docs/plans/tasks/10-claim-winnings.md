# Task: claim_winnings Instruction

Metadata:
- Dependencies: task-06 (resolve_market)
- Provides: anchor/programs/prediction/src/instructions/claim_winnings.rs
- Size: Small (1-2 files)

## Implementation Content
Implement the `claim_winnings` instruction that:
- Validates market is Resolved
- Validates user holds winning tokens
- Burns winning tokens
- Transfers 1 USDC per token to user

## Target Files
- [ ] `anchor/programs/prediction/src/instructions/claim_winnings.rs`
- [ ] Update `anchor/programs/prediction/src/instructions/mod.rs`

## Implementation Steps (TDD: Red-Green-Refactor)

### 1. Red Phase
- [ ] Write test: create, trade, resolve YES, claim with YES tokens
- [ ] Write test: claim with losing tokens fails
- [ ] Verify tests fail

### 2. Green Phase
- [ ] Implement ClaimWinnings accounts struct:

```rust
#[derive(Accounts)]
pub struct ClaimWinnings<'info> {
    #[account(mut)]
    pub claimant: Signer<'info>,

    #[account(
        constraint = market.status == MarketStatus::Resolved @ PredictionError::MarketNotResolved,
    )]
    pub market: Account<'info, Market>,

    #[account(
        mut,
        seeds = [VAULT_SEED, market.key().as_ref()],
        bump,
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub claimant_usdc_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub claimant_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub winning_token_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
}
```

- [ ] Implement claim_winnings function:
  - Validate market.status == Resolved
  - Determine winning side from market.result
  - Validate claimant holds winning tokens
  - Calculate payout: 1 USDC per winning token
  - Burn winning tokens from claimant
  - Transfer USDC from vault to claimant

### 3. Refactor Phase
- [ ] Add validation for correct token mint (YES vs NO)
- [ ] Handle partial claims (claim less than full balance)
- [ ] Run tests and confirm they pass

## Completion Criteria
- [ ] claim_winnings instruction compiles
- [ ] Test passes: correct payout for winning tokens
- [ ] Test passes: losing tokens rejected
- [ ] Operation verified: L2 (tests pass)

## Notes
- Impact scope: Final step in market lifecycle
- Constraints: Market must be Resolved
- 1 winning token = 1 USDC payout
- Losing tokens have no value (cannot be claimed)
