# Task: Frontend Component Tests

Metadata:
- Dependencies: task-27 through task-34 (all components)
- Provides: Component test suite
- Size: Medium (3-5 files)

## Implementation Content
Create component tests for core frontend components:
- Navigation component tests
- MarketCard component tests
- TradingPanel component tests
- Form component tests
- Test utilities and mocks

## Target Files
- [ ] `app/__tests__/components/Navigation.test.tsx`
- [ ] `app/__tests__/components/MarketCard.test.tsx`
- [ ] `app/__tests__/components/TradingPanel.test.tsx`
- [ ] `app/__tests__/components/FormComponents.test.tsx`
- [ ] `app/__tests__/utils/test-utils.tsx`

## Implementation Steps (TDD: Red-Green-Refactor)

### 1. Red Phase
- [ ] Set up testing infrastructure
- [ ] Create test utilities and mocks

### 2. Green Phase

#### Test Utilities
```typescript
// app/__tests__/utils/test-utils.tsx
import { ReactNode } from "react";
import { render, RenderOptions } from "@testing-library/react";

// Mock wallet context
const mockWalletContext = {
  wallet: null,
  status: "disconnected" as const,
  connect: vi.fn(),
  disconnect: vi.fn(),
};

const mockConnectedWalletContext = {
  wallet: {
    address: {
      toString: () => "5YNmS1R9nNSCDzb5a7mMJ1dwK9uHeAAF4CmPEwKgVWr8",
    },
  },
  status: "connected" as const,
  connect: vi.fn(),
  disconnect: vi.fn(),
};

// Test providers wrapper
interface TestProvidersProps {
  children: ReactNode;
  walletConnected?: boolean;
}

function TestProviders({ children, walletConnected = false }: TestProvidersProps) {
  const walletContext = walletConnected ? mockConnectedWalletContext : mockWalletContext;

  // Mock the wallet hook
  vi.mock("@solana/react-hooks", () => ({
    useWalletConnection: () => walletContext,
  }));

  return <>{children}</>;
}

// Custom render with providers
const customRender = (
  ui: React.ReactElement,
  options?: Omit<RenderOptions, "wrapper"> & { walletConnected?: boolean }
) => {
  const { walletConnected, ...renderOptions } = options || {};

  return render(ui, {
    wrapper: ({ children }) => (
      <TestProviders walletConnected={walletConnected}>{children}</TestProviders>
    ),
    ...renderOptions,
  });
};

export * from "@testing-library/react";
export { customRender as render, mockWalletContext, mockConnectedWalletContext };
```

#### Navigation Tests
```typescript
// app/__tests__/components/Navigation.test.tsx
import { describe, it, expect, vi } from "vitest";
import { screen, fireEvent } from "@testing-library/react";
import { render } from "../utils/test-utils";
import { Navigation } from "@/components/Navigation";

vi.mock("next/navigation", () => ({
  usePathname: () => "/markets",
}));

describe("Navigation", () => {
  it("renders navigation links", () => {
    render(<Navigation />);

    expect(screen.getByText("Markets")).toBeInTheDocument();
    expect(screen.getByText("Create")).toBeInTheDocument();
    expect(screen.getByText("Portfolio")).toBeInTheDocument();
  });

  it("shows connect button when disconnected", () => {
    render(<Navigation />, { walletConnected: false });

    expect(screen.getByText("Connect Wallet")).toBeInTheDocument();
  });

  it("shows address when connected", () => {
    render(<Navigation />, { walletConnected: true });

    expect(screen.getByText("5YNm...Wr8")).toBeInTheDocument();
  });

  it("shows disconnect button when connected", () => {
    render(<Navigation />, { walletConnected: true });

    expect(screen.getByText("Disconnect")).toBeInTheDocument();
  });

  it("highlights active link", () => {
    render(<Navigation />);

    const marketsLink = screen.getByText("Markets");
    expect(marketsLink).toHaveClass("text-primary");
  });

  it("toggles mobile menu", () => {
    render(<Navigation />);

    // Initially mobile nav should be hidden
    const mobileNav = screen.queryByRole("navigation", { name: /mobile/i });

    // Find and click mobile menu button
    const menuButton = screen.getByRole("button", { name: /menu/i });
    fireEvent.click(menuButton);

    // Mobile nav should now be visible
    expect(screen.getByText("Markets")).toBeVisible();
  });
});
```

#### MarketCard Tests
```typescript
// app/__tests__/components/MarketCard.test.tsx
import { describe, it, expect } from "vitest";
import { screen } from "@testing-library/react";
import { render } from "../utils/test-utils";
import { MarketCard } from "@/components/MarketCard";

const mockMarket = {
  id: 1n,
  question: "Will Bitcoin reach $100k?",
  endTime: BigInt(Math.floor(Date.now() / 1000) + 86400), // 1 day from now
  status: { Open: {} },
  result: { Undecided: {} },
  totalLiquidity: 10000_000000n, // 10,000 USDC
};

describe("MarketCard", () => {
  it("renders market question", () => {
    render(
      <MarketCard
        market={mockMarket}
        yesPrice={0.65}
        noPrice={0.35}
      />
    );

    expect(screen.getByText("Will Bitcoin reach $100k?")).toBeInTheDocument();
  });

  it("displays YES/NO prices as percentages", () => {
    render(
      <MarketCard
        market={mockMarket}
        yesPrice={0.65}
        noPrice={0.35}
      />
    );

    expect(screen.getByText("65%")).toBeInTheDocument();
    expect(screen.getByText("35%")).toBeInTheDocument();
  });

  it("shows Open status badge", () => {
    render(
      <MarketCard
        market={mockMarket}
        yesPrice={0.65}
        noPrice={0.35}
      />
    );

    expect(screen.getByText("Open")).toBeInTheDocument();
  });

  it("shows liquidity amount", () => {
    render(
      <MarketCard
        market={mockMarket}
        yesPrice={0.65}
        noPrice={0.35}
      />
    );

    expect(screen.getByText(/10,000/)).toBeInTheDocument();
  });

  it("shows time remaining for open markets", () => {
    render(
      <MarketCard
        market={mockMarket}
        yesPrice={0.65}
        noPrice={0.35}
      />
    );

    // Should show time remaining format
    expect(screen.getByText(/\d+[hd]/)).toBeInTheDocument();
  });

  it("shows result for resolved markets", () => {
    const resolvedMarket = {
      ...mockMarket,
      status: { Resolved: {} },
      result: { Yes: {} },
    };

    render(
      <MarketCard
        market={resolvedMarket}
        yesPrice={1}
        noPrice={0}
      />
    );

    expect(screen.getByText("YES")).toBeInTheDocument();
  });

  it("links to market detail page", () => {
    render(
      <MarketCard
        market={mockMarket}
        yesPrice={0.65}
        noPrice={0.35}
      />
    );

    const link = screen.getByRole("link");
    expect(link).toHaveAttribute("href", "/markets/1");
  });
});
```

#### TradingPanel Tests
```typescript
// app/__tests__/components/TradingPanel.test.tsx
import { describe, it, expect, vi } from "vitest";
import { screen, fireEvent, waitFor } from "@testing-library/react";
import { render } from "../utils/test-utils";
import { TradingPanel } from "@/components/TradingPanel";

const mockProps = {
  marketId: 1n,
  yesPool: {
    usdcReserve: 5000_000000n,
    tokenReserve: 5000_000000n,
  },
  noPool: {
    usdcReserve: 5000_000000n,
    tokenReserve: 5000_000000n,
  },
};

vi.mock("@/hooks/useTrade", () => ({
  useTrade: () => ({
    trade: vi.fn().mockResolvedValue(true),
    isLoading: false,
    error: null,
  }),
}));

describe("TradingPanel", () => {
  it("renders YES/NO tabs", () => {
    render(<TradingPanel {...mockProps} />, { walletConnected: true });

    expect(screen.getByText("Buy YES")).toBeInTheDocument();
    expect(screen.getByText("Buy NO")).toBeInTheDocument();
  });

  it("switches between YES and NO", () => {
    render(<TradingPanel {...mockProps} />, { walletConnected: true });

    const noTab = screen.getByText("Buy NO");
    fireEvent.click(noTab);

    // NO tab should now be active
    expect(noTab).toHaveClass("bg-red-500");
  });

  it("updates expected output when amount changes", async () => {
    render(<TradingPanel {...mockProps} />, { walletConnected: true });

    const input = screen.getByPlaceholderText("0.00");
    fireEvent.change(input, { target: { value: "100" } });

    await waitFor(() => {
      expect(screen.getByText(/Expected/)).toBeInTheDocument();
    });
  });

  it("shows connect wallet when disconnected", () => {
    render(<TradingPanel {...mockProps} />, { walletConnected: false });

    expect(screen.getByText("Connect Wallet")).toBeInTheDocument();
  });

  it("disables trading when market is closed", () => {
    render(<TradingPanel {...mockProps} disabled />, { walletConnected: true });

    expect(screen.getByText("Market Closed")).toBeInTheDocument();
  });

  it("shows price impact warning for large trades", async () => {
    render(<TradingPanel {...mockProps} />, { walletConnected: true });

    const input = screen.getByPlaceholderText("0.00");
    fireEvent.change(input, { target: { value: "1000" } }); // Large amount

    await waitFor(() => {
      const priceImpact = screen.getByText(/Price Impact/);
      expect(priceImpact).toBeInTheDocument();
    });
  });
});
```

#### Form Components Tests
```typescript
// app/__tests__/components/FormComponents.test.tsx
import { describe, it, expect, vi } from "vitest";
import { screen, fireEvent } from "@testing-library/react";
import { render } from "../utils/test-utils";
import { AmountInput } from "@/components/AmountInput";
import { SlippageSettings } from "@/components/SlippageSettings";

describe("AmountInput", () => {
  it("renders input with placeholder", () => {
    render(<AmountInput value="" onChange={vi.fn()} />);

    expect(screen.getByPlaceholderText("0.00")).toBeInTheDocument();
  });

  it("calls onChange with valid numeric input", () => {
    const onChange = vi.fn();
    render(<AmountInput value="" onChange={onChange} />);

    const input = screen.getByPlaceholderText("0.00");
    fireEvent.change(input, { target: { value: "123.45" } });

    expect(onChange).toHaveBeenCalledWith("123.45");
  });

  it("rejects non-numeric input", () => {
    const onChange = vi.fn();
    render(<AmountInput value="100" onChange={onChange} />);

    const input = screen.getByPlaceholderText("0.00");
    fireEvent.change(input, { target: { value: "abc" } });

    expect(onChange).not.toHaveBeenCalled();
  });

  it("shows MAX button when max is provided", () => {
    render(<AmountInput value="" onChange={vi.fn()} max={1000} />);

    expect(screen.getByText("MAX")).toBeInTheDocument();
  });

  it("sets value to max when MAX clicked", () => {
    const onChange = vi.fn();
    render(<AmountInput value="" onChange={onChange} max={1000} />);

    fireEvent.click(screen.getByText("MAX"));

    expect(onChange).toHaveBeenCalledWith("1000");
  });

  it("shows error message", () => {
    render(<AmountInput value="" onChange={vi.fn()} error="Invalid amount" />);

    expect(screen.getByText("Invalid amount")).toBeInTheDocument();
  });
});

describe("SlippageSettings", () => {
  it("renders current slippage value", () => {
    render(<SlippageSettings value={5} onChange={vi.fn()} />);

    expect(screen.getByText("Slippage: 5%")).toBeInTheDocument();
  });

  it("expands on click", () => {
    render(<SlippageSettings value={5} onChange={vi.fn()} />);

    fireEvent.click(screen.getByText("Slippage: 5%"));

    expect(screen.getByText("1%")).toBeInTheDocument();
    expect(screen.getByText("3%")).toBeInTheDocument();
    expect(screen.getByText("5%")).toBeInTheDocument();
  });

  it("calls onChange when preset clicked", () => {
    const onChange = vi.fn();
    render(<SlippageSettings value={5} onChange={onChange} />);

    fireEvent.click(screen.getByText("Slippage: 5%"));
    fireEvent.click(screen.getByText("1%"));

    expect(onChange).toHaveBeenCalledWith(1);
  });

  it("shows warning for high slippage", () => {
    render(<SlippageSettings value={15} onChange={vi.fn()} />);

    fireEvent.click(screen.getByText("Slippage: 15%"));

    expect(screen.getByText(/High slippage/)).toBeInTheDocument();
  });
});
```

### 3. Refactor Phase
- [ ] Add more edge case tests
- [ ] Improve mock setup
- [ ] Add snapshot tests where appropriate

## Completion Criteria
- [ ] All component tests pass
- [ ] Test coverage meets 80% for components
- [ ] Mocks properly isolate components
- [ ] Tests run in CI
- [ ] Operation verified: L2 (test verification)

## Notes
- Impact scope: Validates all frontend components
- Constraints: Must mock wallet and hooks properly
- Use Vitest and Testing Library
