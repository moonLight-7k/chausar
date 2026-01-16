# Task: CreateMarketForm Component

Metadata:
- Dependencies: task-33 (AmountInput), task-19 (useMarkets for create function)
- Provides: app/components/CreateMarketForm.tsx
- Size: Small (1 file)

## Implementation Content
Create the market creation form component:
- Question input (max 280 chars)
- Description textarea (max 1000 chars)
- End time datetime picker
- Resolve time datetime picker
- Initial liquidity input (min 100 USDC)
- Oracle address input (optional, defaults to user)
- Form validation
- Submit with transaction

## Target Files
- [ ] `app/components/CreateMarketForm.tsx`

## Implementation Steps (TDD: Red-Green-Refactor)

### 1. Red Phase
- [ ] Define form data interface
- [ ] Create component structure

### 2. Green Phase
- [ ] Implement form component:

```typescript
// app/components/CreateMarketForm.tsx
"use client";

import { useState, useCallback, useMemo } from "react";
import { useRouter } from "next/navigation";
import { useWalletConnection } from "@solana/react-hooks";
import { useCreateMarket } from "@/hooks/useCreateMarket";
import { AmountInput } from "./AmountInput";

interface FormData {
  question: string;
  description: string;
  endTime: string;
  resolveTime: string;
  initialLiquidity: string;
  oracle: string;
}

interface FormErrors {
  question?: string;
  description?: string;
  endTime?: string;
  resolveTime?: string;
  initialLiquidity?: string;
  oracle?: string;
}

const MIN_LIQUIDITY = 100;
const MAX_QUESTION_LENGTH = 280;
const MAX_DESCRIPTION_LENGTH = 1000;

export function CreateMarketForm() {
  const router = useRouter();
  const { wallet, status } = useWalletConnection();
  const { createMarket, isLoading, error: txError } = useCreateMarket();

  const [formData, setFormData] = useState<FormData>({
    question: "",
    description: "",
    endTime: "",
    resolveTime: "",
    initialLiquidity: "",
    oracle: "",
  });

  const [errors, setErrors] = useState<FormErrors>({});
  const [touched, setTouched] = useState<Set<string>>(new Set());

  // Validate form
  const validate = useCallback((): FormErrors => {
    const newErrors: FormErrors = {};
    const now = new Date();

    // Question validation
    if (!formData.question.trim()) {
      newErrors.question = "Question is required";
    } else if (formData.question.length > MAX_QUESTION_LENGTH) {
      newErrors.question = `Question must be ${MAX_QUESTION_LENGTH} characters or less`;
    }

    // Description validation
    if (formData.description.length > MAX_DESCRIPTION_LENGTH) {
      newErrors.description = `Description must be ${MAX_DESCRIPTION_LENGTH} characters or less`;
    }

    // End time validation
    if (!formData.endTime) {
      newErrors.endTime = "End time is required";
    } else {
      const endDate = new Date(formData.endTime);
      if (endDate <= now) {
        newErrors.endTime = "End time must be in the future";
      }
    }

    // Resolve time validation
    if (!formData.resolveTime) {
      newErrors.resolveTime = "Resolve time is required";
    } else if (formData.endTime) {
      const endDate = new Date(formData.endTime);
      const resolveDate = new Date(formData.resolveTime);
      if (resolveDate <= endDate) {
        newErrors.resolveTime = "Resolve time must be after end time";
      }
    }

    // Liquidity validation
    const liquidity = parseFloat(formData.initialLiquidity);
    if (!formData.initialLiquidity || isNaN(liquidity)) {
      newErrors.initialLiquidity = "Initial liquidity is required";
    } else if (liquidity < MIN_LIQUIDITY) {
      newErrors.initialLiquidity = `Minimum liquidity is ${MIN_LIQUIDITY} USDC`;
    }

    // Oracle validation (optional - defaults to wallet)
    if (formData.oracle && !/^[1-9A-HJ-NP-Za-km-z]{32,44}$/.test(formData.oracle)) {
      newErrors.oracle = "Invalid Solana address";
    }

    return newErrors;
  }, [formData]);

  const validationErrors = useMemo(() => validate(), [validate]);
  const isValid = Object.keys(validationErrors).length === 0;
  const isConnected = status === "connected";

  const handleChange = useCallback((field: keyof FormData, value: string) => {
    setFormData((prev) => ({ ...prev, [field]: value }));
    setTouched((prev) => new Set(prev).add(field));
  }, []);

  const handleBlur = useCallback((field: keyof FormData) => {
    setTouched((prev) => new Set(prev).add(field));
    setErrors(validate());
  }, [validate]);

  const handleSubmit = useCallback(async (e: React.FormEvent) => {
    e.preventDefault();

    const validationErrors = validate();
    setErrors(validationErrors);

    if (Object.keys(validationErrors).length > 0) {
      return;
    }

    const endTimestamp = Math.floor(new Date(formData.endTime).getTime() / 1000);
    const resolveTimestamp = Math.floor(new Date(formData.resolveTime).getTime() / 1000);
    const liquidityAmount = BigInt(Math.floor(parseFloat(formData.initialLiquidity) * 1_000_000));

    const marketId = await createMarket({
      question: formData.question,
      description: formData.description,
      endTime: BigInt(endTimestamp),
      resolveTime: BigInt(resolveTimestamp),
      initialLiquidity: liquidityAmount,
      oracle: formData.oracle || undefined, // Use wallet if not provided
    });

    if (marketId !== null) {
      router.push(`/markets/${marketId.toString()}`);
    }
  }, [formData, validate, createMarket, router]);

  const showError = (field: keyof FormData) => touched.has(field) && errors[field];

  return (
    <form onSubmit={handleSubmit} className="space-y-6">
      {/* Question */}
      <div>
        <label className="block text-sm font-medium mb-2">
          Question <span className="text-red-500">*</span>
        </label>
        <input
          type="text"
          value={formData.question}
          onChange={(e) => handleChange("question", e.target.value)}
          onBlur={() => handleBlur("question")}
          placeholder="Will Bitcoin close above $100,000 on January 31, 2026?"
          maxLength={MAX_QUESTION_LENGTH}
          className={`w-full rounded-lg border px-4 py-3 focus:outline-none focus:ring-2 focus:ring-primary/50 ${
            showError("question") ? "border-red-300" : "border-border-low"
          }`}
        />
        <div className="flex justify-between mt-1">
          {showError("question") && (
            <p className="text-sm text-red-500">{errors.question}</p>
          )}
          <p className="text-xs text-muted ml-auto">
            {formData.question.length}/{MAX_QUESTION_LENGTH}
          </p>
        </div>
      </div>

      {/* Description */}
      <div>
        <label className="block text-sm font-medium mb-2">
          Description <span className="text-muted">(optional)</span>
        </label>
        <textarea
          value={formData.description}
          onChange={(e) => handleChange("description", e.target.value)}
          onBlur={() => handleBlur("description")}
          placeholder="Additional details about the market resolution criteria..."
          maxLength={MAX_DESCRIPTION_LENGTH}
          rows={4}
          className={`w-full rounded-lg border px-4 py-3 focus:outline-none focus:ring-2 focus:ring-primary/50 resize-none ${
            showError("description") ? "border-red-300" : "border-border-low"
          }`}
        />
        <div className="flex justify-between mt-1">
          {showError("description") && (
            <p className="text-sm text-red-500">{errors.description}</p>
          )}
          <p className="text-xs text-muted ml-auto">
            {formData.description.length}/{MAX_DESCRIPTION_LENGTH}
          </p>
        </div>
      </div>

      {/* Times */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        <div>
          <label className="block text-sm font-medium mb-2">
            Trading Ends <span className="text-red-500">*</span>
          </label>
          <input
            type="datetime-local"
            value={formData.endTime}
            onChange={(e) => handleChange("endTime", e.target.value)}
            onBlur={() => handleBlur("endTime")}
            className={`w-full rounded-lg border px-4 py-3 focus:outline-none focus:ring-2 focus:ring-primary/50 ${
              showError("endTime") ? "border-red-300" : "border-border-low"
            }`}
          />
          {showError("endTime") && (
            <p className="text-sm text-red-500 mt-1">{errors.endTime}</p>
          )}
        </div>

        <div>
          <label className="block text-sm font-medium mb-2">
            Resolution Time <span className="text-red-500">*</span>
          </label>
          <input
            type="datetime-local"
            value={formData.resolveTime}
            onChange={(e) => handleChange("resolveTime", e.target.value)}
            onBlur={() => handleBlur("resolveTime")}
            min={formData.endTime}
            className={`w-full rounded-lg border px-4 py-3 focus:outline-none focus:ring-2 focus:ring-primary/50 ${
              showError("resolveTime") ? "border-red-300" : "border-border-low"
            }`}
          />
          {showError("resolveTime") && (
            <p className="text-sm text-red-500 mt-1">{errors.resolveTime}</p>
          )}
        </div>
      </div>

      {/* Initial Liquidity */}
      <div>
        <label className="block text-sm font-medium mb-2">
          Initial Liquidity <span className="text-red-500">*</span>
        </label>
        <AmountInput
          value={formData.initialLiquidity}
          onChange={(value) => handleChange("initialLiquidity", value)}
          placeholder="100.00"
          error={showError("initialLiquidity") ? errors.initialLiquidity : undefined}
        />
        <p className="text-xs text-muted mt-1">
          Minimum {MIN_LIQUIDITY} USDC required. This will be split equally between YES and NO pools.
        </p>
      </div>

      {/* Oracle Address */}
      <div>
        <label className="block text-sm font-medium mb-2">
          Oracle Address <span className="text-muted">(optional)</span>
        </label>
        <input
          type="text"
          value={formData.oracle}
          onChange={(e) => handleChange("oracle", e.target.value)}
          onBlur={() => handleBlur("oracle")}
          placeholder={wallet?.address.toString() || "Leave empty to use your wallet"}
          className={`w-full rounded-lg border px-4 py-3 font-mono text-sm focus:outline-none focus:ring-2 focus:ring-primary/50 ${
            showError("oracle") ? "border-red-300" : "border-border-low"
          }`}
        />
        {showError("oracle") && (
          <p className="text-sm text-red-500 mt-1">{errors.oracle}</p>
        )}
        <p className="text-xs text-muted mt-1">
          The oracle is responsible for resolving the market outcome. Defaults to your wallet.
        </p>
      </div>

      {/* Transaction Error */}
      {txError && (
        <div className="p-3 bg-red-50 border border-red-200 rounded-lg text-sm text-red-600">
          {txError.message}
        </div>
      )}

      {/* Submit Button */}
      <button
        type="submit"
        disabled={!isConnected || !isValid || isLoading}
        className={`w-full rounded-lg py-3 text-sm font-medium transition-colors ${
          isConnected && isValid && !isLoading
            ? "bg-primary text-white hover:bg-primary/90"
            : "bg-gray-200 text-gray-500 cursor-not-allowed"
        }`}
      >
        {!isConnected
          ? "Connect Wallet"
          : isLoading
          ? "Creating Market..."
          : "Create Market"}
      </button>
    </form>
  );
}
```

### 3. Refactor Phase
- [ ] Add form persistence (localStorage)
- [ ] Improve validation messages
- [ ] Add confirmation step before submission

## Completion Criteria
- [ ] All form fields validate correctly
- [ ] End time must be in future
- [ ] Resolve time must be after end time
- [ ] Min liquidity enforced (100 USDC)
- [ ] Form submits and creates market
- [ ] Build passes
- [ ] Operation verified: L1 (functional form)

## Notes
- Impact scope: Used in Create Market page
- Constraints: Must match PRD validation requirements
- Oracle defaults to user wallet if not specified
