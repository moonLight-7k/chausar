/// PDA seed constants for the prediction market program
///
/// This module defines all seed constants used for Program Derived Address (PDA)
/// derivation in the prediction market protocol. These seeds ensure deterministic
/// and unique account addresses for markets, pools, vaults, and token mints.
///
/// # PDA Derivation Patterns
///
/// - Market: `["market", market_id.to_le_bytes()]`
/// - Pool: `["pool", market_pubkey, side_byte]`
/// - Vault: `["vault", market_pubkey]`
/// - YES Mint: `["yes_mint", market_pubkey]`
/// - NO Mint: `["no_mint", market_pubkey]`
/// - LP Mint: `["lp_mint", market_pubkey, side_byte]`

/// Seed prefix for Market PDA derivation
///
/// Used with market_id (u64 as little-endian bytes) to derive unique market accounts.
///
/// # Example
/// ```ignore
/// let (market_pda, bump) = Pubkey::find_program_address(
///     &[MARKET_SEED, &market_id.to_le_bytes()],
///     &program_id,
/// );
/// ```
pub const MARKET_SEED: &[u8] = b"market";

/// Seed prefix for Pool PDA derivation
///
/// Used with market pubkey and pool side to derive unique pool accounts
/// for both YES and NO sides of a market.
///
/// # Example
/// ```ignore
/// let side_byte = match pool_side {
///     PoolSide::Yes => 0u8,
///     PoolSide::No => 1u8,
/// };
/// let (pool_pda, bump) = Pubkey::find_program_address(
///     &[POOL_SEED, market_pubkey.as_ref(), &[side_byte]],
///     &program_id,
/// );
/// ```
pub const POOL_SEED: &[u8] = b"pool";

/// Seed prefix for Vault PDA derivation
///
/// Used with market pubkey to derive the USDC vault account for a market.
/// Each market has a single vault that holds all deposited USDC.
///
/// # Example
/// ```ignore
/// let (vault_pda, bump) = Pubkey::find_program_address(
///     &[VAULT_SEED, market_pubkey.as_ref()],
///     &program_id,
/// );
/// ```
pub const VAULT_SEED: &[u8] = b"vault";

/// Seed prefix for YES token mint PDA derivation
///
/// Used with market pubkey to derive the YES outcome token mint.
/// YES tokens represent a position betting on the "Yes" outcome.
///
/// # Example
/// ```ignore
/// let (yes_mint_pda, bump) = Pubkey::find_program_address(
///     &[YES_MINT_SEED, market_pubkey.as_ref()],
///     &program_id,
/// );
/// ```
pub const YES_MINT_SEED: &[u8] = b"yes_mint";

/// Seed prefix for NO token mint PDA derivation
///
/// Used with market pubkey to derive the NO outcome token mint.
/// NO tokens represent a position betting on the "No" outcome.
///
/// # Example
/// ```ignore
/// let (no_mint_pda, bump) = Pubkey::find_program_address(
///     &[NO_MINT_SEED, market_pubkey.as_ref()],
///     &program_id,
/// );
/// ```
pub const NO_MINT_SEED: &[u8] = b"no_mint";

/// Seed prefix for LP (Liquidity Provider) token mint PDA derivation
///
/// Used with market pubkey and pool side to derive LP token mints.
/// Each pool (YES and NO) has its own LP token for tracking liquidity positions.
///
/// # Example
/// ```ignore
/// let side_byte = match pool_side {
///     PoolSide::Yes => 0u8,
///     PoolSide::No => 1u8,
/// };
/// let (lp_mint_pda, bump) = Pubkey::find_program_address(
///     &[LP_MINT_SEED, market_pubkey.as_ref(), &[side_byte]],
///     &program_id,
/// );
/// ```
pub const LP_MINT_SEED: &[u8] = b"lp_mint";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seed_constants_are_unique() {
        // Verify all seed constants are unique to prevent PDA collisions
        let seeds: [&[u8]; 6] = [
            MARKET_SEED,
            POOL_SEED,
            VAULT_SEED,
            YES_MINT_SEED,
            NO_MINT_SEED,
            LP_MINT_SEED,
        ];

        for i in 0..seeds.len() {
            for j in (i + 1)..seeds.len() {
                assert_ne!(
                    seeds[i], seeds[j],
                    "Seed constants must be unique: {:?} == {:?}",
                    seeds[i], seeds[j]
                );
            }
        }
    }

    #[test]
    fn test_seed_constants_are_not_empty() {
        // Verify all seed constants are non-empty
        assert!(!MARKET_SEED.is_empty(), "MARKET_SEED must not be empty");
        assert!(!POOL_SEED.is_empty(), "POOL_SEED must not be empty");
        assert!(!VAULT_SEED.is_empty(), "VAULT_SEED must not be empty");
        assert!(!YES_MINT_SEED.is_empty(), "YES_MINT_SEED must not be empty");
        assert!(!NO_MINT_SEED.is_empty(), "NO_MINT_SEED must not be empty");
        assert!(!LP_MINT_SEED.is_empty(), "LP_MINT_SEED must not be empty");
    }

    #[test]
    fn test_seed_values() {
        // Verify seed constants have expected values
        assert_eq!(MARKET_SEED, b"market");
        assert_eq!(POOL_SEED, b"pool");
        assert_eq!(VAULT_SEED, b"vault");
        assert_eq!(YES_MINT_SEED, b"yes_mint");
        assert_eq!(NO_MINT_SEED, b"no_mint");
        assert_eq!(LP_MINT_SEED, b"lp_mint");
    }
}
