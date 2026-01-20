#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod seeds;
pub mod state;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod pda_verification;

#[cfg(test)]
mod integration_tests;

#[cfg(test)]
mod edge_case_tests;

pub use errors::PredictionError;
pub use instructions::*;
pub use seeds::*;
pub use state::*;

declare_id!("11111111111111111111111111111111");

#[program]
pub mod prediction {
    use super::*;

    pub fn create_market(
        ctx: Context<CreateMarket>,
        market_id: u64,
        question: String,
        description: String,
        end_time: i64,
        resolve_time: i64,
        fee_bps: u16,
    ) -> Result<()> {
        crate::instructions::create_market(
            ctx,
            market_id,
            question,
            description,
            end_time,
            resolve_time,
            fee_bps,
        )
    }

    pub fn trade(
        ctx: Context<Trade>,
        side: PoolSide,
        usdc_amount: u64,
        min_tokens_out: u64,
    ) -> Result<()> {
        crate::instructions::trade(ctx, side, usdc_amount, min_tokens_out)
    }

    pub fn add_liquidity(
        ctx: Context<AddLiquidity>,
        side: PoolSide,
        usdc_amount: u64,
        token_amount: u64,
        min_lp_tokens_out: u64,
    ) -> Result<()> {
        crate::instructions::add_liquidity(ctx, side, usdc_amount, token_amount, min_lp_tokens_out)
    }

    pub fn remove_liquidity(
        ctx: Context<RemoveLiquidity>,
        side: PoolSide,
        lp_tokens_in: u64,
    ) -> Result<()> {
        crate::instructions::remove_liquidity(ctx, side, lp_tokens_in)
    }

    pub fn lock_market(ctx: Context<LockMarket>) -> Result<()> {
        crate::instructions::lock_market(ctx)
    }

    pub fn resolve_market(ctx: Context<ResolveMarket>, result: MarketResult) -> Result<()> {
        crate::instructions::resolve_market(ctx, result)
    }

    pub fn claim_winnings(ctx: Context<ClaimWinnings>) -> Result<()> {
        crate::instructions::claim_winnings(ctx)
    }
}
