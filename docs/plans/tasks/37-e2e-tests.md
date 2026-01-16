# Task: End-to-End Tests

Metadata:
- Dependencies: All phases 1-8 complete
- Provides: E2E test suite
- Size: Medium (3-5 files)

## Implementation Content
Create E2E tests for complete user flows:
- Create market flow
- Trade flow (buy YES/NO)
- Liquidity flow (add/remove)
- Resolution and claim flow
- Portfolio view flow

## Target Files
- [ ] `e2e/market-creation.spec.ts`
- [ ] `e2e/trading.spec.ts`
- [ ] `e2e/liquidity.spec.ts`
- [ ] `e2e/claim-winnings.spec.ts`
- [ ] `e2e/portfolio.spec.ts`

## Implementation Steps (TDD: Red-Green-Refactor)

### 1. Red Phase
- [ ] Set up Playwright or Cypress
- [ ] Configure test environment for devnet

### 2. Green Phase

#### E2E Configuration
```typescript
// playwright.config.ts
import { defineConfig } from "@playwright/test";

export default defineConfig({
  testDir: "./e2e",
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: "html",
  use: {
    baseURL: "http://localhost:3000",
    trace: "on-first-retry",
  },
  projects: [
    {
      name: "chromium",
      use: { browserName: "chromium" },
    },
  ],
  webServer: {
    command: "npm run dev",
    url: "http://localhost:3000",
    reuseExistingServer: !process.env.CI,
  },
});
```

#### Market Creation E2E Test
```typescript
// e2e/market-creation.spec.ts
import { test, expect } from "@playwright/test";

test.describe("Market Creation", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/create");
  });

  test("shows create market form", async ({ page }) => {
    await expect(page.getByText("Create Market")).toBeVisible();
    await expect(page.getByPlaceholder(/Will Bitcoin/)).toBeVisible();
  });

  test("validates required fields", async ({ page }) => {
    // Try to submit empty form
    await page.getByRole("button", { name: "Create Market" }).click();

    // Should show validation errors
    await expect(page.getByText("Question is required")).toBeVisible();
    await expect(page.getByText("End time is required")).toBeVisible();
  });

  test("validates minimum liquidity", async ({ page }) => {
    await page.getByPlaceholder(/Will Bitcoin/).fill("Test question?");

    // Set valid times
    const tomorrow = new Date(Date.now() + 86400000).toISOString().slice(0, 16);
    const nextWeek = new Date(Date.now() + 604800000).toISOString().slice(0, 16);

    await page.locator('input[type="datetime-local"]').first().fill(tomorrow);
    await page.locator('input[type="datetime-local"]').last().fill(nextWeek);

    // Enter less than minimum liquidity
    await page.getByPlaceholder("100.00").fill("50");

    // Should show error
    await expect(page.getByText(/Minimum liquidity is 100/)).toBeVisible();
  });

  test("full market creation flow", async ({ page }) => {
    // This test requires a connected wallet with devnet USDC
    test.skip(!process.env.TEST_WALLET_CONNECTED, "Requires wallet connection");

    await page.getByPlaceholder(/Will Bitcoin/).fill("Will BTC reach $100k by end of 2026?");
    await page.getByPlaceholder(/Additional details/).fill("Based on CoinGecko price at 00:00 UTC");

    const tomorrow = new Date(Date.now() + 86400000).toISOString().slice(0, 16);
    const nextWeek = new Date(Date.now() + 604800000).toISOString().slice(0, 16);

    await page.locator('input[type="datetime-local"]').first().fill(tomorrow);
    await page.locator('input[type="datetime-local"]').last().fill(nextWeek);
    await page.getByPlaceholder("100.00").fill("100");

    await page.getByRole("button", { name: "Create Market" }).click();

    // Should redirect to new market page
    await expect(page).toHaveURL(/\/markets\/\d+/);
  });
});
```

#### Trading E2E Test
```typescript
// e2e/trading.spec.ts
import { test, expect } from "@playwright/test";

test.describe("Trading", () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to an existing market
    await page.goto("/markets");
    await page.getByRole("link").first().click();
  });

  test("shows trading panel", async ({ page }) => {
    await expect(page.getByText("Trade")).toBeVisible();
    await expect(page.getByText("Buy YES")).toBeVisible();
    await expect(page.getByText("Buy NO")).toBeVisible();
  });

  test("calculates expected output", async ({ page }) => {
    await page.getByPlaceholder("0.00").first().fill("10");

    // Should show expected tokens
    await expect(page.getByText(/Expected/)).toBeVisible();
  });

  test("shows price impact", async ({ page }) => {
    await page.getByPlaceholder("0.00").first().fill("100");

    await expect(page.getByText(/Price Impact/)).toBeVisible();
  });

  test("switches between YES and NO", async ({ page }) => {
    await page.getByText("Buy NO").click();

    // NO tab should be active
    const noButton = page.getByText("Buy NO");
    await expect(noButton).toHaveClass(/bg-red-500/);
  });

  test("shows connect wallet prompt when disconnected", async ({ page }) => {
    await expect(page.getByRole("button", { name: "Connect Wallet" })).toBeVisible();
  });

  test("full trade flow", async ({ page }) => {
    test.skip(!process.env.TEST_WALLET_CONNECTED, "Requires wallet connection");

    await page.getByPlaceholder("0.00").first().fill("10");
    await page.getByRole("button", { name: /Buy YES/ }).click();

    // Should show transaction confirmation
    await expect(page.getByText(/Trading|Confirming/)).toBeVisible();

    // Wait for transaction
    await page.waitForSelector('[data-testid="trade-success"]', { timeout: 30000 });
  });
});
```

#### Liquidity E2E Test
```typescript
// e2e/liquidity.spec.ts
import { test, expect } from "@playwright/test";

test.describe("Liquidity", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/markets");
    await page.getByRole("link").first().click();
  });

  test("shows liquidity panel", async ({ page }) => {
    await expect(page.getByText("Liquidity")).toBeVisible();
    await expect(page.getByText("Add")).toBeVisible();
    await expect(page.getByText("Remove")).toBeVisible();
  });

  test("shows pool selection", async ({ page }) => {
    await expect(page.getByText("YES Pool")).toBeVisible();
    await expect(page.getByText("NO Pool")).toBeVisible();
  });

  test("calculates LP tokens", async ({ page }) => {
    // Fill in amount
    const amountInputs = page.getByPlaceholder("0.00");
    await amountInputs.nth(1).fill("50"); // Second input is for liquidity

    await expect(page.getByText(/LP Tokens to Receive/)).toBeVisible();
  });

  test("switches to remove mode", async ({ page }) => {
    await page.getByRole("button", { name: "Remove" }).click();

    await expect(page.getByText("LP Token Amount")).toBeVisible();
  });

  test("full add liquidity flow", async ({ page }) => {
    test.skip(!process.env.TEST_WALLET_CONNECTED, "Requires wallet connection");

    const amountInputs = page.getByPlaceholder("0.00");
    await amountInputs.nth(1).fill("50");

    await page.getByRole("button", { name: "Add Liquidity" }).click();

    await page.waitForSelector('[data-testid="liquidity-success"]', { timeout: 30000 });
  });
});
```

#### Portfolio E2E Test
```typescript
// e2e/portfolio.spec.ts
import { test, expect } from "@playwright/test";

test.describe("Portfolio", () => {
  test("shows connect prompt when disconnected", async ({ page }) => {
    await page.goto("/portfolio");

    await expect(page.getByText("Connect your wallet")).toBeVisible();
  });

  test("shows portfolio sections when connected", async ({ page }) => {
    test.skip(!process.env.TEST_WALLET_CONNECTED, "Requires wallet connection");

    await page.goto("/portfolio");

    await expect(page.getByText("Active Positions")).toBeVisible();
    await expect(page.getByText("LP Positions")).toBeVisible();
  });

  test("shows claimable winnings for resolved markets", async ({ page }) => {
    test.skip(!process.env.TEST_WALLET_CONNECTED, "Requires wallet connection");
    test.skip(!process.env.HAS_WINNING_POSITION, "Requires winning position");

    await page.goto("/portfolio");

    await expect(page.getByText("Claimable Winnings")).toBeVisible();
    await expect(page.getByRole("button", { name: "Claim" })).toBeVisible();
  });
});
```

#### Claim Winnings E2E Test
```typescript
// e2e/claim-winnings.spec.ts
import { test, expect } from "@playwright/test";

test.describe("Claim Winnings", () => {
  test("full claim flow", async ({ page }) => {
    test.skip(!process.env.TEST_WALLET_CONNECTED, "Requires wallet connection");
    test.skip(!process.env.HAS_WINNING_POSITION, "Requires winning position");

    await page.goto("/portfolio");

    // Find claimable section
    await expect(page.getByText("Claimable Winnings")).toBeVisible();

    // Click claim button
    await page.getByRole("button", { name: "Claim" }).first().click();

    // Wait for transaction
    await page.waitForSelector('[data-testid="claim-success"]', { timeout: 30000 });

    // Position should be removed from claimable
    await expect(page.getByText("No claimable winnings")).toBeVisible();
  });
});
```

### 3. Refactor Phase
- [ ] Add visual regression tests
- [ ] Add mobile viewport tests
- [ ] Improve test reliability

## Completion Criteria
- [ ] All E2E tests pass locally
- [ ] Tests work with devnet
- [ ] CI/CD integration configured
- [ ] Test reports generated
- [ ] Operation verified: L1 (functional flows)

## Notes
- Impact scope: Validates complete user journeys
- Constraints: Requires devnet deployment for full testing
- Some tests require wallet connection and test tokens
