# Task: useTrade Hook

Metadata:
- Dependencies: task-19 (market hooks), task-17 (AMM utils)
- Provides: app/hooks/useTrade.ts
- Size: Small (1 file)

## Implementation Content
Create React hook for executing trade transactions:
- Build and send trade transaction
- Handle transaction confirmation
- Calculate expected output and slippage

## Target Files
- [ ] `app/hooks/useTrade.ts`
- [ ] Update `app/hooks/index.ts`

## Implementation Steps (TDD: Red-Green-Refactor)

### 1. Red Phase
- [ ] Define hook interface
- [ ] Identify transaction building requirements

### 2. Green Phase
- [ ] Implement useTrade hook:

```typescript
// app/hooks/useTrade.ts
"use client";

import { useState, useCallback } from "react";
import { useSolanaClient, useWalletConnection } from "@solana/react-hooks";
import { trade as buildTradeInstruction } from "@/generated/prediction";
import { calculateOutput } from "@/lib/amm";
import { CONFIG } from "@/lib/config";
import type { PoolSide } from "@/generated/prediction";

interface TradeParams {
  marketId: bigint;
  side: PoolSide;
  usdcAmount: bigint;
  inputReserve: bigint;
  outputReserve: bigint;
  slippageBps?: number;
}

interface TradeResult {
  signature: string;
  expectedOutput: bigint;
  actualOutput?: bigint;
}

interface UseTradeResult {
  trade: (params: TradeParams) => Promise<TradeResult>;
  isLoading: boolean;
  error: Error | null;
}

export function useTrade(): UseTradeResult {
  const client = useSolanaClient();
  const { wallet } = useWalletConnection();
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<Error | null>(null);

  const executeTrade = useCallback(
    async (params: TradeParams): Promise<TradeResult> => {
      if (!wallet) {
        throw new Error("Wallet not connected");
      }

      setIsLoading(true);
      setError(null);

      try {
        const {
          marketId,
          side,
          usdcAmount,
          inputReserve,
          outputReserve,
          slippageBps = CONFIG.defaultSlippageBps,
        } = params;

        // Calculate expected output
        const expectedOutput = calculateOutput(
          usdcAmount,
          inputReserve,
          outputReserve
        );

        // Calculate min output with slippage
        const minOutput =
          (expectedOutput * BigInt(10000 - slippageBps)) / 10000n;

        // Build transaction
        const instruction = buildTradeInstruction({
          side,
          amountIn: usdcAmount,
          minAmountOut: minOutput,
          // ... account addresses
        });

        // Send transaction
        const signature = await wallet.sendTransaction(
          [instruction],
          client.rpc
        );

        // Wait for confirmation
        await client.rpc.confirmTransaction(signature);

        return { signature, expectedOutput };
      } catch (e) {
        const err = e instanceof Error ? e : new Error("Trade failed");
        setError(err);
        throw err;
      } finally {
        setIsLoading(false);
      }
    },
    [client, wallet]
  );

  return { trade: executeTrade, isLoading, error };
}
```

- [ ] Update hooks index:

```typescript
export { useTrade } from "./useTrade";
```

### 3. Refactor Phase
- [ ] Add proper account derivation
- [ ] Add transaction simulation before send
- [ ] Handle priority fees if needed

## Completion Criteria
- [ ] Hook implemented with slippage protection
- [ ] Calculates expected output
- [ ] Handles errors gracefully
- [ ] Build passes
- [ ] Operation verified: L3 (build succeeds)

## Notes
- Uses AMM utilities for calculations
- Default slippage is 5% (500 bps)
- Wallet must be connected
