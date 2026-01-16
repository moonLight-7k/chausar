# Task: useMarkets and useMarket Hooks

Metadata:
- Dependencies: task-15 (client), task-16 (config)
- Provides: app/hooks/useMarkets.ts, app/hooks/useMarket.ts
- Size: Small (2 files)

## Implementation Content
Create React hooks for fetching market data:
- useMarkets: Fetch all markets
- useMarket: Fetch single market by ID

## Target Files
- [ ] `app/hooks/useMarkets.ts`
- [ ] `app/hooks/useMarket.ts`
- [ ] `app/hooks/index.ts`

## Implementation Steps (TDD: Red-Green-Refactor)

### 1. Red Phase
- [ ] Define hook interfaces
- [ ] Identify required RPC calls

### 2. Green Phase
- [ ] Implement useMarkets hook:

```typescript
// app/hooks/useMarkets.ts
"use client";

import { useEffect, useState } from "react";
import { useSolanaClient } from "@solana/react-hooks";
import { fetchAllMarketAccounts } from "@/generated/prediction";
import type { Market } from "@/generated/prediction";

interface UseMarketsResult {
  markets: Market[];
  isLoading: boolean;
  error: Error | null;
  refetch: () => void;
}

export function useMarkets(): UseMarketsResult {
  const client = useSolanaClient();
  const [markets, setMarkets] = useState<Market[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);

  const fetchMarkets = async () => {
    setIsLoading(true);
    setError(null);
    try {
      const accounts = await fetchAllMarketAccounts(client.rpc);
      setMarkets(accounts.map(a => a.data));
    } catch (e) {
      setError(e instanceof Error ? e : new Error("Failed to fetch markets"));
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    fetchMarkets();
  }, [client]);

  return { markets, isLoading, error, refetch: fetchMarkets };
}
```

- [ ] Implement useMarket hook:

```typescript
// app/hooks/useMarket.ts
"use client";

import { useEffect, useState } from "react";
import { useSolanaClient } from "@solana/react-hooks";
import { fetchMarketAccount, fetchPoolAccount } from "@/generated/prediction";
import type { Market, Pool } from "@/generated/prediction";
import { CONFIG } from "@/lib/config";
import { getMarketPda, getPoolPda } from "@/lib/pda";

interface MarketData {
  market: Market;
  yesPool: Pool;
  noPool: Pool;
}

interface UseMarketResult {
  data: MarketData | null;
  isLoading: boolean;
  error: Error | null;
  refetch: () => void;
}

export function useMarket(marketId: bigint): UseMarketResult {
  const client = useSolanaClient();
  const [data, setData] = useState<MarketData | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);

  const fetchMarket = async () => {
    setIsLoading(true);
    setError(null);
    try {
      const marketPda = getMarketPda(marketId);
      const market = await fetchMarketAccount(client.rpc, marketPda);

      const yesPoolPda = getPoolPda(marketPda, "yes");
      const noPoolPda = getPoolPda(marketPda, "no");

      const [yesPool, noPool] = await Promise.all([
        fetchPoolAccount(client.rpc, yesPoolPda),
        fetchPoolAccount(client.rpc, noPoolPda),
      ]);

      setData({
        market: market.data,
        yesPool: yesPool.data,
        noPool: noPool.data,
      });
    } catch (e) {
      setError(e instanceof Error ? e : new Error("Failed to fetch market"));
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    fetchMarket();
  }, [client, marketId]);

  return { data, isLoading, error, refetch: fetchMarket };
}
```

- [ ] Create hooks index:

```typescript
// app/hooks/index.ts
export { useMarkets } from "./useMarkets";
export { useMarket } from "./useMarket";
```

### 3. Refactor Phase
- [ ] Add PDA helper utilities if needed
- [ ] Add caching or SWR if desired
- [ ] Verify hooks work with client

## Completion Criteria
- [ ] Both hooks implemented
- [ ] Handles loading and error states
- [ ] Build passes
- [ ] Operation verified: L3 (build succeeds)

## Notes
- Uses generated client from Codama
- Fetches pool data alongside market
- Supports refetching for updates
