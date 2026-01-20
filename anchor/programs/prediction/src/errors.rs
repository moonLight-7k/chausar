use anchor_lang::error_code;

#[error_code]
pub enum PredictionError {
    #[msg("Market already exists")]
    MarketAlreadyExists,

    #[msg("Market not found")]
    MarketNotFound,

    #[msg("Invalid market status")]
    InvalidMarketStatus,

    #[msg("Market is not open for trading")]
    MarketNotOpen,

    #[msg("Market is not locked")]
    MarketNotLocked,

    #[msg("Market is not resolved")]
    MarketNotResolved,

    #[msg("Invalid oracle")]
    InvalidOracle,

    #[msg("Invalid amount")]
    InvalidAmount,

    #[msg("Invalid pool")]
    InvalidPool,

    #[msg("Insufficient liquidity")]
    InsufficientLiquidity,

    #[msg("Insufficient balance")]
    InsufficientBalance,

    #[msg("Invalid mint")]
    InvalidMint,

    #[msg("Invalid outcome")]
    InvalidOutcome,

    #[msg("Market not yet resolved")]
    MarketNotYetResolved,

    #[msg("Already claimed winnings")]
    AlreadyClaimedWinnings,

    #[msg("Invalid trading fee")]
    InvalidTradingFee,

    #[msg("Trading fee too high")]
    TradingFeeTooHigh,

    #[msg("Time has not passed")]
    TimeHasNotPassed,

    #[msg("Invalid time")]
    InvalidTime,

    #[msg("Overflow in calculation")]
    CalculationOverflow,

    #[msg("Invalid account")]
    InvalidAccount,

    #[msg("Unauthorized")]
    Unauthorized,

    #[msg("Invalid instruction data")]
    InvalidInstructionData,

    #[msg("Slippage tolerance exceeded")]
    SlippageExceeded,

    #[msg("Initial liquidity must be at least 100 USDC")]
    InsufficientInitialLiquidity,
}
