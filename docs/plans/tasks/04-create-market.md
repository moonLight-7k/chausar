# Task: create_market Instruction

Metadata:
- Dependencies: task-01, task-02, task-03 (Phase 1 completion)
- Provides: anchor/programs/prediction/src/instructions/create_market.rs
- Size: Medium (3-4 files)

## Implementation Content
Implement the `create_market` instruction that:
- Creates a new prediction market with initial liquidity
- Initializes YES/NO token mints
- Initializes YES/NO pools with liquidity
- Creates USDC vault for the market

## Target Files
- [ ] `anchor/programs/prediction/src/instructions/mod.rs`
- [ ] `anchor/programs/prediction/src/instructions/create_market.rs`
- [ ] Update `anchor/programs/prediction/src/lib.rs`

## Implementation Steps (TDD: Red-Green-Refactor)

### 1. Red Phase
- [ ] Create instructions module structure
- [ ] Define CreateMarket context with all required accounts
- [ ] Write test that calls create_market with valid parameters
- [ ] Verify test fails (instruction not implemented)

### 2. Green Phase
- [ ] Implement CreateMarket accounts struct:

```rust
#[derive(Accounts)]
#[instruction(market_id: u64, question: String, description: String)]
pub struct CreateMarket<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        init,
        payer = creator,
        space = Market::SPACE,
        seeds = [MARKET_SEED, &market_id.to_le_bytes()],
        bump
    )]
    pub market: Account<'info, Market>,

    pub oracle: UncheckedAccount<'info>,

    // YES token mint
    #[account(
        init,
        payer = creator,
        seeds = [YES_MINT_SEED, market.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = market
    )]
    pub yes_mint: Account<'info, Mint>,

    // NO token mint
    #[account(
        init,
        payer = creator,
        seeds = [NO_MINT_SEED, market.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = market
    )]
    pub no_mint: Account<'info, Mint>,

    // YES Pool
    #[account(
        init,
        payer = creator,
        space = Pool::SPACE,
        seeds = [POOL_SEED, market.key().as_ref(), &[0u8]], // 0 = Yes
        bump
    )]
    pub yes_pool: Account<'info, Pool>,

    // NO Pool
    #[account(
        init,
        payer = creator,
        space = Pool::SPACE,
        seeds = [POOL_SEED, market.key().as_ref(), &[1u8]], // 1 = No
        bump
    )]
    pub no_pool: Account<'info, Pool>,

    // USDC vault
    #[account(
        init,
        payer = creator,
        seeds = [VAULT_SEED, market.key().as_ref()],
        bump,
        token::mint = usdc_mint,
        token::authority = market
    )]
    pub vault: Account<'info, TokenAccount>,

    pub usdc_mint: Account<'info, Mint>,

    #[account(mut)]
    pub creator_usdc_account: Account<'info, TokenAccount>,

    // LP mints for each pool
    #[account(
        init,
        payer = creator,
        seeds = [LP_MINT_SEED, yes_pool.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = yes_pool
    )]
    pub yes_lp_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = creator,
        seeds = [LP_MINT_SEED, no_pool.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = no_pool
    )]
    pub no_lp_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
```

- [ ] Implement create_market function:
  - Validate question length <= 280
  - Validate description length <= 1000
  - Validate end_time > now
  - Validate resolve_time > end_time
  - Validate initial_liquidity >= 100_000_000 (100 USDC with 6 decimals)
  - Transfer USDC from creator to vault
  - Initialize Market account fields
  - Initialize Pool accounts
  - Mint initial LP tokens to creator

### 3. Refactor Phase
- [ ] Extract validation logic to helper functions
- [ ] Add comprehensive error messages
- [ ] Run test and confirm it passes

## Completion Criteria
- [ ] create_market instruction compiles
- [ ] Test passes: market created with correct fields
- [ ] Operation verified: L2 (test passes)

## Notes
- Impact scope: Foundation for all market operations
- Constraints: Minimum 100 USDC liquidity
- Initial pools should have equal USDC, so YES/NO prices start at 50%
- Fee is 0.3% (30 bps) = fee_bps: 30
