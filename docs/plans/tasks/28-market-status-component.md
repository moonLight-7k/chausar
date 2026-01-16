# Task: MarketStatus Component

Metadata:
- Dependencies: task-15 (client generation - for enum types)
- Provides: app/components/MarketStatus.tsx
- Size: Small (1 file)

## Implementation Content
Create a status badge component displaying market status:
- Open (green) - Trading active
- Locked (yellow) - Awaiting resolution
- Resolved (blue/gray) - Outcome determined

## Target Files
- [ ] `app/components/MarketStatus.tsx`

## Implementation Steps (TDD: Red-Green-Refactor)

### 1. Red Phase
- [ ] Define status types matching contract enums
- [ ] Create component structure

### 2. Green Phase
- [ ] Implement status badge component:

```typescript
// app/components/MarketStatus.tsx
import { MarketStatus as MarketStatusEnum } from "@/generated/prediction";

interface MarketStatusProps {
  status: MarketStatusEnum;
  size?: "sm" | "md" | "lg";
}

const statusConfig = {
  Open: {
    label: "Open",
    className: "bg-green-100 text-green-800 border-green-200",
  },
  Locked: {
    label: "Locked",
    className: "bg-yellow-100 text-yellow-800 border-yellow-200",
  },
  Resolved: {
    label: "Resolved",
    className: "bg-gray-100 text-gray-800 border-gray-200",
  },
};

const sizeConfig = {
  sm: "px-2 py-0.5 text-xs",
  md: "px-2.5 py-1 text-sm",
  lg: "px-3 py-1.5 text-base",
};

export function MarketStatus({ status, size = "md" }: MarketStatusProps) {
  // Handle enum object from Codama-generated client
  const statusKey = typeof status === "object"
    ? Object.keys(status)[0] as keyof typeof statusConfig
    : status as keyof typeof statusConfig;

  const config = statusConfig[statusKey] || statusConfig.Open;

  return (
    <span
      className={`inline-flex items-center rounded-full border font-medium ${config.className} ${sizeConfig[size]}`}
    >
      {config.label}
    </span>
  );
}

// Also export a result badge for resolved markets
interface MarketResultProps {
  result: "Yes" | "No" | "Undecided";
  size?: "sm" | "md" | "lg";
}

const resultConfig = {
  Yes: {
    label: "YES",
    className: "bg-green-500 text-white",
  },
  No: {
    label: "NO",
    className: "bg-red-500 text-white",
  },
  Undecided: {
    label: "Undecided",
    className: "bg-gray-200 text-gray-600",
  },
};

export function MarketResult({ result, size = "md" }: MarketResultProps) {
  // Handle enum object from Codama-generated client
  const resultKey = typeof result === "object"
    ? Object.keys(result)[0] as keyof typeof resultConfig
    : result as keyof typeof resultConfig;

  const config = resultConfig[resultKey] || resultConfig.Undecided;

  return (
    <span
      className={`inline-flex items-center rounded-full font-bold ${config.className} ${sizeConfig[size]}`}
    >
      {config.label}
    </span>
  );
}
```

### 3. Refactor Phase
- [ ] Ensure consistent styling with design system
- [ ] Add optional icon support
- [ ] Verify enum handling from generated client

## Completion Criteria
- [ ] Renders correct badge for each status
- [ ] Handles Codama enum objects correctly
- [ ] Size variants work
- [ ] Build passes
- [ ] Operation verified: L1 (functional component)

## Notes
- Impact scope: Used in MarketCard, Market Detail page
- Constraints: Must handle Codama-generated enum format
- The status enum from Codama may be `{ Open: {} }` format
