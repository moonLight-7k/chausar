# Task: LiquidityPanel Component

Metadata:
- Dependencies: task-21 (useLiquidity), task-33 (AmountInput)
- Provides: app/components/LiquidityPanel.tsx
- Size: Small (1-2 files)

## Implementation Content
Create the liquidity management interface:
- Add/Remove liquidity tabs
- Pool selection (YES/NO)
- Amount input
- LP token display
- Pool share calculation
- Add/Remove buttons with transaction execution

## Target Files
- [ ] `app/components/LiquidityPanel.tsx`

## Implementation Steps (TDD: Red-Green-Refactor)

### 1. Red Phase
- [ ] Define LiquidityPanel props
- [ ] Create component structure

### 2. Green Phase
- [ ] Implement liquidity panel component:

```typescript
// app/components/LiquidityPanel.tsx
"use client";

import { useState, useCallback } from "react";
import { useWalletConnection } from "@solana/react-hooks";
import { useLiquidity } from "@/hooks/useLiquidity";
import { formatTokenAmount } from "@/lib/formatters";
import { AmountInput } from "./AmountInput";

interface LiquidityPanelProps {
  marketId: bigint;
  yesPool: {
    usdcReserve: bigint;
    tokenReserve: bigint;
    totalLpSupply: bigint;
  };
  noPool: {
    usdcReserve: bigint;
    tokenReserve: bigint;
    totalLpSupply: bigint;
  };
  userLpBalance?: {
    yes: bigint;
    no: bigint;
  };
  disabled?: boolean;
  onComplete?: () => void;
}

type Mode = "add" | "remove";
type Side = "yes" | "no";

export function LiquidityPanel({
  marketId,
  yesPool,
  noPool,
  userLpBalance = { yes: 0n, no: 0n },
  disabled = false,
  onComplete,
}: LiquidityPanelProps) {
  const { status } = useWalletConnection();
  const { addLiquidity, removeLiquidity, isLoading, error } = useLiquidity();

  const [mode, setMode] = useState<Mode>("add");
  const [side, setSide] = useState<Side>("yes");
  const [amount, setAmount] = useState<string>("");

  const pool = side === "yes" ? yesPool : noPool;
  const lpBalance = side === "yes" ? userLpBalance.yes : userLpBalance.no;

  // Parse amount to bigint (USDC for add, LP tokens for remove)
  const amountBigInt = amount ? BigInt(Math.floor(parseFloat(amount) * 1_000_000)) : 0n;

  // Calculate LP tokens to receive (for add) or USDC to receive (for remove)
  const calculateLpTokens = (usdcAmount: bigint): bigint => {
    if (pool.totalLpSupply === 0n) {
      return usdcAmount; // Initial liquidity: 1:1
    }
    return (usdcAmount * pool.totalLpSupply) / pool.usdcReserve;
  };

  const calculateUsdcOut = (lpAmount: bigint): bigint => {
    if (pool.totalLpSupply === 0n) return 0n;
    return (lpAmount * pool.usdcReserve) / pool.totalLpSupply;
  };

  const expectedOutput = mode === "add"
    ? calculateLpTokens(amountBigInt)
    : calculateUsdcOut(amountBigInt);

  // Calculate pool share
  const newPoolShare = mode === "add" && pool.totalLpSupply > 0n
    ? Number(calculateLpTokens(amountBigInt)) / (Number(pool.totalLpSupply) + Number(calculateLpTokens(amountBigInt))) * 100
    : 0;

  const handleSubmit = useCallback(async () => {
    if (!amountBigInt || amountBigInt <= 0n) return;

    let success: boolean;

    if (mode === "add") {
      success = await addLiquidity({
        marketId,
        side: side === "yes" ? { Yes: {} } : { No: {} },
        usdcAmount: amountBigInt,
      });
    } else {
      success = await removeLiquidity({
        marketId,
        side: side === "yes" ? { Yes: {} } : { No: {} },
        lpAmount: amountBigInt,
      });
    }

    if (success) {
      setAmount("");
      onComplete?.();
    }
  }, [mode, addLiquidity, removeLiquidity, marketId, side, amountBigInt, onComplete]);

  const isConnected = status === "connected";
  const canSubmit = isConnected && amountBigInt > 0n && !isLoading && !disabled;
  const insufficientLp = mode === "remove" && amountBigInt > lpBalance;

  return (
    <div className="rounded-2xl border border-border-low bg-card p-5">
      <h3 className="text-lg font-semibold mb-4">Liquidity</h3>

      {/* Mode Selection */}
      <div className="flex rounded-lg bg-gray-100 p-1 mb-4">
        <button
          onClick={() => setMode("add")}
          className={`flex-1 rounded-md py-2 text-sm font-medium transition-colors ${
            mode === "add"
              ? "bg-white shadow text-gray-900"
              : "text-gray-600 hover:text-gray-900"
          }`}
        >
          Add
        </button>
        <button
          onClick={() => setMode("remove")}
          className={`flex-1 rounded-md py-2 text-sm font-medium transition-colors ${
            mode === "remove"
              ? "bg-white shadow text-gray-900"
              : "text-gray-600 hover:text-gray-900"
          }`}
        >
          Remove
        </button>
      </div>

      {/* Pool Selection */}
      <div className="mb-4">
        <label className="block text-sm font-medium text-muted mb-2">
          Pool
        </label>
        <div className="flex gap-2">
          <button
            onClick={() => setSide("yes")}
            className={`flex-1 rounded-lg border py-2 text-sm font-medium transition-colors ${
              side === "yes"
                ? "border-green-500 bg-green-50 text-green-700"
                : "border-gray-200 hover:border-gray-300"
            }`}
          >
            YES Pool
          </button>
          <button
            onClick={() => setSide("no")}
            className={`flex-1 rounded-lg border py-2 text-sm font-medium transition-colors ${
              side === "no"
                ? "border-red-500 bg-red-50 text-red-700"
                : "border-gray-200 hover:border-gray-300"
            }`}
          >
            NO Pool
          </button>
        </div>
      </div>

      {/* Amount Input */}
      <div className="mb-4">
        <div className="flex justify-between items-center mb-2">
          <label className="text-sm font-medium text-muted">
            {mode === "add" ? "USDC Amount" : "LP Token Amount"}
          </label>
          {mode === "remove" && (
            <button
              onClick={() => setAmount((Number(lpBalance) / 1_000_000).toString())}
              className="text-xs text-primary hover:underline"
            >
              Max: {formatTokenAmount(lpBalance)}
            </button>
          )}
        </div>
        <AmountInput
          value={amount}
          onChange={setAmount}
          disabled={disabled || isLoading}
          placeholder="0.00"
        />
        {insufficientLp && (
          <p className="text-xs text-red-500 mt-1">Insufficient LP token balance</p>
        )}
      </div>

      {/* Summary */}
      {amountBigInt > 0n && (
        <div className="space-y-2 mb-4 p-3 bg-gray-50 rounded-lg text-sm">
          <div className="flex justify-between">
            <span className="text-muted">
              {mode === "add" ? "LP Tokens to Receive" : "USDC to Receive"}
            </span>
            <span>{formatTokenAmount(expectedOutput)}</span>
          </div>
          {mode === "add" && (
            <div className="flex justify-between">
              <span className="text-muted">Pool Share</span>
              <span>{newPoolShare.toFixed(2)}%</span>
            </div>
          )}
          <div className="flex justify-between">
            <span className="text-muted">Current Pool Reserves</span>
            <span>${formatTokenAmount(pool.usdcReserve)}</span>
          </div>
        </div>
      )}

      {/* Error Display */}
      {error && (
        <div className="mb-4 p-3 bg-red-50 border border-red-200 rounded-lg text-sm text-red-600">
          {error.message}
        </div>
      )}

      {/* Submit Button */}
      <button
        onClick={handleSubmit}
        disabled={!canSubmit || insufficientLp}
        className={`w-full rounded-lg py-3 text-sm font-medium transition-colors ${
          canSubmit && !insufficientLp
            ? "bg-primary text-white hover:bg-primary/90"
            : "bg-gray-200 text-gray-500 cursor-not-allowed"
        }`}
      >
        {!isConnected
          ? "Connect Wallet"
          : isLoading
          ? mode === "add" ? "Adding..." : "Removing..."
          : disabled
          ? "Market Closed"
          : mode === "add"
          ? "Add Liquidity"
          : "Remove Liquidity"}
      </button>
    </div>
  );
}
```

### 3. Refactor Phase
- [ ] Add loading states
- [ ] Improve error handling
- [ ] Add confirmation step for large removals

## Completion Criteria
- [ ] Add/Remove mode toggle works
- [ ] Pool selection works
- [ ] LP token calculations correct
- [ ] Transactions execute properly
- [ ] Build passes
- [ ] Operation verified: L1 (functional component)

## Notes
- Impact scope: Used in Market Detail page
- Constraints: Requires wallet connection
- Should show user's LP balance for remove mode
