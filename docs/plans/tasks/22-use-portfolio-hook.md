# Task: usePortfolio Hook

Metadata:
- Dependencies: task-19 (market hooks)
- Provides: app/hooks/usePortfolio.ts
- Size: Small (1 file)

## Implementation Content
Create React hook for fetching user portfolio:
- Active positions (YES/NO token balances per market)
- LP positions (LP token balances per pool)
- Claimable winnings (resolved markets with winning tokens)

## Target Files
- [ ] `app/hooks/usePortfolio.ts`
- [ ] Update `app/hooks/index.ts`

## Implementation Steps (TDD: Red-Green-Refactor)

### 1. Red Phase
- [ ] Define portfolio data structure
- [ ] Identify required token account queries

### 2. Green Phase
- [ ] Implement usePortfolio hook:

```typescript
// app/hooks/usePortfolio.ts
"use client";

import { useEffect, useState, useCallback } from "react";
import { useSolanaClient, useWalletConnection } from "@solana/react-hooks";
import { fetchAllMarketAccounts, fetchPoolAccount } from "@/generated/prediction";
import type { Market, Pool, MarketStatus, MarketResult } from "@/generated/prediction";

interface Position {
  marketId: bigint;
  question: string;
  yesBalance: bigint;
  noBalance: bigint;
  status: MarketStatus;
  result: MarketResult;
}

interface LpPosition {
  marketId: bigint;
  question: string;
  side: "yes" | "no";
  lpBalance: bigint;
  poolShare: number; // percentage
}

interface ClaimableWinning {
  marketId: bigint;
  question: string;
  winningTokens: bigint;
  payout: bigint; // 1:1 with USDC
}

interface Portfolio {
  positions: Position[];
  lpPositions: LpPosition[];
  claimable: ClaimableWinning[];
  totalValue: bigint; // estimated USDC value
}

interface UsePortfolioResult {
  portfolio: Portfolio | null;
  isLoading: boolean;
  error: Error | null;
  refetch: () => void;
}

export function usePortfolio(): UsePortfolioResult {
  const client = useSolanaClient();
  const { wallet } = useWalletConnection();
  const [portfolio, setPortfolio] = useState<Portfolio | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<Error | null>(null);

  const fetchPortfolio = useCallback(async () => {
    if (!wallet) {
      setPortfolio(null);
      return;
    }

    setIsLoading(true);
    setError(null);

    try {
      const walletAddress = wallet.account.address;

      // Fetch all markets
      const marketAccounts = await fetchAllMarketAccounts(client.rpc);

      const positions: Position[] = [];
      const lpPositions: LpPosition[] = [];
      const claimable: ClaimableWinning[] = [];

      for (const marketAccount of marketAccounts) {
        const market = marketAccount.data;

        // Get user's YES token balance
        const yesBalance = await getTokenBalance(
          client.rpc,
          walletAddress,
          market.yesMint
        );

        // Get user's NO token balance
        const noBalance = await getTokenBalance(
          client.rpc,
          walletAddress,
          market.noMint
        );

        if (yesBalance > 0n || noBalance > 0n) {
          positions.push({
            marketId: market.id,
            question: market.question,
            yesBalance,
            noBalance,
            status: market.status,
            result: market.result,
          });

          // Check for claimable winnings
          if (market.status === "Resolved") {
            const winningTokens =
              market.result === "Yes" ? yesBalance : noBalance;
            if (winningTokens > 0n) {
              claimable.push({
                marketId: market.id,
                question: market.question,
                winningTokens,
                payout: winningTokens, // 1:1
              });
            }
          }
        }

        // Get LP balances for each pool
        // ... (similar pattern for LP tokens)
      }

      setPortfolio({
        positions,
        lpPositions,
        claimable,
        totalValue: calculateTotalValue(positions, lpPositions),
      });
    } catch (e) {
      const err = e instanceof Error ? e : new Error("Failed to fetch portfolio");
      setError(err);
    } finally {
      setIsLoading(false);
    }
  }, [client, wallet]);

  useEffect(() => {
    fetchPortfolio();
  }, [fetchPortfolio]);

  return { portfolio, isLoading, error, refetch: fetchPortfolio };
}

// Helper to get token balance
async function getTokenBalance(
  rpc: any,
  owner: string,
  mint: string
): Promise<bigint> {
  // Implementation using getTokenAccountsByOwner
  return 0n; // placeholder
}

function calculateTotalValue(
  positions: Position[],
  lpPositions: LpPosition[]
): bigint {
  // Sum up estimated values
  return 0n; // placeholder
}
```

- [ ] Update hooks index:

```typescript
export { usePortfolio } from "./usePortfolio";
```

### 3. Refactor Phase
- [ ] Optimize with batched RPC calls
- [ ] Add value estimation logic
- [ ] Cache results appropriately

## Completion Criteria
- [ ] Portfolio data structure complete
- [ ] Fetches all user positions
- [ ] Identifies claimable winnings
- [ ] Build passes
- [ ] Operation verified: L3 (build succeeds)

## Notes
- Only fetches for connected wallet
- Groups positions by market
- Calculates estimated value
