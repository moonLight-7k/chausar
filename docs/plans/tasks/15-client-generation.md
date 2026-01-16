# Task: Build Anchor Program and Generate TypeScript Client

Metadata:
- Dependencies: Phase 2 completion (all instructions)
- Provides: app/generated/prediction/ TypeScript client
- Size: Small (configuration + generated files)

## Implementation Content
Build the Anchor program and generate TypeScript client using Codama:
- Build Anchor program to generate IDL
- Run Codama to generate TypeScript client
- Verify generated types match expected interfaces

## Target Files
- [ ] `anchor/target/idl/prediction.json` (generated)
- [ ] `app/generated/prediction/` (generated directory)
- [ ] `codama.json` (update if needed)

## Implementation Steps (TDD: Red-Green-Refactor)

### 1. Red Phase
- [ ] Verify current codama.json configuration
- [ ] Identify any needed configuration updates

### 2. Green Phase
- [ ] Update codama.json for prediction program:

```json
{
  "input": "anchor/target/idl/prediction.json",
  "output": "app/generated/prediction"
}
```

- [ ] Build Anchor program:
```bash
cd anchor && anchor build
```

- [ ] Verify IDL generated at `anchor/target/idl/prediction.json`

- [ ] Generate TypeScript client:
```bash
npm run codama:js
```

- [ ] Verify generated files:
  - `app/generated/prediction/index.ts`
  - `app/generated/prediction/instructions/`
  - `app/generated/prediction/accounts/`
  - `app/generated/prediction/types/`

### 3. Refactor Phase
- [ ] Remove old vault-related generated files
- [ ] Verify all types exported correctly
- [ ] Test import in a simple TypeScript file

## Completion Criteria
- [ ] IDL generated successfully
- [ ] TypeScript client generated
- [ ] All instructions have TypeScript functions
- [ ] All account types have TypeScript interfaces
- [ ] Operation verified: L3 (build succeeds)

## Notes
- Generated files should NOT be manually edited
- Re-run codama after any instruction changes
- Check for breaking changes in generated types

## Expected Generated Types

```typescript
// Market account
interface Market {
  id: bigint;
  question: string;
  description: string;
  creator: PublicKey;
  oracle: PublicKey;
  endTime: bigint;
  resolveTime: bigint;
  yesMint: PublicKey;
  noMint: PublicKey;
  yesPool: PublicKey;
  noPool: PublicKey;
  vaultUsdc: PublicKey;
  status: MarketStatus;
  result: MarketResult;
  totalLiquidity: bigint;
  createdAt: bigint;
  bump: number;
}

// Enums
type MarketStatus = 'Open' | 'Locked' | 'Resolved';
type MarketResult = 'Undecided' | 'Yes' | 'No';
type PoolSide = 'Yes' | 'No';

// Instruction builders
function createMarket(params: CreateMarketParams): TransactionInstruction;
function trade(params: TradeParams): TransactionInstruction;
// ... etc
```
