# Task: Create Market Page

Metadata:
- Dependencies: task-16 (config), task-31 (CreateMarketForm)
- Provides: app/create/page.tsx
- Size: Small (1 file)

## Implementation Content
Create the market creation page with:
- Market creation form
- Initial liquidity input
- Oracle address input
- Form validation and submission

## Target Files
- [ ] `app/create/page.tsx`

## Implementation Steps (TDD: Red-Green-Refactor)

### 1. Red Phase
- [ ] Create page structure
- [ ] Define form requirements

### 2. Green Phase
- [ ] Implement create market page:

```typescript
// app/create/page.tsx
"use client";

import { useRouter } from "next/navigation";
import { useWalletConnection } from "@solana/react-hooks";
import { CreateMarketForm } from "@/components/CreateMarketForm";

export default function CreateMarketPage() {
  const router = useRouter();
  const { wallet, status } = useWalletConnection();

  if (status !== "connected") {
    return (
      <div className="min-h-screen bg-bg1 px-6 py-16">
        <div className="mx-auto max-w-2xl">
          <header className="mb-8">
            <h1 className="text-3xl font-semibold tracking-tight mb-2">
              Create Market
            </h1>
            <p className="text-muted">
              Create a new prediction market
            </p>
          </header>

          <div className="rounded-2xl border border-border-low bg-card p-6 text-center">
            <p className="text-muted mb-4">
              Connect your wallet to create a market
            </p>
          </div>
        </div>
      </div>
    );
  }

  const handleSuccess = (marketId: bigint) => {
    router.push(`/markets/${marketId.toString()}`);
  };

  return (
    <div className="min-h-screen bg-bg1 px-6 py-16">
      <div className="mx-auto max-w-2xl">
        <header className="mb-8">
          <h1 className="text-3xl font-semibold tracking-tight mb-2">
            Create Market
          </h1>
          <p className="text-muted">
            Create a new prediction market with initial liquidity
          </p>
        </header>

        <div className="rounded-2xl border border-border-low bg-card p-6">
          <CreateMarketForm
            creatorAddress={wallet.account.address.toString()}
            onSuccess={handleSuccess}
          />
        </div>

        {/* Requirements */}
        <div className="mt-6 rounded-xl bg-cream p-4">
          <h3 className="font-medium mb-2">Requirements</h3>
          <ul className="text-sm text-muted space-y-1">
            <li>Minimum initial liquidity: 100 USDC</li>
            <li>End time must be in the future</li>
            <li>Resolve time must be after end time</li>
            <li>Question max 280 characters</li>
            <li>Description max 1000 characters</li>
          </ul>
        </div>
      </div>
    </div>
  );
}
```

### 3. Refactor Phase
- [ ] Add USDC balance check
- [ ] Improve form validation feedback
- [ ] Add loading state during creation

## Completion Criteria
- [ ] Page renders form when connected
- [ ] Shows connect prompt when disconnected
- [ ] Redirects to market on success
- [ ] Build passes
- [ ] Operation verified: L1 (functional page)

## Notes
- Requires wallet connection
- Minimum 100 USDC liquidity
- Uses CreateMarketForm component
