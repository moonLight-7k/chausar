# Task: Portfolio Page

Metadata:
- Dependencies: task-22 (usePortfolio), task-31 (PositionCard)
- Provides: app/portfolio/page.tsx
- Size: Small (1 file)

## Implementation Content
Create the portfolio page displaying:
- Active positions (YES/NO token balances)
- LP positions
- Claimable winnings with claim buttons

## Target Files
- [ ] `app/portfolio/page.tsx`

## Implementation Steps (TDD: Red-Green-Refactor)

### 1. Red Phase
- [ ] Create page structure
- [ ] Define portfolio sections

### 2. Green Phase
- [ ] Implement portfolio page:

```typescript
// app/portfolio/page.tsx
"use client";

import { useWalletConnection } from "@solana/react-hooks";
import { usePortfolio } from "@/hooks";
import { PositionCard } from "@/components/PositionCard";
import { formatTokenAmount } from "@/lib/formatters";
import { useClaimWinnings } from "@/hooks/useClaimWinnings";

export default function PortfolioPage() {
  const { wallet, status } = useWalletConnection();
  const { portfolio, isLoading, error, refetch } = usePortfolio();
  const { claim, isLoading: isClaiming } = useClaimWinnings();

  if (status !== "connected") {
    return (
      <div className="min-h-screen bg-bg1 px-6 py-16">
        <div className="mx-auto max-w-4xl">
          <header className="mb-8">
            <h1 className="text-3xl font-semibold tracking-tight mb-2">
              Portfolio
            </h1>
            <p className="text-muted">View your positions and winnings</p>
          </header>

          <div className="rounded-2xl border border-border-low bg-card p-6 text-center">
            <p className="text-muted">Connect your wallet to view portfolio</p>
          </div>
        </div>
      </div>
    );
  }

  if (isLoading) {
    return (
      <div className="min-h-screen bg-bg1 px-6 py-16">
        <div className="mx-auto max-w-4xl">
          <p className="text-muted">Loading portfolio...</p>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="min-h-screen bg-bg1 px-6 py-16">
        <div className="mx-auto max-w-4xl">
          <p className="text-red-500">Error: {error.message}</p>
        </div>
      </div>
    );
  }

  const handleClaim = async (marketId: bigint) => {
    await claim(marketId);
    refetch();
  };

  return (
    <div className="min-h-screen bg-bg1 px-6 py-16">
      <div className="mx-auto max-w-4xl">
        <header className="mb-8">
          <h1 className="text-3xl font-semibold tracking-tight mb-2">
            Portfolio
          </h1>
          <p className="text-muted">
            Total Value: ${formatTokenAmount(portfolio?.totalValue || 0n)}
          </p>
        </header>

        {/* Claimable Winnings */}
        {portfolio?.claimable && portfolio.claimable.length > 0 && (
          <section className="mb-8">
            <h2 className="text-xl font-semibold mb-4">Claimable Winnings</h2>
            <div className="space-y-4">
              {portfolio.claimable.map((claim) => (
                <div
                  key={claim.marketId.toString()}
                  className="flex items-center justify-between rounded-2xl border border-green-200 bg-green-50 p-4"
                >
                  <div>
                    <p className="font-medium">{claim.question}</p>
                    <p className="text-sm text-muted">
                      Payout: ${formatTokenAmount(claim.payout)}
                    </p>
                  </div>
                  <button
                    onClick={() => handleClaim(claim.marketId)}
                    disabled={isClaiming}
                    className="rounded-lg bg-green-600 px-4 py-2 text-sm font-medium text-white hover:bg-green-700 disabled:opacity-50"
                  >
                    {isClaiming ? "Claiming..." : "Claim"}
                  </button>
                </div>
              ))}
            </div>
          </section>
        )}

        {/* Active Positions */}
        <section className="mb-8">
          <h2 className="text-xl font-semibold mb-4">Active Positions</h2>
          {portfolio?.positions && portfolio.positions.length > 0 ? (
            <div className="space-y-4">
              {portfolio.positions.map((position) => (
                <PositionCard
                  key={position.marketId.toString()}
                  position={position}
                />
              ))}
            </div>
          ) : (
            <p className="text-muted">No active positions</p>
          )}
        </section>

        {/* LP Positions */}
        <section>
          <h2 className="text-xl font-semibold mb-4">LP Positions</h2>
          {portfolio?.lpPositions && portfolio.lpPositions.length > 0 ? (
            <div className="space-y-4">
              {portfolio.lpPositions.map((lp, i) => (
                <div
                  key={`${lp.marketId}-${lp.side}-${i}`}
                  className="rounded-2xl border border-border-low bg-card p-4"
                >
                  <p className="font-medium">{lp.question}</p>
                  <p className="text-sm text-muted">
                    {lp.side.toUpperCase()} Pool - Share: {lp.poolShare.toFixed(2)}%
                  </p>
                  <p className="text-sm text-muted">
                    LP Tokens: {formatTokenAmount(lp.lpBalance)}
                  </p>
                </div>
              ))}
            </div>
          ) : (
            <p className="text-muted">No LP positions</p>
          )}
        </section>
      </div>
    </div>
  );
}
```

### 3. Refactor Phase
- [ ] Add claim success/error feedback
- [ ] Add position value estimation
- [ ] Improve loading states

## Completion Criteria
- [ ] Shows positions for connected wallet
- [ ] Claim button works for winnings
- [ ] Shows LP positions
- [ ] Build passes
- [ ] Operation verified: L1 (functional page)

## Notes
- Requires wallet connection
- Displays claimable winnings prominently
- Uses PositionCard component
