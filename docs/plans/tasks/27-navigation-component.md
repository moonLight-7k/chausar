# Task: Navigation Component

Metadata:
- Dependencies: task-16 (provider config)
- Provides: app/components/Navigation.tsx
- Size: Small (1 file)

## Implementation Content
Create the main navigation component with:
- Header with app logo/name
- Navigation links (Markets, Create, Portfolio)
- Wallet connect button integration
- Responsive mobile menu

## Target Files
- [ ] `app/components/Navigation.tsx`

## Implementation Steps (TDD: Red-Green-Refactor)

### 1. Red Phase
- [ ] Create component file structure
- [ ] Define navigation link types

### 2. Green Phase
- [ ] Implement navigation component:

```typescript
// app/components/Navigation.tsx
"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";
import { useWalletConnection } from "@solana/react-hooks";
import { useState } from "react";

const navLinks = [
  { href: "/markets", label: "Markets" },
  { href: "/create", label: "Create" },
  { href: "/portfolio", label: "Portfolio" },
];

export function Navigation() {
  const pathname = usePathname();
  const { wallet, status, connect, disconnect } = useWalletConnection();
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false);

  const isActive = (href: string) => pathname === href || pathname?.startsWith(href + "/");

  const truncateAddress = (address: string) => {
    return `${address.slice(0, 4)}...${address.slice(-4)}`;
  };

  return (
    <nav className="border-b border-border-low bg-card">
      <div className="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
        <div className="flex h-16 items-center justify-between">
          {/* Logo */}
          <Link href="/" className="flex items-center gap-2">
            <span className="text-xl font-bold">Chausar</span>
          </Link>

          {/* Desktop Navigation */}
          <div className="hidden sm:flex sm:items-center sm:gap-6">
            {navLinks.map((link) => (
              <Link
                key={link.href}
                href={link.href}
                className={`text-sm font-medium transition-colors ${
                  isActive(link.href)
                    ? "text-primary"
                    : "text-muted hover:text-foreground"
                }`}
              >
                {link.label}
              </Link>
            ))}
          </div>

          {/* Wallet Button */}
          <div className="flex items-center gap-4">
            {status === "connected" && wallet ? (
              <div className="flex items-center gap-2">
                <span className="text-sm text-muted">
                  {truncateAddress(wallet.address.toString())}
                </span>
                <button
                  onClick={() => disconnect()}
                  className="rounded-lg border border-border-low px-3 py-1.5 text-sm font-medium hover:bg-hover"
                >
                  Disconnect
                </button>
              </div>
            ) : (
              <button
                onClick={() => connect()}
                className="rounded-lg bg-primary px-4 py-2 text-sm font-medium text-white hover:bg-primary/90"
              >
                Connect Wallet
              </button>
            )}

            {/* Mobile menu button */}
            <button
              onClick={() => setMobileMenuOpen(!mobileMenuOpen)}
              className="sm:hidden rounded-lg p-2 hover:bg-hover"
            >
              <svg className="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                {mobileMenuOpen ? (
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                ) : (
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 6h16M4 12h16M4 18h16" />
                )}
              </svg>
            </button>
          </div>
        </div>

        {/* Mobile Navigation */}
        {mobileMenuOpen && (
          <div className="sm:hidden py-4 space-y-2">
            {navLinks.map((link) => (
              <Link
                key={link.href}
                href={link.href}
                onClick={() => setMobileMenuOpen(false)}
                className={`block rounded-lg px-3 py-2 text-sm font-medium ${
                  isActive(link.href)
                    ? "bg-primary/10 text-primary"
                    : "text-muted hover:bg-hover"
                }`}
              >
                {link.label}
              </Link>
            ))}
          </div>
        )}
      </div>
    </nav>
  );
}
```

### 3. Refactor Phase
- [ ] Add active link highlighting
- [ ] Improve mobile responsiveness
- [ ] Add animation for mobile menu

## Completion Criteria
- [ ] Navigation renders with all links
- [ ] Wallet connect/disconnect works
- [ ] Mobile menu toggles correctly
- [ ] Build passes
- [ ] Operation verified: L1 (functional component)

## Notes
- Impact scope: Used in app/layout.tsx
- Constraints: Must work with @solana/react-hooks wallet
- Should be added to layout for all pages
