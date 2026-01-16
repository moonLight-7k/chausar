# Task: useLiquidity Hook

Metadata:
- Dependencies: task-19 (market hooks), task-17 (AMM utils)
- Provides: app/hooks/useLiquidity.ts
- Size: Small (1 file)

## Implementation Content
Create React hook for liquidity operations:
- Add liquidity transaction
- Remove liquidity transaction
- Calculate LP tokens and withdrawals

## Target Files
- [ ] `app/hooks/useLiquidity.ts`
- [ ] Update `app/hooks/index.ts`

## Implementation Steps (TDD: Red-Green-Refactor)

### 1. Red Phase
- [ ] Define hook interface
- [ ] Identify transaction requirements

### 2. Green Phase
- [ ] Implement useLiquidity hook:

```typescript
// app/hooks/useLiquidity.ts
"use client";

import { useState, useCallback } from "react";
import { useSolanaClient, useWalletConnection } from "@solana/react-hooks";
import {
  addLiquidity as buildAddLiquidityInstruction,
  removeLiquidity as buildRemoveLiquidityInstruction,
} from "@/generated/prediction";
import { calculateLpTokens, calculateWithdrawal } from "@/lib/amm";
import type { PoolSide } from "@/generated/prediction";

interface AddLiquidityParams {
  marketId: bigint;
  side: PoolSide;
  usdcAmount: bigint;
  poolUsdcReserve: bigint;
  totalLpSupply: bigint;
}

interface RemoveLiquidityParams {
  marketId: bigint;
  side: PoolSide;
  lpAmount: bigint;
  poolUsdcReserve: bigint;
  poolTokenReserve: bigint;
  totalLpSupply: bigint;
}

interface LiquidityResult {
  signature: string;
}

interface UseLiquidityResult {
  addLiquidity: (params: AddLiquidityParams) => Promise<LiquidityResult>;
  removeLiquidity: (params: RemoveLiquidityParams) => Promise<LiquidityResult>;
  isLoading: boolean;
  error: Error | null;
}

export function useLiquidity(): UseLiquidityResult {
  const client = useSolanaClient();
  const { wallet } = useWalletConnection();
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<Error | null>(null);

  const addLiquidity = useCallback(
    async (params: AddLiquidityParams): Promise<LiquidityResult> => {
      if (!wallet) {
        throw new Error("Wallet not connected");
      }

      setIsLoading(true);
      setError(null);

      try {
        const { marketId, side, usdcAmount } = params;

        // Calculate expected LP tokens
        const expectedLpTokens = calculateLpTokens(
          usdcAmount,
          params.poolUsdcReserve,
          params.totalLpSupply
        );

        // Build transaction
        const instruction = buildAddLiquidityInstruction({
          side,
          usdcAmount,
          // ... account addresses
        });

        // Send transaction
        const signature = await wallet.sendTransaction(
          [instruction],
          client.rpc
        );

        await client.rpc.confirmTransaction(signature);

        return { signature };
      } catch (e) {
        const err = e instanceof Error ? e : new Error("Add liquidity failed");
        setError(err);
        throw err;
      } finally {
        setIsLoading(false);
      }
    },
    [client, wallet]
  );

  const removeLiquidity = useCallback(
    async (params: RemoveLiquidityParams): Promise<LiquidityResult> => {
      if (!wallet) {
        throw new Error("Wallet not connected");
      }

      setIsLoading(true);
      setError(null);

      try {
        const { marketId, side, lpAmount } = params;

        // Calculate expected outputs
        const { usdcOut, tokensOut } = calculateWithdrawal(
          lpAmount,
          params.poolUsdcReserve,
          params.poolTokenReserve,
          params.totalLpSupply
        );

        // Build transaction
        const instruction = buildRemoveLiquidityInstruction({
          side,
          lpAmount,
          // ... account addresses
        });

        // Send transaction
        const signature = await wallet.sendTransaction(
          [instruction],
          client.rpc
        );

        await client.rpc.confirmTransaction(signature);

        return { signature };
      } catch (e) {
        const err =
          e instanceof Error ? e : new Error("Remove liquidity failed");
        setError(err);
        throw err;
      } finally {
        setIsLoading(false);
      }
    },
    [client, wallet]
  );

  return { addLiquidity, removeLiquidity, isLoading, error };
}
```

- [ ] Update hooks index:

```typescript
export { useLiquidity } from "./useLiquidity";
```

### 3. Refactor Phase
- [ ] Add preview functions for UI
- [ ] Handle associated token account creation
- [ ] Verify transaction building

## Completion Criteria
- [ ] Both add/remove functions implemented
- [ ] Calculates expected LP tokens
- [ ] Handles errors gracefully
- [ ] Build passes
- [ ] Operation verified: L3 (build succeeds)

## Notes
- Uses AMM utilities for calculations
- Creates token accounts if needed
- Returns LP tokens on add, USDC+tokens on remove
