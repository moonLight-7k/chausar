# Task: Provider Configuration and Program ID Setup

Metadata:
- Dependencies: task-15 (client generation)
- Provides: app/components/providers.tsx (updated), app/lib/config.ts
- Size: Small (2 files)

## Implementation Content
Update provider configuration for the prediction market:
- Configure RPC endpoint for devnet
- Add program ID configuration
- Set up USDC mint address for devnet

## Target Files
- [ ] `app/components/providers.tsx`
- [ ] `app/lib/config.ts`

## Implementation Steps (TDD: Red-Green-Refactor)

### 1. Red Phase
- [ ] Review current providers.tsx
- [ ] Identify needed configuration additions

### 2. Green Phase
- [ ] Create config.ts:

```typescript
// app/lib/config.ts
import { address } from "@solana/kit";

export const CONFIG = {
  // RPC endpoint
  rpcEndpoint: process.env.NEXT_PUBLIC_RPC_ENDPOINT || "https://api.devnet.solana.com",

  // Program ID (update after deployment)
  programId: address(process.env.NEXT_PUBLIC_PROGRAM_ID || "PredictionMarket111111111111111111111111111"),

  // USDC mint (devnet USDC or mock)
  usdcMint: address(process.env.NEXT_PUBLIC_USDC_MINT || "4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU"),

  // Default values
  defaultSlippageBps: 500, // 5%
  minLiquidity: 100_000_000, // 100 USDC (6 decimals)
  feeBps: 30, // 0.3%
} as const;
```

- [ ] Update providers.tsx:

```typescript
"use client";

import { SolanaProvider } from "@solana/react-hooks";
import { PropsWithChildren } from "react";
import { autoDiscover, createClient } from "@solana/client";
import { CONFIG } from "@/lib/config";

const client = createClient({
  endpoint: CONFIG.rpcEndpoint,
  walletConnectors: autoDiscover(),
});

export function Providers({ children }: PropsWithChildren) {
  return <SolanaProvider client={client}>{children}</SolanaProvider>;
}
```

### 3. Refactor Phase
- [ ] Add environment variable documentation
- [ ] Verify configuration loads correctly
- [ ] Run `npm run build` to verify no errors

## Completion Criteria
- [ ] Config file created with all constants
- [ ] Providers updated to use config
- [ ] Build passes
- [ ] Operation verified: L3 (build succeeds)

## Notes
- Program ID will be updated after deployment
- Use devnet USDC for testing
- Environment variables for different environments
