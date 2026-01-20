use crate::errors::*;
use crate::seeds::*;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

/// Integer square root using Newton's method (deterministic)
fn integer_sqrt(n: u128) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut x = (n as u64).max(1);
    let mut y = (x + n as u64 / x) / 2;
    while y < x {
        x = y;
        y = (y + n as u64 / y) / 2;
    }
    x
}

/// Creates a new prediction market
pub fn create_market(
    ctx: Context<CreateMarket>,
    market_id: u64,
    question: String,
    description: String,
    end_time: i64,
    resolve_time: i64,
    fee_bps: u16,
) -> Result<()> {
    // Validate inputs
    require!(!question.is_empty(), PredictionError::InvalidAmount);
    require!(
        question.len() <= MAX_QUESTION_LEN,
        PredictionError::InvalidAmount
    );
    require!(
        description.len() <= MAX_DESCRIPTION_LEN,
        PredictionError::InvalidAmount
    );
    require!(fee_bps <= 10000, PredictionError::InvalidTradingFee);

    let now = Clock::get()?.unix_timestamp;
    require!(end_time > now, PredictionError::InvalidTime);
    require!(resolve_time >= end_time, PredictionError::InvalidTime);

    let market = &mut ctx.accounts.market;
    market.id = market_id;
    market.question = question;
    market.description = description;
    market.creator = ctx.accounts.creator.key();
    market.oracle = ctx.accounts.oracle.key();
    market.end_time = end_time;
    market.resolve_time = resolve_time;
    market.yes_mint = ctx.accounts.yes_mint.key();
    market.no_mint = ctx.accounts.no_mint.key();
    market.yes_pool = ctx.accounts.yes_pool.key();
    market.no_pool = ctx.accounts.no_pool.key();
    market.vault_usdc = ctx.accounts.vault_usdc.key();
    market.status = MarketStatus::Open;
    market.result = MarketResult::Undecided;
    market.total_liquidity = 0;
    market.created_at = now;
    market.bump = ctx.bumps.market;
    market.vault_authority_bump = ctx.bumps.vault_authority;
    market.yes_mint_authority_bump = ctx.bumps.yes_mint_authority;
    market.no_mint_authority_bump = ctx.bumps.no_mint_authority;
    market.yes_lp_mint_authority_bump = ctx.bumps.yes_lp_mint_authority;
    market.no_lp_mint_authority_bump = ctx.bumps.no_lp_mint_authority;

    // Initialize YES pool
    let yes_pool = &mut ctx.accounts.yes_pool;
    yes_pool.market = market.key();
    yes_pool.side = PoolSide::Yes;
    yes_pool.usdc_reserve = 0;
    yes_pool.token_reserve = 0;
    yes_pool.lp_mint = ctx.accounts.yes_lp_mint.key();
    yes_pool.total_lp_supply = 0;
    yes_pool.fee_bps = fee_bps;
    yes_pool.collected_fees = 0;
    yes_pool.bump = ctx.bumps.yes_pool;

    // Initialize NO pool
    let no_pool = &mut ctx.accounts.no_pool;
    no_pool.market = market.key();
    no_pool.side = PoolSide::No;
    no_pool.usdc_reserve = 0;
    no_pool.token_reserve = 0;
    no_pool.lp_mint = ctx.accounts.no_lp_mint.key();
    no_pool.total_lp_supply = 0;
    no_pool.fee_bps = fee_bps;
    no_pool.collected_fees = 0;
    no_pool.bump = ctx.bumps.no_pool;

    Ok(())
}

/// Executes a trade: swap USDC for YES/NO tokens via AMM
pub fn trade(
    ctx: Context<Trade>,
    side: PoolSide,
    usdc_amount: u64,
    min_tokens_out: u64,
) -> Result<()> {
    require!(usdc_amount > 0, PredictionError::InvalidAmount);

    let market = &ctx.accounts.market;
    require!(
        market.status == MarketStatus::Open,
        PredictionError::MarketNotOpen
    );

    let now = Clock::get()?.unix_timestamp;
    require!(now < market.end_time, PredictionError::MarketNotOpen);

    let pool = match side {
        PoolSide::Yes => &mut ctx.accounts.yes_pool,
        PoolSide::No => &mut ctx.accounts.no_pool,
    };

    require!(pool.market == market.key(), PredictionError::InvalidPool);
    require!(pool.side == side, PredictionError::InvalidPool);

    // Save pool details for later use
    let market_key = pool.market;
    let pool_bump = pool.bump;

    // Calculate trading fee
    let fee_amount = usdc_amount
        .checked_mul(pool.fee_bps as u64)
        .ok_or(PredictionError::CalculationOverflow)?
        .checked_div(10_000)
        .ok_or(PredictionError::CalculationOverflow)?;

    // USDC amount after fee
    let usdc_after_fee = usdc_amount
        .checked_sub(fee_amount)
        .ok_or(PredictionError::CalculationOverflow)?;

    // Update collected fees
    pool.collected_fees = pool
        .collected_fees
        .checked_add(fee_amount)
        .ok_or(PredictionError::CalculationOverflow)?;

    // Calculate output using constant product AMM: x * y = k
    // tokens_out = (usdc_after_fee * token_reserve) / (usdc_reserve + usdc_after_fee)
    let tokens_out = usdc_after_fee
        .checked_mul(pool.token_reserve)
        .ok_or(PredictionError::CalculationOverflow)?
        .checked_div(
            pool.usdc_reserve
                .checked_add(usdc_after_fee)
                .ok_or(PredictionError::CalculationOverflow)?,
        )
        .ok_or(PredictionError::CalculationOverflow)?;

    require!(
        tokens_out >= min_tokens_out,
        PredictionError::InsufficientLiquidity
    );

    // Update pool reserves (only usdc_after_fee goes to pool, fee is collected)
    pool.usdc_reserve = pool
        .usdc_reserve
        .checked_add(usdc_after_fee)
        .ok_or(PredictionError::CalculationOverflow)?;
    pool.token_reserve = pool
        .token_reserve
        .checked_sub(tokens_out)
        .ok_or(PredictionError::InsufficientLiquidity)?;

    // Drop mutable borrow before doing transfers
    drop(pool);

    // Transfer USDC from user to pool
    anchor_spl::token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::Transfer {
                from: ctx.accounts.user_usdc.to_account_info(),
                to: ctx.accounts.pool_usdc.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        usdc_amount,
    )?;

    // Transfer tokens to user (using pool PDA as authority)
    let side_byte: u8 = match side {
        PoolSide::Yes => 0,
        PoolSide::No => 1,
    };

    let seeds: &[&[&[u8]]] = &[&[b"pool", market_key.as_ref(), &[side_byte], &[pool_bump]]];

    anchor_spl::token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::Transfer {
                from: ctx.accounts.pool_tokens.to_account_info(),
                to: ctx.accounts.user_tokens.to_account_info(),
                authority: match side {
                    PoolSide::Yes => ctx.accounts.yes_pool.to_account_info(),
                    PoolSide::No => ctx.accounts.no_pool.to_account_info(),
                },
            },
            seeds,
        ),
        tokens_out,
    )?;

    Ok(())
}

/// Adds liquidity to a pool
pub fn add_liquidity(
    ctx: Context<AddLiquidity>,
    side: PoolSide,
    usdc_amount: u64,
    token_amount: u64,
) -> Result<()> {
    require!(
        usdc_amount > 0 && token_amount > 0,
        PredictionError::InvalidAmount
    );

    let market = &ctx.accounts.market;
    require!(
        market.status == MarketStatus::Open,
        PredictionError::MarketNotOpen
    );

    let pool = &mut ctx.accounts.pool;
    require!(pool.market == market.key(), PredictionError::InvalidPool);
    require!(pool.side == side, PredictionError::InvalidPool);

    // Calculate LP tokens to mint
    let lp_tokens_to_mint = if pool.total_lp_supply == 0 {
        // Initial liquidity: mint sqrt(usdc_amount * token_amount) LP tokens
        let product = (usdc_amount as u128)
            .checked_mul(token_amount as u128)
            .ok_or(PredictionError::CalculationOverflow)?;
        // Integer square root using Newton's method (deterministic)
        let sqrt = integer_sqrt(product);
        sqrt
    } else {
        // Proportional minting
        let usdc_ratio = (usdc_amount as u128)
            .checked_mul(pool.total_lp_supply as u128)
            .ok_or(PredictionError::CalculationOverflow)?
            .checked_div(pool.usdc_reserve as u128)
            .ok_or(PredictionError::CalculationOverflow)? as u64;

        let token_ratio = (token_amount as u128)
            .checked_mul(pool.total_lp_supply as u128)
            .ok_or(PredictionError::CalculationOverflow)?
            .checked_div(pool.token_reserve as u128)
            .ok_or(PredictionError::CalculationOverflow)? as u64;

        std::cmp::min(usdc_ratio, token_ratio)
    };

    require!(lp_tokens_to_mint > 0, PredictionError::InvalidAmount);

    // Transfer USDC from user to pool
    anchor_spl::token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::Transfer {
                from: ctx.accounts.user_usdc.to_account_info(),
                to: ctx.accounts.pool_usdc.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        usdc_amount,
    )?;

    // Transfer tokens from user to pool
    anchor_spl::token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::Transfer {
                from: ctx.accounts.user_tokens.to_account_info(),
                to: ctx.accounts.pool_tokens.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        token_amount,
    )?;

    // Mint LP tokens to user
    let pool_bump = pool.bump;
    let market_key = pool.market;
    let side_byte: u8 = match side {
        PoolSide::Yes => 0,
        PoolSide::No => 1,
    };

    let seeds: &[&[&[u8]]] = &[&[b"lp_mint", market_key.as_ref(), &[side_byte], &[pool_bump]]];

    anchor_spl::token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::MintTo {
                mint: ctx.accounts.lp_mint.to_account_info(),
                to: ctx.accounts.user_lp_tokens.to_account_info(),
                authority: ctx.accounts.lp_mint_authority.to_account_info(),
            },
            seeds,
        ),
        lp_tokens_to_mint,
    )?;

    // Update pool state
    pool.usdc_reserve = pool
        .usdc_reserve
        .checked_add(usdc_amount)
        .ok_or(PredictionError::CalculationOverflow)?;
    pool.token_reserve = pool
        .token_reserve
        .checked_add(token_amount)
        .ok_or(PredictionError::CalculationOverflow)?;
    pool.total_lp_supply = pool
        .total_lp_supply
        .checked_add(lp_tokens_to_mint)
        .ok_or(PredictionError::CalculationOverflow)?;

    Ok(())
}

/// Removes liquidity from a pool
pub fn remove_liquidity(
    ctx: Context<RemoveLiquidity>,
    side: PoolSide,
    lp_tokens_in: u64,
) -> Result<()> {
    require!(lp_tokens_in > 0, PredictionError::InvalidAmount);

    let market = &ctx.accounts.market;
    let pool = &mut ctx.accounts.pool;

    require!(pool.market == market.key(), PredictionError::InvalidPool);
    require!(pool.side == side, PredictionError::InvalidPool);
    require!(
        pool.total_lp_supply > 0,
        PredictionError::InsufficientLiquidity
    );

    // Calculate amounts to return
    let usdc_amount = (lp_tokens_in as u128)
        .checked_mul(pool.usdc_reserve as u128)
        .ok_or(PredictionError::CalculationOverflow)?
        .checked_div(pool.total_lp_supply as u128)
        .ok_or(PredictionError::CalculationOverflow)? as u64;

    let token_amount = (lp_tokens_in as u128)
        .checked_mul(pool.token_reserve as u128)
        .ok_or(PredictionError::CalculationOverflow)?
        .checked_div(pool.total_lp_supply as u128)
        .ok_or(PredictionError::CalculationOverflow)? as u64;

    require!(
        usdc_amount > 0 && token_amount > 0,
        PredictionError::InvalidAmount
    );

    // Burn LP tokens
    anchor_spl::token::burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::Burn {
                mint: ctx.accounts.lp_mint.to_account_info(),
                from: ctx.accounts.user_lp_tokens.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        lp_tokens_in,
    )?;

    // Transfer USDC to user
    let pool_bump = pool.bump;
    let market_key = pool.market;
    let side_byte: u8 = match side {
        PoolSide::Yes => 0,
        PoolSide::No => 1,
    };

    let seeds: &[&[&[u8]]] = &[&[b"pool", market_key.as_ref(), &[side_byte], &[pool_bump]]];

    anchor_spl::token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::Transfer {
                from: ctx.accounts.pool_usdc.to_account_info(),
                to: ctx.accounts.user_usdc.to_account_info(),
                authority: ctx.accounts.pool_authority.to_account_info(),
            },
            seeds,
        ),
        usdc_amount,
    )?;

    // Transfer tokens to user
    anchor_spl::token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::Transfer {
                from: ctx.accounts.pool_tokens.to_account_info(),
                to: ctx.accounts.user_tokens.to_account_info(),
                authority: ctx.accounts.pool_authority.to_account_info(),
            },
            seeds,
        ),
        token_amount,
    )?;

    // Update pool state
    pool.usdc_reserve = pool
        .usdc_reserve
        .checked_sub(usdc_amount)
        .ok_or(PredictionError::InsufficientLiquidity)?;
    pool.token_reserve = pool
        .token_reserve
        .checked_sub(token_amount)
        .ok_or(PredictionError::InsufficientLiquidity)?;
    pool.total_lp_supply = pool
        .total_lp_supply
        .checked_sub(lp_tokens_in)
        .ok_or(PredictionError::InsufficientLiquidity)?;

    Ok(())
}

/// Locks a market (stops trading, prepares for resolution)
pub fn lock_market(ctx: Context<LockMarket>) -> Result<()> {
    let market = &mut ctx.accounts.market;

    require!(
        market.oracle == ctx.accounts.oracle.key(),
        PredictionError::Unauthorized
    );
    require!(
        market.status == MarketStatus::Open,
        PredictionError::InvalidMarketStatus
    );

    let now = Clock::get()?.unix_timestamp;
    require!(now >= market.end_time, PredictionError::TimeHasNotPassed);

    market.status = MarketStatus::Locked;

    Ok(())
}

/// Resolves a market with a YES or NO outcome
pub fn resolve_market(ctx: Context<ResolveMarket>, result: MarketResult) -> Result<()> {
    require!(
        result == MarketResult::Yes || result == MarketResult::No,
        PredictionError::InvalidOutcome
    );

    let market = &mut ctx.accounts.market;

    require!(
        market.oracle == ctx.accounts.oracle.key(),
        PredictionError::Unauthorized
    );
    require!(
        market.status == MarketStatus::Locked,
        PredictionError::InvalidMarketStatus
    );

    let now = Clock::get()?.unix_timestamp;
    require!(
        now >= market.resolve_time,
        PredictionError::TimeHasNotPassed
    );

    market.status = MarketStatus::Resolved;
    market.result = result;

    Ok(())
}

/// Claims winnings after market resolution
pub fn claim_winnings(ctx: Context<ClaimWinnings>) -> Result<()> {
    let market = &ctx.accounts.market;

    require!(
        market.status == MarketStatus::Resolved,
        PredictionError::MarketNotResolved
    );
    require!(
        market.result != MarketResult::Undecided,
        PredictionError::InvalidOutcome
    );

    // Check if user has winning tokens
    let winning_token_balance = if market.result == MarketResult::Yes {
        ctx.accounts.user_yes_tokens.amount
    } else {
        ctx.accounts.user_no_tokens.amount
    };

    require!(
        winning_token_balance > 0,
        PredictionError::InsufficientBalance
    );

    // Burn winning tokens
    let winning_token_account = if market.result == MarketResult::Yes {
        ctx.accounts.user_yes_tokens.to_account_info()
    } else {
        ctx.accounts.user_no_tokens.to_account_info()
    };

    let winning_mint = if market.result == MarketResult::Yes {
        ctx.accounts.yes_mint.to_account_info()
    } else {
        ctx.accounts.no_mint.to_account_info()
    };

    anchor_spl::token::burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::Burn {
                mint: winning_mint,
                from: winning_token_account,
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        winning_token_balance,
    )?;

    // Transfer USDC from vault to user
    let market_key = market.key();
    // Get vault bump from vault PDA derivation
    let (_, vault_bump) =
        Pubkey::find_program_address(&[b"vault", market_key.as_ref()], &crate::ID);
    let vault_seeds: &[&[&[u8]]] = &[&[b"vault", market_key.as_ref(), &[vault_bump]]];

    anchor_spl::token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::Transfer {
                from: ctx.accounts.vault_usdc.to_account_info(),
                to: ctx.accounts.user_usdc.to_account_info(),
                authority: ctx.accounts.vault_authority.to_account_info(),
            },
            vault_seeds,
        ),
        winning_token_balance,
    )?;

    Ok(())
}

// ============ Account Structures ============

#[derive(Accounts)]
#[instruction(market_id: u64, question: String, description: String, end_time: i64, resolve_time: i64, fee_bps: u16)]
pub struct CreateMarket<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    /// CHECK: Oracle is validated by storing its pubkey; can be any account designated for resolution
    #[account(
        constraint = oracle.key() != Pubkey::default() @ PredictionError::InvalidOracle
    )]
    pub oracle: UncheckedAccount<'info>,

    #[account(
        init,
        payer = creator,
        space = Market::SPACE,
        seeds = [MARKET_SEED, market_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub market: Account<'info, Market>,

    #[account(
        init,
        payer = creator,
        space = Pool::SPACE,
        seeds = [POOL_SEED, market.key().as_ref(), &[0u8]],
        bump,
    )]
    pub yes_pool: Account<'info, Pool>,

    #[account(
        init,
        payer = creator,
        space = Pool::SPACE,
        seeds = [POOL_SEED, market.key().as_ref(), &[1u8]],
        bump,
    )]
    pub no_pool: Account<'info, Pool>,

    /// YES outcome token mint - initialized as PDA
    #[account(
        init,
        payer = creator,
        mint::decimals = 6,
        mint::authority = yes_mint_authority,
        seeds = [YES_MINT_SEED, market.key().as_ref()],
        bump,
    )]
    pub yes_mint: Account<'info, Mint>,

    /// NO outcome token mint - initialized as PDA
    #[account(
        init,
        payer = creator,
        mint::decimals = 6,
        mint::authority = no_mint_authority,
        seeds = [NO_MINT_SEED, market.key().as_ref()],
        bump,
    )]
    pub no_mint: Account<'info, Mint>,

    /// YES LP token mint - initialized as PDA
    #[account(
        init,
        payer = creator,
        mint::decimals = 6,
        mint::authority = yes_lp_mint_authority,
        seeds = [LP_MINT_SEED, market.key().as_ref(), &[0u8]],
        bump,
    )]
    pub yes_lp_mint: Account<'info, Mint>,

    /// NO LP token mint - initialized as PDA
    #[account(
        init,
        payer = creator,
        mint::decimals = 6,
        mint::authority = no_lp_mint_authority,
        seeds = [LP_MINT_SEED, market.key().as_ref(), &[1u8]],
        bump,
    )]
    pub no_lp_mint: Account<'info, Mint>,

    /// USDC vault token account - initialized as PDA
    #[account(
        init,
        payer = creator,
        token::mint = usdc_mint,
        token::authority = vault_authority,
        seeds = [VAULT_SEED, market.key().as_ref()],
        bump,
    )]
    pub vault_usdc: Account<'info, TokenAccount>,

    /// CHECK: Vault authority PDA - used to sign transfers from vault
    #[account(
        seeds = [VAULT_AUTHORITY_SEED, market.key().as_ref()],
        bump,
    )]
    pub vault_authority: UncheckedAccount<'info>,

    /// CHECK: YES mint authority PDA
    #[account(
        seeds = [YES_MINT_AUTHORITY_SEED, market.key().as_ref()],
        bump,
    )]
    pub yes_mint_authority: UncheckedAccount<'info>,

    /// CHECK: NO mint authority PDA
    #[account(
        seeds = [NO_MINT_AUTHORITY_SEED, market.key().as_ref()],
        bump,
    )]
    pub no_mint_authority: UncheckedAccount<'info>,

    /// CHECK: YES LP mint authority PDA
    #[account(
        seeds = [YES_LP_MINT_AUTHORITY_SEED, market.key().as_ref()],
        bump,
    )]
    pub yes_lp_mint_authority: UncheckedAccount<'info>,

    /// CHECK: NO LP mint authority PDA
    #[account(
        seeds = [NO_LP_MINT_AUTHORITY_SEED, market.key().as_ref()],
        bump,
    )]
    pub no_lp_mint_authority: UncheckedAccount<'info>,

    /// USDC mint (existing SPL token)
    pub usdc_mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Trade<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub market: Account<'info, Market>,

    #[account(mut)]
    pub yes_pool: Account<'info, Pool>,

    #[account(mut)]
    pub no_pool: Account<'info, Pool>,

    #[account(mut)]
    pub user_usdc: Account<'info, TokenAccount>,

    #[account(mut)]
    pub pool_usdc: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_tokens: Account<'info, TokenAccount>,

    #[account(mut)]
    pub pool_tokens: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct AddLiquidity<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub market: Account<'info, Market>,

    #[account(mut)]
    pub pool: Account<'info, Pool>,

    #[account(mut)]
    pub user_usdc: Account<'info, TokenAccount>,

    #[account(mut)]
    pub pool_usdc: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_tokens: Account<'info, TokenAccount>,

    #[account(mut)]
    pub pool_tokens: Account<'info, TokenAccount>,

    #[account(mut)]
    pub lp_mint: Account<'info, Mint>,

    #[account(mut)]
    pub user_lp_tokens: Account<'info, TokenAccount>,

    pub lp_mint_authority: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct RemoveLiquidity<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub market: Account<'info, Market>,

    #[account(mut)]
    pub pool: Account<'info, Pool>,

    #[account(mut)]
    pub user_usdc: Account<'info, TokenAccount>,

    #[account(mut)]
    pub pool_usdc: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_tokens: Account<'info, TokenAccount>,

    #[account(mut)]
    pub pool_tokens: Account<'info, TokenAccount>,

    #[account(mut)]
    pub lp_mint: Account<'info, Mint>,

    #[account(mut)]
    pub user_lp_tokens: Account<'info, TokenAccount>,

    pub pool_authority: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct LockMarket<'info> {
    pub oracle: Signer<'info>,

    #[account(mut)]
    pub market: Account<'info, Market>,
}

#[derive(Accounts)]
pub struct ResolveMarket<'info> {
    pub oracle: Signer<'info>,

    #[account(mut)]
    pub market: Account<'info, Market>,
}

#[derive(Accounts)]
pub struct ClaimWinnings<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub market: Account<'info, Market>,

    #[account(mut)]
    pub user_yes_tokens: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_no_tokens: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_usdc: Account<'info, TokenAccount>,

    #[account(mut)]
    pub vault_usdc: Account<'info, TokenAccount>,

    pub yes_mint: Account<'info, Mint>,
    pub no_mint: Account<'info, Mint>,

    pub vault_authority: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
}
