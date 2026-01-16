# Overall Design Document: Chausar Prediction Market Implementation

Generation Date: 2026-01-16
Updated: 2026-01-17
Target Plan Document: docs/todo.md

## Project Overview

### Purpose and Goals
Implement a decentralized prediction market on Solana where users can:
- Create binary (Yes/No) prediction markets with initial liquidity
- Trade YES/NO tokens via constant product AMM
- Provide/remove liquidity and earn trading fees
- Claim winnings after oracle resolution

### Background and Context
The existing codebase has a simple vault program that will be replaced with the prediction market logic. The frontend has basic wallet connection infrastructure using @solana/react-hooks.

## Task Division Design

### Division Policy
- **Horizontal Slice for Smart Contract**: Foundation first (accounts, PDAs) then instructions
- **Vertical Slice for Frontend**: Feature-complete pages with components
- **Verifiability**: L2 (test verification) for smart contract, L1 (functional operation) for frontend

### Inter-task Relationship Map

```
Phase 1: Smart Contract Foundation
  Task 01: Account Structures + Enums
    Deliverable: anchor/programs/prediction/src/state.rs
      |
  Task 02: PDA Derivation + Seeds Constants
    Deliverable: anchor/programs/prediction/src/seeds.rs
      |
  Task 03: Error Types
    Deliverable: anchor/programs/prediction/src/errors.rs

Phase 2: Smart Contract Instructions (depends on Phase 1)
  Task 04: create_market instruction
      |
  Task 05: lock_market instruction
      |
  Task 06: resolve_market instruction
      |
  Task 07: trade instruction
      |
  Task 08: add_liquidity instruction
      |
  Task 09: remove_liquidity instruction
      |
  Task 10: claim_winnings instruction

Phase 3: Smart Contract Testing (depends on Phase 2)
  Task 11: AMM calculation unit tests
      |
  Task 12: Instruction unit tests
      |
  Task 13: Integration tests
      |
  Task 14: Edge case and access control tests

Phase 4: Client Generation (depends on Phase 2)
  Task 15: Build Anchor program and generate TypeScript client

Phase 5: Frontend Infrastructure (depends on Phase 4)
  Task 16: Provider configuration and program ID setup
      |
  Task 17: AMM utilities (app/lib/amm.ts)
      |
  Task 18: Formatter utilities (app/lib/formatters.ts)
      |
  Task 19: useMarkets and useMarket hooks
      |
  Task 20: useTrade hook
      |
  Task 21: useLiquidity hook
      |
  Task 22: usePortfolio hook

Phase 6: Frontend Pages (depends on Phase 5)
  Task 23: Market List Page (app/markets/page.tsx)
      |
  Task 24: Market Detail Page (app/markets/[id]/page.tsx)
      |
  Task 25: Create Market Page (app/create/page.tsx)
      |
  Task 26: Portfolio Page (app/portfolio/page.tsx)

Phase 7: Frontend Components (can parallel with Phase 6)
  Task 27: Navigation component
      |
  Task 28: MarketStatus component
      |
  Task 29: MarketCard component
      |
  Task 30: TradingPanel component
      |
  Task 31: LiquidityPanel component
      |
  Task 32: PositionCard component
      |
  Task 33: Form input components (AmountInput, PriceDisplay, SlippageSettings)
      |
  Task 34: CreateMarketForm component

Phase 8: Wallet Integration Enhancement
  Task 35: Enhanced wallet connection with persistence

Phase 9: Quality Assurance
  Task 36: Frontend component tests
      |
  Task 37: E2E tests

Phase 10: Deployment
  Task 38: Devnet deployment and configuration
```

### Task Summary by Phase

| Phase | Tasks | Description |
|-------|-------|-------------|
| Phase 1 | 01-03 | Smart Contract Foundation |
| Phase 2 | 04-10 | Smart Contract Instructions |
| Phase 3 | 11-14 | Smart Contract Testing |
| Phase 4 | 15 | Client Generation |
| Phase 5 | 16-22 | Frontend Infrastructure |
| Phase 6 | 23-26 | Frontend Pages |
| Phase 7 | 27-34 | Frontend Components |
| Phase 8 | 35 | Wallet Integration |
| Phase 9 | 36-37 | Quality Assurance |
| Phase 10 | 38 | Deployment |

**Total Tasks: 38**

### Common Processing Points
- **PDA Seeds**: Shared constants across instructions (seeds.rs)
- **AMM Calculations**: Shared utility for price/output calculations
- **Error Types**: Centralized error definitions
- **Formatters**: Shared formatting utilities for frontend

## Implementation Considerations

### Principles to Maintain Throughout
1. Use TDD (Red-Green-Refactor) for all smart contract code
2. Follow Anchor best practices for account validation
3. Use generated TypeScript client - do not manually edit app/generated/
4. Maintain consistent error handling patterns

### Risks and Countermeasures
- Risk: AMM calculation precision issues
  Countermeasure: Use u128 for intermediate calculations, extensive unit tests

- Risk: PDA collision or incorrect derivation
  Countermeasure: Centralized seeds.rs module, comprehensive PDA tests

- Risk: Frontend/client type mismatch after regeneration
  Countermeasure: Always regenerate client after instruction changes

### Impact Scope Management
- Allowed change scope: anchor/programs/prediction/, app/, docs/
- No-change areas: External dependencies, anchor configuration patterns
- Replace existing vault program with prediction market program

### Existing Code to Replace
- `anchor/programs/vault/` - Will be replaced by `anchor/programs/prediction/`
- `app/generated/vault/` - Will be replaced by `app/generated/prediction/`

## File Structure After Completion

```
chausar/
├── anchor/
│   ├── programs/
│   │   └── prediction/
│   │       └── src/
│   │           ├── lib.rs
│   │           ├── state.rs
│   │           ├── seeds.rs
│   │           ├── errors.rs
│   │           └── instructions/
│   │               ├── mod.rs
│   │               ├── create_market.rs
│   │               ├── lock_market.rs
│   │               ├── resolve_market.rs
│   │               ├── trade.rs
│   │               ├── add_liquidity.rs
│   │               ├── remove_liquidity.rs
│   │               └── claim_winnings.rs
│   └── tests/
├── app/
│   ├── components/
│   │   ├── providers.tsx
│   │   ├── Navigation.tsx
│   │   ├── MarketStatus.tsx
│   │   ├── MarketCard.tsx
│   │   ├── TradingPanel.tsx
│   │   ├── LiquidityPanel.tsx
│   │   ├── PositionCard.tsx
│   │   ├── AmountInput.tsx
│   │   ├── PriceDisplay.tsx
│   │   ├── SlippageSettings.tsx
│   │   └── CreateMarketForm.tsx
│   ├── hooks/
│   │   ├── index.ts
│   │   ├── useMarkets.ts
│   │   ├── useMarket.ts
│   │   ├── useTrade.ts
│   │   ├── useLiquidity.ts
│   │   ├── usePortfolio.ts
│   │   └── useWallet.ts
│   ├── lib/
│   │   ├── config.ts
│   │   ├── amm.ts
│   │   ├── formatters.ts
│   │   └── wallet.ts
│   ├── generated/
│   │   └── prediction/
│   ├── markets/
│   │   ├── page.tsx
│   │   └── [id]/
│   │       └── page.tsx
│   ├── create/
│   │   └── page.tsx
│   ├── portfolio/
│   │   └── page.tsx
│   └── __tests__/
└── e2e/
```

## Execution Order

Recommended execution order with phase dependencies:

1. **Phase 1** (Tasks 01-03): Smart Contract Foundation
2. **Phase 2** (Tasks 04-10): Smart Contract Instructions
3. **Phase 3** (Tasks 11-14): Smart Contract Testing
4. **Phase 4** (Task 15): Client Generation
5. **Phase 5** (Tasks 16-22): Frontend Infrastructure
6. **Phase 6 + 7** (Tasks 23-34): Frontend Pages and Components (can be parallelized)
7. **Phase 8** (Task 35): Wallet Integration
8. **Phase 9** (Tasks 36-37): Quality Assurance
9. **Phase 10** (Task 38): Deployment

Note: Phase 6 and Phase 7 can be executed in parallel as pages and components can be developed together.
