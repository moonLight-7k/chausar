# Task: Form Input Components (AmountInput, PriceDisplay, SlippageSettings)

Metadata:
- Dependencies: task-18 (formatters)
- Provides: app/components/AmountInput.tsx, app/components/PriceDisplay.tsx, app/components/SlippageSettings.tsx
- Size: Small (3 files)

## Implementation Content
Create shared form input components:
- AmountInput: USDC amount input with validation
- PriceDisplay: YES/NO price display with visual bar
- SlippageSettings: Slippage tolerance configuration

## Target Files
- [ ] `app/components/AmountInput.tsx`
- [ ] `app/components/PriceDisplay.tsx`
- [ ] `app/components/SlippageSettings.tsx`

## Implementation Steps (TDD: Red-Green-Refactor)

### 1. Red Phase
- [ ] Define component interfaces
- [ ] Create file structure

### 2. Green Phase

#### AmountInput Component
```typescript
// app/components/AmountInput.tsx
"use client";

import { useCallback } from "react";

interface AmountInputProps {
  value: string;
  onChange: (value: string) => void;
  disabled?: boolean;
  placeholder?: string;
  max?: number;
  label?: string;
  error?: string;
}

export function AmountInput({
  value,
  onChange,
  disabled = false,
  placeholder = "0.00",
  max,
  label,
  error,
}: AmountInputProps) {
  const handleChange = useCallback(
    (e: React.ChangeEvent<HTMLInputElement>) => {
      const inputValue = e.target.value;

      // Allow empty input
      if (inputValue === "") {
        onChange("");
        return;
      }

      // Validate numeric input with decimals
      const regex = /^\d*\.?\d{0,6}$/;
      if (regex.test(inputValue)) {
        // Enforce max if provided
        if (max !== undefined && parseFloat(inputValue) > max) {
          onChange(max.toString());
        } else {
          onChange(inputValue);
        }
      }
    },
    [onChange, max]
  );

  const handleMax = useCallback(() => {
    if (max !== undefined) {
      onChange(max.toString());
    }
  }, [onChange, max]);

  return (
    <div>
      {label && (
        <label className="block text-sm font-medium text-muted mb-2">
          {label}
        </label>
      )}
      <div className="relative">
        <input
          type="text"
          inputMode="decimal"
          value={value}
          onChange={handleChange}
          disabled={disabled}
          placeholder={placeholder}
          className={`w-full rounded-lg border px-4 py-3 text-lg font-medium
            focus:outline-none focus:ring-2 focus:ring-primary/50
            disabled:bg-gray-50 disabled:text-gray-400
            ${error ? "border-red-300 focus:ring-red-500/50" : "border-border-low"}`}
        />
        <div className="absolute right-3 top-1/2 -translate-y-1/2 flex items-center gap-2">
          {max !== undefined && !disabled && (
            <button
              type="button"
              onClick={handleMax}
              className="text-xs text-primary font-medium hover:underline"
            >
              MAX
            </button>
          )}
          <span className="text-sm text-muted font-medium">USDC</span>
        </div>
      </div>
      {error && (
        <p className="mt-1 text-sm text-red-500">{error}</p>
      )}
    </div>
  );
}
```

#### PriceDisplay Component
```typescript
// app/components/PriceDisplay.tsx
interface PriceDisplayProps {
  yesPrice: number; // 0-1 decimal
  noPrice: number;  // 0-1 decimal
  size?: "sm" | "md" | "lg";
  showBars?: boolean;
}

const sizeConfig = {
  sm: {
    container: "gap-1",
    text: "text-sm",
    bar: "h-1.5",
  },
  md: {
    container: "gap-2",
    text: "text-base",
    bar: "h-2",
  },
  lg: {
    container: "gap-3",
    text: "text-lg",
    bar: "h-3",
  },
};

export function PriceDisplay({
  yesPrice,
  noPrice,
  size = "md",
  showBars = true,
}: PriceDisplayProps) {
  const config = sizeConfig[size];

  return (
    <div className={`space-y-2 ${config.container}`}>
      {/* YES Price */}
      <div className="flex items-center gap-3">
        <span className={`w-8 font-semibold text-green-600 ${config.text}`}>
          YES
        </span>
        {showBars && (
          <div className="flex-1 bg-gray-100 rounded-full overflow-hidden">
            <div
              className={`${config.bar} bg-green-500 rounded-full transition-all duration-300`}
              style={{ width: `${yesPrice * 100}%` }}
            />
          </div>
        )}
        <span className={`w-12 text-right font-semibold ${config.text}`}>
          {(yesPrice * 100).toFixed(1)}%
        </span>
      </div>

      {/* NO Price */}
      <div className="flex items-center gap-3">
        <span className={`w-8 font-semibold text-red-600 ${config.text}`}>
          NO
        </span>
        {showBars && (
          <div className="flex-1 bg-gray-100 rounded-full overflow-hidden">
            <div
              className={`${config.bar} bg-red-500 rounded-full transition-all duration-300`}
              style={{ width: `${noPrice * 100}%` }}
            />
          </div>
        )}
        <span className={`w-12 text-right font-semibold ${config.text}`}>
          {(noPrice * 100).toFixed(1)}%
        </span>
      </div>
    </div>
  );
}

// Compact inline variant
interface PriceInlineProps {
  yesPrice: number;
  noPrice: number;
}

export function PriceInline({ yesPrice, noPrice }: PriceInlineProps) {
  return (
    <div className="flex items-center gap-3 text-sm">
      <span>
        <span className="font-medium text-green-600">YES</span>{" "}
        <span className="font-semibold">{(yesPrice * 100).toFixed(0)}%</span>
      </span>
      <span className="text-muted">/</span>
      <span>
        <span className="font-medium text-red-600">NO</span>{" "}
        <span className="font-semibold">{(noPrice * 100).toFixed(0)}%</span>
      </span>
    </div>
  );
}
```

#### SlippageSettings Component
```typescript
// app/components/SlippageSettings.tsx
"use client";

import { useState } from "react";

interface SlippageSettingsProps {
  value: number; // Percentage (e.g., 5 for 5%)
  onChange: (value: number) => void;
  className?: string;
}

const presets = [1, 3, 5];

export function SlippageSettings({
  value,
  onChange,
  className = "",
}: SlippageSettingsProps) {
  const [isOpen, setIsOpen] = useState(false);
  const [customValue, setCustomValue] = useState("");

  const handlePresetClick = (preset: number) => {
    onChange(preset);
    setCustomValue("");
  };

  const handleCustomChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const input = e.target.value;
    setCustomValue(input);

    const parsed = parseFloat(input);
    if (!isNaN(parsed) && parsed >= 0.1 && parsed <= 50) {
      onChange(parsed);
    }
  };

  return (
    <div className={className}>
      <button
        type="button"
        onClick={() => setIsOpen(!isOpen)}
        className="flex items-center gap-1 text-sm text-muted hover:text-foreground"
      >
        <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2}
            d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"
          />
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2}
            d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
          />
        </svg>
        Slippage: {value}%
        <svg
          className={`w-4 h-4 transition-transform ${isOpen ? "rotate-180" : ""}`}
          fill="none" viewBox="0 0 24 24" stroke="currentColor"
        >
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
        </svg>
      </button>

      {isOpen && (
        <div className="mt-2 p-3 bg-gray-50 rounded-lg">
          <p className="text-xs text-muted mb-2">
            Your transaction will revert if the price changes by more than this percentage.
          </p>

          <div className="flex items-center gap-2">
            {presets.map((preset) => (
              <button
                key={preset}
                type="button"
                onClick={() => handlePresetClick(preset)}
                className={`px-3 py-1 rounded-md text-sm font-medium transition-colors ${
                  value === preset && !customValue
                    ? "bg-primary text-white"
                    : "bg-white border border-border-low hover:border-border-high"
                }`}
              >
                {preset}%
              </button>
            ))}

            <div className="relative flex-1">
              <input
                type="text"
                value={customValue}
                onChange={handleCustomChange}
                placeholder="Custom"
                className="w-full rounded-md border border-border-low px-3 py-1 text-sm focus:outline-none focus:ring-2 focus:ring-primary/50"
              />
              <span className="absolute right-3 top-1/2 -translate-y-1/2 text-sm text-muted">
                %
              </span>
            </div>
          </div>

          {value > 10 && (
            <p className="mt-2 text-xs text-yellow-600">
              High slippage tolerance. Your trade may be frontrun.
            </p>
          )}
        </div>
      )}
    </div>
  );
}
```

### 3. Refactor Phase
- [ ] Add input validation feedback
- [ ] Ensure consistent styling
- [ ] Add accessibility attributes

## Completion Criteria
- [ ] AmountInput validates numeric input
- [ ] PriceDisplay shows correct percentages
- [ ] SlippageSettings persists selection
- [ ] All components render correctly
- [ ] Build passes
- [ ] Operation verified: L1 (functional components)

## Notes
- Impact scope: Used across TradingPanel, LiquidityPanel, forms
- Constraints: Must handle edge cases (0, negative, very large numbers)
- AmountInput should support max 6 decimal places for USDC
