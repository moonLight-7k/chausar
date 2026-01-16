use anchor_lang::prelude::*;

/// Maximum length for market question (280 characters)
pub const MAX_QUESTION_LEN: usize = 280;

/// Maximum length for market description (1000 characters)
pub const MAX_DESCRIPTION_LEN: usize = 1000;

/// Status of a prediction market
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Default)]
pub enum MarketStatus {
    /// Trading is active
    #[default]
    Open,
    /// Trading ended, awaiting resolution
    Locked,
    /// Outcome has been determined
    Resolved,
}

/// Result of a prediction market
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Default)]
pub enum MarketResult {
    /// Outcome not yet determined
    #[default]
    Undecided,
    /// Market resolved to Yes
    Yes,
    /// Market resolved to No
    No,
}

/// Side of a pool (Yes or No)
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum PoolSide {
    /// Yes side of the market
    Yes,
    /// No side of the market
    No,
}

/// Market account storing prediction market data
///
/// Space calculation:
/// - discriminator: 8 bytes
/// - id: 8 bytes (u64)
/// - question: 4 + 280 bytes (String prefix + max chars)
/// - description: 4 + 1000 bytes (String prefix + max chars)
/// - creator: 32 bytes (Pubkey)
/// - oracle: 32 bytes (Pubkey)
/// - end_time: 8 bytes (i64)
/// - resolve_time: 8 bytes (i64)
/// - yes_mint: 32 bytes (Pubkey)
/// - no_mint: 32 bytes (Pubkey)
/// - yes_pool: 32 bytes (Pubkey)
/// - no_pool: 32 bytes (Pubkey)
/// - vault_usdc: 32 bytes (Pubkey)
/// - status: 1 byte (enum)
/// - result: 1 byte (enum)
/// - total_liquidity: 8 bytes (u64)
/// - created_at: 8 bytes (i64)
/// - bump: 1 byte (u8)
/// Total: 8 + 8 + 284 + 1004 + 32*7 + 8 + 8 + 1 + 1 + 8 + 8 + 1 = 1565 bytes
#[account]
pub struct Market {
    /// Unique market ID
    pub id: u64,

    /// Market question (max 280 chars)
    pub question: String,

    /// Market description (max 1000 chars)
    pub description: String,

    /// Market creator public key
    pub creator: Pubkey,

    /// Oracle authority for resolution
    pub oracle: Pubkey,

    /// Unix timestamp when trading ends
    pub end_time: i64,

    /// Expected resolution timestamp
    pub resolve_time: i64,

    /// YES token mint address
    pub yes_mint: Pubkey,

    /// NO token mint address
    pub no_mint: Pubkey,

    /// YES AMM pool address
    pub yes_pool: Pubkey,

    /// NO AMM pool address
    pub no_pool: Pubkey,

    /// USDC vault address
    pub vault_usdc: Pubkey,

    /// Current market status
    pub status: MarketStatus,

    /// Market resolution result
    pub result: MarketResult,

    /// Total USDC locked in the market
    pub total_liquidity: u64,

    /// Market creation timestamp
    pub created_at: i64,

    /// PDA bump seed
    pub bump: u8,
}

impl Market {
    /// Space required for Market account (including discriminator)
    pub const SPACE: usize = 8   // discriminator
        + 8                      // id
        + 4 + MAX_QUESTION_LEN   // question (string prefix + max chars)
        + 4 + MAX_DESCRIPTION_LEN // description (string prefix + max chars)
        + 32                     // creator
        + 32                     // oracle
        + 8                      // end_time
        + 8                      // resolve_time
        + 32                     // yes_mint
        + 32                     // no_mint
        + 32                     // yes_pool
        + 32                     // no_pool
        + 32                     // vault_usdc
        + 1                      // status
        + 1                      // result
        + 8                      // total_liquidity
        + 8                      // created_at
        + 1; // bump
}

/// Pool account storing AMM pool data for YES or NO side
///
/// Space calculation:
/// - discriminator: 8 bytes
/// - market: 32 bytes (Pubkey)
/// - side: 1 byte (enum)
/// - usdc_reserve: 8 bytes (u64)
/// - token_reserve: 8 bytes (u64)
/// - lp_mint: 32 bytes (Pubkey)
/// - total_lp_supply: 8 bytes (u64)
/// - fee_bps: 2 bytes (u16)
/// - collected_fees: 8 bytes (u64)
/// - bump: 1 byte (u8)
/// Total: 8 + 32 + 1 + 8 + 8 + 32 + 8 + 2 + 8 + 1 = 108 bytes
#[account]
pub struct Pool {
    /// Parent market address
    pub market: Pubkey,

    /// Pool side (Yes or No)
    pub side: PoolSide,

    /// USDC reserve in pool
    pub usdc_reserve: u64,

    /// YES/NO token reserve in pool
    pub token_reserve: u64,

    /// LP token mint address
    pub lp_mint: Pubkey,

    /// Total LP tokens in circulation
    pub total_lp_supply: u64,

    /// Trading fee in basis points (e.g., 30 = 0.3%)
    pub fee_bps: u16,

    /// Accumulated trading fees
    pub collected_fees: u64,

    /// PDA bump seed
    pub bump: u8,
}

impl Pool {
    /// Space required for Pool account (including discriminator)
    pub const SPACE: usize = 8  // discriminator
        + 32                    // market
        + 1                     // side
        + 8                     // usdc_reserve
        + 8                     // token_reserve
        + 32                    // lp_mint
        + 8                     // total_lp_supply
        + 2                     // fee_bps
        + 8                     // collected_fees
        + 1; // bump
}
