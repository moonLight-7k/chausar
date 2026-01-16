# Task: Formatter Utilities

Metadata:
- Dependencies: task-16 (config)
- Provides: app/lib/formatters.ts
- Size: Small (1 file)

## Implementation Content
Create formatting utilities for display:
- Price formatting (percentage, currency)
- Time formatting (countdown, date)
- Amount formatting (token amounts with decimals)
- Address formatting (truncation)

## Target Files
- [ ] `app/lib/formatters.ts`

## Implementation Steps (TDD: Red-Green-Refactor)

### 1. Red Phase
- [ ] Define function signatures
- [ ] Identify formatting requirements

### 2. Green Phase
- [ ] Implement formatters:

```typescript
// app/lib/formatters.ts

/**
 * Format token amount (6 decimals) to human readable
 * 100_000_000 -> "100.00"
 */
export function formatTokenAmount(
  amount: bigint | number,
  decimals: number = 6
): string {
  const value = typeof amount === "bigint" ? Number(amount) : amount;
  const divisor = Math.pow(10, decimals);
  return (value / divisor).toLocaleString("en-US", {
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  });
}

/**
 * Format price as percentage
 * 0.65 -> "65%"
 */
export function formatPrice(price: number): string {
  return `${(price * 100).toFixed(0)}%`;
}

/**
 * Format price with cents
 * 0.65 -> "$0.65"
 */
export function formatPriceCurrency(price: number): string {
  return `$${price.toFixed(2)}`;
}

/**
 * Format remaining time
 * Returns "2d 5h", "5h 30m", "30m", "Ended"
 */
export function formatTimeRemaining(endTime: number | bigint): string {
  const end = typeof endTime === "bigint" ? Number(endTime) : endTime;
  const now = Date.now() / 1000;
  const remaining = end - now;

  if (remaining <= 0) return "Ended";

  const days = Math.floor(remaining / 86400);
  const hours = Math.floor((remaining % 86400) / 3600);
  const minutes = Math.floor((remaining % 3600) / 60);

  if (days > 0) return `${days}d ${hours}h`;
  if (hours > 0) return `${hours}h ${minutes}m`;
  return `${minutes}m`;
}

/**
 * Format date from unix timestamp
 * Returns localized date string
 */
export function formatDate(timestamp: number | bigint): string {
  const ts = typeof timestamp === "bigint" ? Number(timestamp) : timestamp;
  return new Date(ts * 1000).toLocaleDateString("en-US", {
    year: "numeric",
    month: "short",
    day: "numeric",
    hour: "2-digit",
    minute: "2-digit",
  });
}

/**
 * Truncate address for display
 * "5Abc...xyz" (first 4, last 3)
 */
export function formatAddress(address: string): string {
  if (address.length <= 10) return address;
  return `${address.slice(0, 4)}...${address.slice(-3)}`;
}

/**
 * Format large numbers with K/M suffix
 * 1500 -> "1.5K", 1500000 -> "1.5M"
 */
export function formatCompactNumber(value: number): string {
  if (value >= 1_000_000) {
    return `${(value / 1_000_000).toFixed(1)}M`;
  }
  if (value >= 1_000) {
    return `${(value / 1_000).toFixed(1)}K`;
  }
  return value.toFixed(0);
}
```

### 3. Refactor Phase
- [ ] Add locale support if needed
- [ ] Test edge cases (zero, negative, very large)
- [ ] Build and verify

## Completion Criteria
- [ ] All formatters implemented
- [ ] Handles edge cases gracefully
- [ ] Build passes
- [ ] Operation verified: L3 (build succeeds)

## Notes
- Use for consistent display across app
- All timestamps are Unix seconds
- Token amounts use 6 decimals (USDC standard)
