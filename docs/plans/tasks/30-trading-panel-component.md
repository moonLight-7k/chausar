# Task: TradingPanel Component

Metadata:
- Dependencies: task-20 (useTrade), task-33 (AmountInput)
- Provides: app/components/TradingPanel.tsx
- Size: Small (1-2 files)

## Implementation Content
Create the trading interface for buying YES/NO tokens:
- YES/NO side selection tabs
- Amount input in USDC
- Price impact display
- Expected tokens output
- Slippage tolerance setting
- Buy button with transaction execution

## Target Files
- [ ] `app/components/TradingPanel.tsx`

## Implementation Steps (TDD: Red-Green-Refactor)

### 1. Red Phase
- [ ] Define TradingPanel props
- [ ] Create component structure with tabs

### 2. Green Phase
- [ ] Implement trading panel component:

```typescript
// app/components/TradingPanel.tsx
"use client";

import { useState, useCallback } from "react";
import { useWalletConnection } from "@solana/react-hooks";
import { useTrade } from "@/hooks/useTrade";
import { calculateTokensOut, calculatePriceImpact } from "@/lib/amm";
import { formatTokenAmount } from "@/lib/formatters";
import { AmountInput } from "./AmountInput";
import { SlippageSettings } from "./SlippageSettings";

interface TradingPanelProps {
  marketId: bigint;
  yesPool: {
    usdcReserve: bigint;
    tokenReserve: bigint;
  };
  noPool: {
    usdcReserve: bigint;
    tokenReserve: bigint;
  };
  disabled?: boolean;
  onTradeComplete?: () => void;
}

type Side = "yes" | "no";

export function TradingPanel({
  marketId,
  yesPool,
  noPool,
  disabled = false,
  onTradeComplete,
}: TradingPanelProps) {
  const { status } = useWalletConnection();
  const { trade, isLoading, error } = useTrade();

  const [side, setSide] = useState<Side>("yes");
  const [amount, setAmount] = useState<string>("");
  const [slippage, setSlippage] = useState<number>(5); // 5% default

  const pool = side === "yes" ? yesPool : noPool;
  const amountBigInt = amount ? BigInt(Math.floor(parseFloat(amount) * 1_000_000)) : 0n;

  // Calculate expected output
  const expectedTokens = amountBigInt > 0
    ? calculateTokensOut(amountBigInt, pool.usdcReserve, pool.tokenReserve)
    : 0n;

  // Calculate price impact
  const priceImpact = amountBigInt > 0
    ? calculatePriceImpact(amountBigInt, pool.usdcReserve, pool.tokenReserve)
    : 0;

  // Calculate minimum output with slippage
  const minAmountOut = expectedTokens > 0
    ? expectedTokens - (expectedTokens * BigInt(slippage * 10) / 1000n)
    : 0n;

  // Current price
  const currentPrice = Number(pool.usdcReserve) / (Number(pool.usdcReserve) + Number(pool.tokenReserve));

  const handleTrade = useCallback(async () => {
    if (!amountBigInt || amountBigInt <= 0n) return;

    const success = await trade({
      marketId,
      side: side === "yes" ? { Yes: {} } : { No: {} },
      amountIn: amountBigInt,
      minAmountOut,
    });

    if (success) {
      setAmount("");
      onTradeComplete?.();
    }
  }, [trade, marketId, side, amountBigInt, minAmountOut, onTradeComplete]);

  const isConnected = status === "connected";
  const canTrade = isConnected && amountBigInt > 0n && !isLoading && !disabled;

  return (
    <div className="rounded-2xl border border-border-low bg-card p-5">
      <h3 className="text-lg font-semibold mb-4">Trade</h3>

      {/* Side Selection Tabs */}
      <div className="flex rounded-lg bg-gray-100 p-1 mb-4">
        <button
          onClick={() => setSide("yes")}
          className={`flex-1 rounded-md py-2 text-sm font-medium transition-colors ${
            side === "yes"
              ? "bg-green-500 text-white"
              : "text-gray-600 hover:text-gray-900"
          }`}
        >
          Buy YES
        </button>
        <button
          onClick={() => setSide("no")}
          className={`flex-1 rounded-md py-2 text-sm font-medium transition-colors ${
            side === "no"
              ? "bg-red-500 text-white"
              : "text-gray-600 hover:text-gray-900"
          }`}
        >
          Buy NO
        </button>
      </div>

      {/* Amount Input */}
      <div className="mb-4">
        <label className="block text-sm font-medium text-muted mb-2">
          Amount (USDC)
        </label>
        <AmountInput
          value={amount}
          onChange={setAmount}
          disabled={disabled || isLoading}
          placeholder="0.00"
        />
      </div>

      {/* Trade Summary */}
      {amountBigInt > 0n && (
        <div className="space-y-2 mb-4 p-3 bg-gray-50 rounded-lg text-sm">
          <div className="flex justify-between">
            <span className="text-muted">Current Price</span>
            <span>{(currentPrice * 100).toFixed(1)}%</span>
          </div>
          <div className="flex justify-between">
            <span className="text-muted">Expected {side.toUpperCase()} Tokens</span>
            <span>{formatTokenAmount(expectedTokens)}</span>
          </div>
          <div className="flex justify-between">
            <span className="text-muted">Price Impact</span>
            <span className={priceImpact > 5 ? "text-red-500" : ""}>
              {priceImpact.toFixed(2)}%
            </span>
          </div>
          <div className="flex justify-between">
            <span className="text-muted">Minimum Received</span>
            <span>{formatTokenAmount(minAmountOut)}</span>
          </div>
        </div>
      )}

      {/* Slippage Settings */}
      <SlippageSettings
        value={slippage}
        onChange={setSlippage}
        className="mb-4"
      />

      {/* Error Display */}
      {error && (
        <div className="mb-4 p-3 bg-red-50 border border-red-200 rounded-lg text-sm text-red-600">
          {error.message}
        </div>
      )}

      {/* Trade Button */}
      <button
        onClick={handleTrade}
        disabled={!canTrade}
        className={`w-full rounded-lg py-3 text-sm font-medium transition-colors ${
          canTrade
            ? side === "yes"
              ? "bg-green-500 text-white hover:bg-green-600"
              : "bg-red-500 text-white hover:bg-red-600"
            : "bg-gray-200 text-gray-500 cursor-not-allowed"
        }`}
      >
        {!isConnected
          ? "Connect Wallet"
          : isLoading
          ? "Trading..."
          : disabled
          ? "Market Closed"
          : `Buy ${side.toUpperCase()}`}
      </button>
    </div>
  );
}
```

### 3. Refactor Phase
- [ ] Add loading skeleton states
- [ ] Improve error messaging
- [ ] Add price impact warning for high slippage

## Completion Criteria
- [ ] Side selection works
- [ ] Amount input updates calculations
- [ ] Price impact calculated correctly
- [ ] Trade transaction executes
- [ ] Build passes
- [ ] Operation verified: L1 (functional component)

## Notes
- Impact scope: Used in Market Detail page
- Constraints: Requires wallet connection for trading
- Should show price impact warnings for > 5%
