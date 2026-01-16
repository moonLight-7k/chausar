# Task: MarketCard Component

Metadata:
- Dependencies: task-28 (MarketStatus), task-18 (formatters)
- Provides: app/components/MarketCard.tsx
- Size: Small (1 file)

## Implementation Content
Create a market summary card for the market list view showing:
- Question text
- YES/NO prices as bars
- Time remaining
- Total liquidity
- Status badge

## Target Files
- [ ] `app/components/MarketCard.tsx`

## Implementation Steps (TDD: Red-Green-Refactor)

### 1. Red Phase
- [ ] Define MarketCard props interface
- [ ] Create component structure

### 2. Green Phase
- [ ] Implement market card component:

```typescript
// app/components/MarketCard.tsx
import Link from "next/link";
import { MarketStatus, MarketResult } from "./MarketStatus";
import { formatTokenAmount, formatTimeRemaining } from "@/lib/formatters";

interface MarketCardProps {
  market: {
    id: bigint;
    question: string;
    endTime: bigint;
    status: { Open: {} } | { Locked: {} } | { Resolved: {} };
    result: { Undecided: {} } | { Yes: {} } | { No: {} };
    totalLiquidity: bigint;
  };
  yesPrice: number; // 0-1 decimal
  noPrice: number;  // 0-1 decimal
}

export function MarketCard({ market, yesPrice, noPrice }: MarketCardProps) {
  const statusKey = Object.keys(market.status)[0] as "Open" | "Locked" | "Resolved";
  const resultKey = Object.keys(market.result)[0] as "Undecided" | "Yes" | "No";
  const endTime = Number(market.endTime) * 1000; // Convert to milliseconds
  const now = Date.now();
  const isExpired = now >= endTime;

  return (
    <Link
      href={`/markets/${market.id.toString()}`}
      className="block rounded-2xl border border-border-low bg-card p-5 transition-all hover:border-border-high hover:shadow-md"
    >
      {/* Header */}
      <div className="flex items-start justify-between gap-4 mb-4">
        <h3 className="text-lg font-medium leading-snug line-clamp-2">
          {market.question}
        </h3>
        <div className="flex-shrink-0">
          <MarketStatus status={market.status} size="sm" />
        </div>
      </div>

      {/* Price Bars */}
      <div className="space-y-2 mb-4">
        <div className="flex items-center gap-2">
          <span className="w-8 text-sm font-medium text-green-600">YES</span>
          <div className="flex-1 h-6 bg-gray-100 rounded-full overflow-hidden">
            <div
              className="h-full bg-green-500 rounded-full transition-all"
              style={{ width: `${yesPrice * 100}%` }}
            />
          </div>
          <span className="w-12 text-sm font-medium text-right">
            {(yesPrice * 100).toFixed(0)}%
          </span>
        </div>
        <div className="flex items-center gap-2">
          <span className="w-8 text-sm font-medium text-red-600">NO</span>
          <div className="flex-1 h-6 bg-gray-100 rounded-full overflow-hidden">
            <div
              className="h-full bg-red-500 rounded-full transition-all"
              style={{ width: `${noPrice * 100}%` }}
            />
          </div>
          <span className="w-12 text-sm font-medium text-right">
            {(noPrice * 100).toFixed(0)}%
          </span>
        </div>
      </div>

      {/* Footer Stats */}
      <div className="flex items-center justify-between text-sm text-muted">
        <div className="flex items-center gap-4">
          <span>
            Liquidity: ${formatTokenAmount(market.totalLiquidity)}
          </span>
        </div>
        <div>
          {statusKey === "Resolved" ? (
            <span className="flex items-center gap-1">
              Result: <MarketResult result={market.result} size="sm" />
            </span>
          ) : isExpired ? (
            <span className="text-yellow-600">Awaiting resolution</span>
          ) : (
            <span>{formatTimeRemaining(endTime)}</span>
          )}
        </div>
      </div>
    </Link>
  );
}
```

### 3. Refactor Phase
- [ ] Improve visual hierarchy
- [ ] Add hover animations
- [ ] Ensure accessibility (keyboard nav, screen readers)

## Completion Criteria
- [ ] Displays market question
- [ ] Shows YES/NO price bars correctly
- [ ] Shows time remaining or resolution status
- [ ] Links to market detail page
- [ ] Build passes
- [ ] Operation verified: L1 (functional component)

## Notes
- Impact scope: Used in Market List page
- Constraints: Must handle all market status states
- Price bars should visually represent probability
