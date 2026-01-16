# Task: Market Detail Page

Metadata:
- Dependencies: task-19 (useMarket), task-29 (TradingPanel), task-30 (LiquidityPanel)
- Provides: app/markets/[id]/page.tsx
- Size: Small (1 file)

## Implementation Content
Create the market detail page with:
- Full market information display
- Trading panel (YES/NO)
- Liquidity panel (add/remove)
- Price display and status indicator

## Target Files
- [ ] `app/markets/[id]/page.tsx`

## Implementation Steps (TDD: Red-Green-Refactor)

### 1. Red Phase
- [ ] Create dynamic route structure
- [ ] Define page layout

### 2. Green Phase
- [ ] Implement market detail page:

```typescript
// app/markets/[id]/page.tsx
"use client";

import { use } from "react";
import { useMarket } from "@/hooks";
import { TradingPanel } from "@/components/TradingPanel";
import { LiquidityPanel } from "@/components/LiquidityPanel";
import { MarketStatus } from "@/components/MarketStatus";
import { PriceDisplay } from "@/components/PriceDisplay";
import { formatDate, formatTimeRemaining, formatTokenAmount } from "@/lib/formatters";
import { calculateYesPrice, calculateNoPrice } from "@/lib/amm";

interface PageProps {
  params: Promise<{ id: string }>;
}

export default function MarketDetailPage({ params }: PageProps) {
  const { id } = use(params);
  const marketId = BigInt(id);
  const { data, isLoading, error, refetch } = useMarket(marketId);

  if (isLoading) {
    return (
      <div className="min-h-screen bg-bg1 px-6 py-16">
        <div className="mx-auto max-w-4xl">
          <p className="text-muted">Loading market...</p>
        </div>
      </div>
    );
  }

  if (error || !data) {
    return (
      <div className="min-h-screen bg-bg1 px-6 py-16">
        <div className="mx-auto max-w-4xl">
          <p className="text-red-500">
            {error?.message || "Market not found"}
          </p>
        </div>
      </div>
    );
  }

  const { market, yesPool, noPool } = data;
  const yesPrice = calculateYesPrice(yesPool.usdcReserve, noPool.usdcReserve);
  const noPrice = calculateNoPrice(yesPool.usdcReserve, noPool.usdcReserve);

  return (
    <div className="min-h-screen bg-bg1 px-6 py-16">
      <div className="mx-auto max-w-4xl">
        {/* Header */}
        <header className="mb-8">
          <div className="flex items-center gap-3 mb-4">
            <MarketStatus status={market.status} />
            {market.status === "Resolved" && (
              <span className="rounded-full bg-green-100 px-3 py-1 text-xs font-semibold text-green-800">
                Result: {market.result}
              </span>
            )}
          </div>

          <h1 className="text-2xl font-semibold tracking-tight mb-2">
            {market.question}
          </h1>

          {market.description && (
            <p className="text-muted mb-4">{market.description}</p>
          )}

          <div className="flex flex-wrap gap-4 text-sm text-muted">
            <span>Ends: {formatDate(market.endTime)}</span>
            <span>Time left: {formatTimeRemaining(market.endTime)}</span>
            <span>
              Liquidity: ${formatTokenAmount(market.totalLiquidity)}
            </span>
          </div>
        </header>

        {/* Prices */}
        <section className="mb-8">
          <PriceDisplay yesPrice={yesPrice} noPrice={noPrice} />
        </section>

        {/* Main content grid */}
        <div className="grid gap-8 lg:grid-cols-2">
          {/* Trading Panel */}
          <section className="rounded-2xl border border-border-low bg-card p-6">
            <h2 className="text-lg font-semibold mb-4">Trade</h2>
            {market.status === "Open" ? (
              <TradingPanel
                marketId={market.id}
                yesPool={yesPool}
                noPool={noPool}
                onSuccess={refetch}
              />
            ) : (
              <p className="text-muted">
                Trading is {market.status === "Locked" ? "locked" : "closed"}
              </p>
            )}
          </section>

          {/* Liquidity Panel */}
          <section className="rounded-2xl border border-border-low bg-card p-6">
            <h2 className="text-lg font-semibold mb-4">Liquidity</h2>
            {market.status === "Open" ? (
              <LiquidityPanel
                marketId={market.id}
                yesPool={yesPool}
                noPool={noPool}
                onSuccess={refetch}
              />
            ) : (
              <p className="text-muted">Liquidity management disabled</p>
            )}
          </section>
        </div>

        {/* Pool Stats */}
        <section className="mt-8 rounded-2xl border border-border-low bg-card p-6">
          <h2 className="text-lg font-semibold mb-4">Pool Statistics</h2>
          <div className="grid gap-4 sm:grid-cols-2">
            <div>
              <h3 className="font-medium text-green-600">YES Pool</h3>
              <p className="text-sm text-muted">
                USDC: ${formatTokenAmount(yesPool.usdcReserve)}
              </p>
              <p className="text-sm text-muted">
                Tokens: {formatTokenAmount(yesPool.tokenReserve)}
              </p>
            </div>
            <div>
              <h3 className="font-medium text-red-600">NO Pool</h3>
              <p className="text-sm text-muted">
                USDC: ${formatTokenAmount(noPool.usdcReserve)}
              </p>
              <p className="text-sm text-muted">
                Tokens: {formatTokenAmount(noPool.tokenReserve)}
              </p>
            </div>
          </div>
        </section>
      </div>
    </div>
  );
}
```

### 3. Refactor Phase
- [ ] Add error boundary
- [ ] Improve layout responsiveness
- [ ] Add claim button for resolved markets

## Completion Criteria
- [ ] Page displays market info
- [ ] Trading panel visible for open markets
- [ ] Liquidity panel visible for open markets
- [ ] Build passes
- [ ] Operation verified: L1 (functional page)

## Notes
- Dynamic route with [id] parameter
- Shows different content based on market status
- Depends on TradingPanel and LiquidityPanel components
