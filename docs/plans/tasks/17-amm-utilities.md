# Task: AMM Utilities (Frontend)

Metadata:
- Dependencies: task-16 (config)
- Provides: app/lib/amm.ts
- Size: Small (1 file)

## Implementation Content
Create frontend utilities for AMM price calculations:
- Calculate output amount for given input
- Calculate current prices (YES/NO)
- Calculate price impact
- Calculate LP token amounts

## Target Files
- [ ] `app/lib/amm.ts`
- [ ] `app/lib/__tests__/amm.test.ts` (optional)

## Implementation Steps (TDD: Red-Green-Refactor)

### 1. Red Phase
- [ ] Define function signatures
- [ ] Write unit tests for calculations

### 2. Green Phase
- [ ] Implement AMM utilities:

```typescript
// app/lib/amm.ts
import { CONFIG } from "./config";

/**
 * Calculate output tokens for given USDC input
 * Formula: dy = (y * dx * (10000 - fee)) / (x * 10000 + dx * (10000 - fee))
 */
export function calculateOutput(
  inputAmount: bigint,
  inputReserve: bigint,
  outputReserve: bigint,
  feeBps: number = CONFIG.feeBps
): bigint {
  const feeFactor = BigInt(10000 - feeBps);
  const amountWithFee = inputAmount * feeFactor;
  const numerator = outputReserve * amountWithFee;
  const denominator = inputReserve * 10000n + amountWithFee;
  return numerator / denominator;
}

/**
 * Calculate current price of YES tokens
 * Price = USDC_yes / (USDC_yes + USDC_no)
 */
export function calculateYesPrice(
  yesUsdcReserve: bigint,
  noUsdcReserve: bigint
): number {
  const total = yesUsdcReserve + noUsdcReserve;
  if (total === 0n) return 0.5;
  return Number(yesUsdcReserve) / Number(total);
}

/**
 * Calculate current price of NO tokens
 */
export function calculateNoPrice(
  yesUsdcReserve: bigint,
  noUsdcReserve: bigint
): number {
  return 1 - calculateYesPrice(yesUsdcReserve, noUsdcReserve);
}

/**
 * Calculate price impact for a trade
 * Returns percentage (0-100)
 */
export function calculatePriceImpact(
  inputAmount: bigint,
  inputReserve: bigint,
  outputReserve: bigint
): number {
  const spotPrice = Number(outputReserve) / Number(inputReserve);
  const outputAmount = calculateOutput(inputAmount, inputReserve, outputReserve);
  const effectivePrice = Number(outputAmount) / Number(inputAmount);
  const impact = (spotPrice - effectivePrice) / spotPrice * 100;
  return Math.abs(impact);
}

/**
 * Calculate LP tokens to receive for liquidity provision
 */
export function calculateLpTokens(
  usdcAmount: bigint,
  poolUsdcReserve: bigint,
  totalLpSupply: bigint
): bigint {
  if (totalLpSupply === 0n) {
    return usdcAmount; // First LP
  }
  return (usdcAmount * totalLpSupply) / poolUsdcReserve;
}

/**
 * Calculate withdrawal amounts for LP tokens
 */
export function calculateWithdrawal(
  lpAmount: bigint,
  poolUsdcReserve: bigint,
  poolTokenReserve: bigint,
  totalLpSupply: bigint
): { usdcOut: bigint; tokensOut: bigint } {
  const usdcOut = (lpAmount * poolUsdcReserve) / totalLpSupply;
  const tokensOut = (lpAmount * poolTokenReserve) / totalLpSupply;
  return { usdcOut, tokensOut };
}
```

### 3. Refactor Phase
- [ ] Add input validation
- [ ] Add error handling for edge cases
- [ ] Run tests if created

## Completion Criteria
- [ ] All calculation functions implemented
- [ ] Matches smart contract logic
- [ ] Build passes
- [ ] Operation verified: L3 (build succeeds)

## Notes
- Use bigint for precision with token amounts
- Must match smart contract AMM logic exactly
- These utilities will be used by hooks and components
