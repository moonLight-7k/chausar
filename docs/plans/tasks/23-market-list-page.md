# Task: Market List Page

Metadata:
- Dependencies: task-19 (useMarkets hook), task-28 (MarketCard component)
- Provides: app/markets/page.tsx
- Size: Small (1 file)

## Implementation Content
Create the market list page displaying all markets:
- Grid of market cards
- Sorting options (ending soon, volume, newest)
- Basic search/filter functionality
- Loading and empty states

## Target Files
- [ ] `app/markets/page.tsx`
- [ ] `app/markets/layout.tsx` (optional)

## Implementation Steps (TDD: Red-Green-Refactor)

### 1. Red Phase
- [ ] Create page file structure
- [ ] Define component interface

### 2. Green Phase
- [ ] Implement market list page:

```typescript
// app/markets/page.tsx
"use client";

import { useState, useMemo } from "react";
import { useMarkets } from "@/hooks";
import { MarketCard } from "@/components/MarketCard";
import { MarketStatus } from "@/generated/prediction";

type SortOption = "ending" | "volume" | "newest";

export default function MarketsPage() {
  const { markets, isLoading, error } = useMarkets();
  const [sortBy, setSortBy] = useState<SortOption>("ending");
  const [search, setSearch] = useState("");
  const [statusFilter, setStatusFilter] = useState<MarketStatus | "all">("all");

  const filteredMarkets = useMemo(() => {
    let result = [...markets];

    // Filter by search
    if (search) {
      result = result.filter(
        (m) =>
          m.question.toLowerCase().includes(search.toLowerCase()) ||
          m.description.toLowerCase().includes(search.toLowerCase())
      );
    }

    // Filter by status
    if (statusFilter !== "all") {
      result = result.filter((m) => m.status === statusFilter);
    }

    // Sort
    switch (sortBy) {
      case "ending":
        result.sort((a, b) => Number(a.endTime - b.endTime));
        break;
      case "volume":
        result.sort((a, b) => Number(b.totalLiquidity - a.totalLiquidity));
        break;
      case "newest":
        result.sort((a, b) => Number(b.createdAt - a.createdAt));
        break;
    }

    return result;
  }, [markets, sortBy, search, statusFilter]);

  if (isLoading) {
    return (
      <div className="min-h-screen bg-bg1 px-6 py-16">
        <div className="mx-auto max-w-6xl">
          <p className="text-muted">Loading markets...</p>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="min-h-screen bg-bg1 px-6 py-16">
        <div className="mx-auto max-w-6xl">
          <p className="text-red-500">Error: {error.message}</p>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-bg1 px-6 py-16">
      <div className="mx-auto max-w-6xl">
        <header className="mb-8">
          <h1 className="text-3xl font-semibold tracking-tight mb-2">
            Markets
          </h1>
          <p className="text-muted">
            Trade on binary prediction markets
          </p>
        </header>

        {/* Filters */}
        <div className="flex flex-wrap gap-4 mb-8">
          <input
            type="text"
            placeholder="Search markets..."
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            className="rounded-lg border border-border-low bg-card px-4 py-2 text-sm"
          />

          <select
            value={sortBy}
            onChange={(e) => setSortBy(e.target.value as SortOption)}
            className="rounded-lg border border-border-low bg-card px-4 py-2 text-sm"
          >
            <option value="ending">Ending Soon</option>
            <option value="volume">Highest Volume</option>
            <option value="newest">Newest</option>
          </select>

          <select
            value={statusFilter}
            onChange={(e) =>
              setStatusFilter(e.target.value as MarketStatus | "all")
            }
            className="rounded-lg border border-border-low bg-card px-4 py-2 text-sm"
          >
            <option value="all">All Status</option>
            <option value="Open">Open</option>
            <option value="Locked">Locked</option>
            <option value="Resolved">Resolved</option>
          </select>
        </div>

        {/* Market Grid */}
        {filteredMarkets.length === 0 ? (
          <p className="text-muted">No markets found</p>
        ) : (
          <div className="grid gap-6 sm:grid-cols-2 lg:grid-cols-3">
            {filteredMarkets.map((market) => (
              <MarketCard key={market.id.toString()} market={market} />
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
```

### 3. Refactor Phase
- [ ] Extract filter components
- [ ] Add skeleton loading states
- [ ] Improve responsive design

## Completion Criteria
- [ ] Page renders market list
- [ ] Sorting works correctly
- [ ] Search filters markets
- [ ] Build passes
- [ ] Operation verified: L1 (functional page)

## Notes
- Uses useMarkets hook for data
- Depends on MarketCard component
- Shows loading/error/empty states
