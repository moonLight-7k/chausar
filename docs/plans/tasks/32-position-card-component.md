# Task: PositionCard Component

Metadata:
- Dependencies: task-28 (MarketStatus), task-18 (formatters)
- Provides: app/components/PositionCard.tsx
- Size: Small (1 file)

## Implementation Content
Create a position display card showing:
- Market question
- Position side (YES/NO)
- Token balance
- Current value estimate
- Market status
- Profit/loss indicator

## Target Files
- [ ] `app/components/PositionCard.tsx`

## Implementation Steps (TDD: Red-Green-Refactor)

### 1. Red Phase
- [ ] Define Position interface
- [ ] Create component structure

### 2. Green Phase
- [ ] Implement position card component:

```typescript
// app/components/PositionCard.tsx
import Link from "next/link";
import { MarketStatus } from "./MarketStatus";
import { formatTokenAmount } from "@/lib/formatters";

interface Position {
  marketId: bigint;
  question: string;
  side: "yes" | "no";
  balance: bigint;
  currentPrice: number; // 0-1 decimal
  avgEntryPrice?: number; // Optional for P/L calculation
  status: { Open: {} } | { Locked: {} } | { Resolved: {} };
}

interface PositionCardProps {
  position: Position;
}

export function PositionCard({ position }: PositionCardProps) {
  const { marketId, question, side, balance, currentPrice, avgEntryPrice, status } = position;

  // Calculate current value (balance * price)
  const currentValue = Number(balance) / 1_000_000 * currentPrice;

  // Calculate P/L if entry price available
  const pnl = avgEntryPrice !== undefined
    ? ((currentPrice - avgEntryPrice) / avgEntryPrice) * 100
    : undefined;

  const sideConfig = {
    yes: {
      label: "YES",
      bgColor: "bg-green-50",
      borderColor: "border-green-200",
      textColor: "text-green-700",
      accentColor: "text-green-600",
    },
    no: {
      label: "NO",
      bgColor: "bg-red-50",
      borderColor: "border-red-200",
      textColor: "text-red-700",
      accentColor: "text-red-600",
    },
  };

  const config = sideConfig[side];

  return (
    <Link
      href={`/markets/${marketId.toString()}`}
      className={`block rounded-2xl border ${config.borderColor} ${config.bgColor} p-4 transition-all hover:shadow-md`}
    >
      <div className="flex items-start justify-between gap-4 mb-3">
        <div className="flex-1 min-w-0">
          <p className="text-sm font-medium truncate">{question}</p>
        </div>
        <MarketStatus status={status} size="sm" />
      </div>

      <div className="flex items-center justify-between">
        <div className="flex items-center gap-3">
          {/* Position Badge */}
          <span className={`inline-flex items-center rounded-md px-2 py-1 text-xs font-bold ${config.textColor} bg-white border ${config.borderColor}`}>
            {config.label}
          </span>

          {/* Balance */}
          <div>
            <p className="text-sm font-medium">
              {formatTokenAmount(balance)} tokens
            </p>
            <p className="text-xs text-muted">
              Current price: {(currentPrice * 100).toFixed(1)}%
            </p>
          </div>
        </div>

        {/* Value and P/L */}
        <div className="text-right">
          <p className="text-sm font-semibold">
            ${currentValue.toFixed(2)}
          </p>
          {pnl !== undefined && (
            <p className={`text-xs font-medium ${
              pnl >= 0 ? "text-green-600" : "text-red-600"
            }`}>
              {pnl >= 0 ? "+" : ""}{pnl.toFixed(1)}%
            </p>
          )}
        </div>
      </div>
    </Link>
  );
}

// Compact variant for lists
interface PositionRowProps {
  position: Position;
}

export function PositionRow({ position }: PositionRowProps) {
  const { marketId, question, side, balance, currentPrice } = position;
  const currentValue = Number(balance) / 1_000_000 * currentPrice;

  return (
    <Link
      href={`/markets/${marketId.toString()}`}
      className="flex items-center justify-between py-3 px-4 hover:bg-gray-50 rounded-lg transition-colors"
    >
      <div className="flex items-center gap-3 flex-1 min-w-0">
        <span className={`inline-flex items-center justify-center w-8 h-8 rounded-full text-xs font-bold ${
          side === "yes"
            ? "bg-green-100 text-green-700"
            : "bg-red-100 text-red-700"
        }`}>
          {side === "yes" ? "Y" : "N"}
        </span>
        <p className="text-sm truncate">{question}</p>
      </div>

      <div className="flex items-center gap-6 flex-shrink-0">
        <div className="text-right">
          <p className="text-sm font-medium">{formatTokenAmount(balance)}</p>
          <p className="text-xs text-muted">{(currentPrice * 100).toFixed(0)}%</p>
        </div>
        <p className="text-sm font-semibold w-20 text-right">
          ${currentValue.toFixed(2)}
        </p>
      </div>
    </Link>
  );
}
```

### 3. Refactor Phase
- [ ] Add hover effects
- [ ] Improve value formatting
- [ ] Add loading skeleton variant

## Completion Criteria
- [ ] Displays position information correctly
- [ ] Color coding for YES/NO
- [ ] Shows current value
- [ ] Links to market detail
- [ ] Build passes
- [ ] Operation verified: L1 (functional component)

## Notes
- Impact scope: Used in Portfolio page
- Constraints: Must handle various market statuses
- Should be visually distinct for YES vs NO positions
