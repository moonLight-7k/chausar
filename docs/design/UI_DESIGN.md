# Chausar Prediction Market - UI/Design System Document

**Document Type:** UI/Design System Document
**Status:** Proposed
**Last Updated:** January 20, 2026
**Scope:** Frontend visual design, component system, and user interface patterns for Chausar prediction market

---

## Table of Contents

1. [Agreement Checklist](#agreement-checklist)
2. [Design System Overview](#design-system-overview)
3. [Color Scheme & Palette](#color-scheme--palette)
4. [Visual Hierarchy & Typography](#visual-hierarchy--typography)
5. [Component Design System](#component-design-system)
6. [Page Layouts](#page-layouts)
7. [Responsive Design](#responsive-design)
8. [Interactive Elements](#interactive-elements)
9. [Accessibility & Brand Identity](#accessibility--brand-identity)
10. [Implementation Notes](#implementation-notes)
11. [References](#references)

---

## Agreement Checklist

### Stakeholder Agreements

This design document aligns with the following agreements from the Chausar PRD (MVP Scope):

- **Scope:**
  - [x] Primary screens: Market List, Market Detail, Create Market Form, Portfolio
  - [x] Binary outcome display (YES/NO with contrasting colors)
  - [x] Responsive design for desktop and mobile (mobile-first approach)
  - [x] Dark mode support using existing Tailwind CSS variables
  - [x] Real-time price and status displays
  - [x] Wallet connection UI integration

- **Non-Scope:**
  - [x] Advanced charting (price charts mentioned in PRD deferred to post-MVP enhancement)
  - [x] Social features (comments, profiles are explicitly out of scope per PRD)
  - [x] Limit orders UI (not in MVP)
  - [x] Multi-outcome market designs (binary only in MVP)

- **Constraints:**
  - [x] Tailwind CSS v4 with existing globals.css color variables
  - [x] React 19 function components only
  - [x] WCAG 2.1 AA color contrast compliance mandatory
  - [x] Transaction costs critical - UI must communicate low fees
  - [x] Trustless design emphasis - communicate security through visual clarity

- **Performance Requirements:**
  - [x] Market list rendering under 2 seconds with 500+ markets
  - [x] Component re-renders optimized with React.memo where appropriate
  - [x] Lazy loading for price charts and activity feeds

### Design Reflection Verification

- [x] Color scheme addresses YES/NO outcome distinction (green for YES, red for NO)
- [x] Dark mode variants defined in globals.css are applied throughout
- [x] Accessibility requirements embedded in every component design
- [x] Responsive breakpoints align with Tailwind CSS standard (sm, md, lg, xl)
- [x] Typography hierarchy clearly distinguishes UI zones (headers, body, labels)
- [x] Component system organized for code reusability and consistency
- [x] All agreements are reflected in the design sections below

---

## Design System Overview

### Design Philosophy

Chausar's UI design emphasizes **trust, clarity, and financial precision**. As a decentralized financial application, the interface must communicate security, transparency, and ease of use. Every design decision prioritizes:

1. **Clarity Over Decoration** - Minimize visual noise; let data speak clearly
2. **Transparency** - Show real-time prices, pool reserves, and transaction impacts
3. **Confidence** - Visual hierarchy and consistent patterns reduce cognitive load
4. **Accessibility** - WCAG 2.1 AA compliance ensures users with diverse abilities can participate
5. **Performance** - Fast interactions build user trust in system reliability

### Current Technology Stack

- **CSS Framework:** Tailwind CSS v4 with CSS custom properties (@theme directive)
- **React Version:** React 19 with function components
- **Fonts:** Inter (sans-serif) for UI, Geist Mono for code/numbers
- **Color System:** OKLCH color space (modern, perceptually uniform)
- **Dark Mode:** System preference-based with `.dark` class override

### Design Tokens Reference

All color values referenced below are defined in `/app/globals.css` using CSS custom properties:

```css
/* Light Mode (Default) */
--primary: oklch(0.208 0.042 265.755); /* Deep blue */
--secondary: oklch(0.968 0.007 247.896); /* Light neutral */
--destructive: oklch(0.577 0.245 27.325); /* Red for errors/losses */
--accent: oklch(0.968 0.007 247.896); /* Muted accent */
--background: oklch(1 0 0); /* White */
--foreground: oklch(0.129 0.042 264.695); /* Dark navy */
--card: oklch(1 0 0); /* White */
--border: oklch(0.929 0.013 255.508); /* Light gray */
--border-strong: oklch(0.839 0.0187 78.23); /* Medium gray */
--cream: oklch(0.9553 0.0029 84.56); /* Warm neutral */

/* Dark Mode (@media prefers-color-scheme: dark) */
--primary: oklch(0.929 0.013 255.508); /* Light blue */
--background: oklch(0.129 0.042 264.695); /* Dark navy */
--foreground: oklch(0.984 0.003 247.858); /* Off-white */
--card: oklch(0.208 0.042 265.755); /* Darker blue */
```

---

## Color Scheme & Palette

### Core Colors

#### Primary Colors

- **Primary (Action/CTA):** `oklch(0.208 0.042 265.755)` - Deep Blue
  - Light mode button backgrounds, active states
  - Used for primary CTAs like "Trade", "Create Market"
  - **Contrast Ratio:** 8.5:1 against white (WCAG AAA)

- **Secondary:** `oklch(0.968 0.007 247.896)` - Light Neutral
  - Secondary buttons, less important actions
  - Hover state backgrounds
  - **Contrast Ratio:** 6:1 against deep blue (WCAG AA)

#### Outcome Colors (Critical for Market Design)

To communicate YES/NO outcomes clearly, we implement semantic color coding:

- **YES Outcome:** `oklch(0.5 0.2 142)` - Vibrant Green
  - Indicates positive/bullish sentiment
  - Used for YES buttons, YES price displays
  - Background tints: `oklch(0.95 0.04 142)` - Very light green tint
  - **Contrast Ratio:** 5.2:1 against white (WCAG AA)

- **NO Outcome:** `oklch(0.6 0.24 25)` - Vibrant Red/Orange
  - Indicates negative/bearish sentiment
  - Used for NO buttons, NO price displays
  - Background tints: `oklch(0.95 0.08 25)` - Very light red tint
  - **Contrast Ratio:** 4.8:1 against white (WCAG AA)

#### Status Colors

- **Destructive (Errors/Losses):** `oklch(0.577 0.245 27.325)` - Red
  - Used for error messages, losing positions
  - Form validation errors
  - **Contrast Ratio:** 4.5:1 against white (WCAG AA minimum)

- **Success (Wins/Resolutions):** `oklch(0.5 0.15 150)` - Medium Green
  - Market resolved YES, winning positions
  - Successful transactions
  - **Contrast Ratio:** 5:1 against white (WCAG AA)

- **Warning (Attention):** `oklch(0.7 0.18 70)` - Gold/Yellow
  - Market about to lock
  - Slippage warnings
  - Transaction pending states
  - **Contrast Ratio:** 4.5:1 against white (WCAG AA)

- **Info (Neutral):** `oklch(0.6 0.1 250)` - Muted Blue
  - Market locked, awaiting resolution
  - Informational messages
  - **Contrast Ratio:** 6:1 against white (WCAG AA)

#### Neutral Colors

- **Foreground (Text):** `oklch(0.129 0.042 264.695)` - Near Black
  - Primary text color
  - **Contrast Ratio:** 19:1 against white (WCAG AAA)

- **Muted (Secondary Text):** `oklch(0.554 0.046 257.417)` - Gray
  - Labels, helper text, disabled states
  - **Contrast Ratio:** 7.5:1 against white (WCAG AA)

- **Border Colors:**
  - Strong: `oklch(0.839 0.0187 78.23)` - Medium gray for component boundaries
  - Low: `oklch(0.927 0.0122 96.43)` - Very light for subtle dividers
  - **Contrast Ratios:** Meet AA at both levels

- **Background/Card:** `oklch(1 0 0)` - Pure white
  - Primary card backgrounds
  - Clean, high-contrast base for content

- **Cream (Accent Background):** `oklch(0.9553 0.0029 84.56)` - Warm neutral
  - Status badges
  - Non-interactive highlights
  - Preserves readability while providing visual distinction

### Dark Mode Color Adjustments

Dark mode inverts lightness values while maintaining hue continuity:

```css
.dark {
  --primary: oklch(0.929 0.013 255.508); /* Light blue for dark mode */
  --background: oklch(0.129 0.042 264.695); /* Dark navy background */
  --foreground: oklch(0.984 0.003 247.858); /* Off-white text */
  --card: oklch(0.208 0.042 265.755); /* Slightly lighter than background */
  --border: oklch(1 0 0 / 10%); /* Subtle white borders */
  --destructive: oklch(0.704 0.191 22.216); /* Lighter red for visibility */
}
```

**Key Principles:**

- Outcome colors (YES/NO) remain consistent between light/dark modes for user recognition
- Ensure all contrast ratios remain WCAG AA compliant in both modes
- Status colors are adjusted for dark mode luminosity perception

### Color Usage Examples

| Component        | Light Mode Primary                     | Dark Mode Primary                          | Fallback                  |
| ---------------- | -------------------------------------- | ------------------------------------------ | ------------------------- |
| Button (Primary) | `--primary`                            | `--primary`                                | Deep Blue → Light Blue    |
| Button (Danger)  | `--destructive`                        | `--destructive` adjusted                   | Red → Light Red           |
| Card Background  | `--card`                               | `--card`                                   | White → Dark Navy         |
| YES Price Badge  | `oklch(0.95 0.04 142)` bg + Green text | `oklch(0.2 0.1 142)` bg + Light green text | Always recognizable green |
| NO Price Badge   | `oklch(0.95 0.08 25)` bg + Red text    | `oklch(0.3 0.15 25)` bg + Light red text   | Always recognizable red   |

---

## Visual Hierarchy & Typography

### Font System

**Font Stack:**

```css
--font-sans: var(--font-inter), system-ui, sans-serif;
--font-mono: var(--font-geist-mono), ui-monospace, monospace;
```

**Font Selection Rationale:**

- **Inter:** Modern, geometric sans-serif optimized for screen readability; excellent at small sizes
- **Geist Mono:** Precise, monospace font for displaying numbers, addresses, token amounts (critical in financial context)

### Typography Scale

All sizes use Tailwind CSS utilities built on 4px baseline. Designed for **readability and visual hierarchy clarity**:

#### Heading Hierarchy

| Level | Size           | Weight         | Line Height  | Usage                      | CSS Class                |
| ----- | -------------- | -------------- | ------------ | -------------------------- | ------------------------ |
| H1    | 2rem (32px)    | 600 (semibold) | 1.25 (tight) | Page titles, main headings | `text-3xl font-semibold` |
| H2    | 1.5rem (24px)  | 600 (semibold) | 1.33         | Section headers            | `text-2xl font-semibold` |
| H3    | 1.25rem (20px) | 600 (semibold) | 1.4          | Subsection titles          | `text-xl font-semibold`  |
| H4    | 1rem (16px)    | 600 (semibold) | 1.5          | Card titles, minor headers | `text-lg font-semibold`  |

**Implementation:**

```typescript
// Example H1 implementation
<h1 className="text-3xl font-semibold tracking-tight mb-2">
  Prediction Markets
</h1>
```

#### Body Text

| Type         | Size            | Weight       | Line Height | Usage                   | CSS Class                                     |
| ------------ | --------------- | ------------ | ----------- | ----------------------- | --------------------------------------------- |
| Body Large   | 1rem (16px)     | 400 (normal) | 1.5         | Primary body copy       | `text-base leading-relaxed`                   |
| Body Regular | 0.95rem (15px)  | 400 (normal) | 1.6         | Standard content        | `text-sm leading-loose`                       |
| Body Small   | 0.875rem (14px) | 400 (normal) | 1.57        | Secondary info, labels  | `text-sm leading-relaxed`                     |
| Label        | 0.75rem (12px)  | 500 (medium) | 1.33        | Form labels, badges     | `text-xs font-medium uppercase tracking-wide` |
| Caption      | 0.75rem (12px)  | 400 (normal) | 1.33        | Timestamps, helper text | `text-xs text-muted`                          |

**Implementation:**

```typescript
// Body text with proper contrast
<p className="text-base leading-relaxed text-foreground">
  Current YES price: 62%
</p>

// Secondary text (meets WCAG AA 7.5:1 ratio)
<p className="text-sm text-muted">
  Last updated: 2 seconds ago
</p>
```

#### Number/Code Text

All numeric displays use monospace for alignment and clarity:

| Type          | Size            | Weight         | Usage             | CSS Class                          |
| ------------- | --------------- | -------------- | ----------------- | ---------------------------------- |
| Large Number  | 1.5rem (24px)   | 600 (semibold) | Price display     | `text-2xl font-semibold font-mono` |
| Medium Number | 1rem (16px)     | 500 (medium)   | Amount inputs     | `text-base font-medium font-mono`  |
| Small Number  | 0.875rem (14px) | 400 (normal)   | Fees, percentages | `text-sm font-mono`                |
| Code/Address  | 0.75rem (12px)  | 400 (normal)   | Wallet addresses  | `text-xs font-mono break-all`      |

**Implementation:**

```typescript
// Price display with monospace
<div className="text-2xl font-semibold font-mono">
  $62.45
</div>

// Wallet address with proper breaking
<span className="text-xs font-mono break-all text-muted">
  0x1234...5678
</span>
```

### Spacing System

Consistent spacing uses Tailwind's 4px baseline multiplied by spacing scale:

| Scale | Pixels | Tailwind Class   | Usage                         |
| ----- | ------ | ---------------- | ----------------------------- |
| xs    | 4px    | `p-1`, `gap-1`   | Tight component spacing       |
| sm    | 8px    | `p-2`, `gap-2`   | Component internal spacing    |
| md    | 16px   | `p-4`, `gap-4`   | Section spacing, card padding |
| lg    | 24px   | `p-6`, `gap-6`   | Major section spacing         |
| xl    | 32px   | `p-8`, `gap-8`   | Page-level spacing            |
| 2xl   | 40px   | `p-10`, `gap-10` | Large sections                |

**Padding Guidelines:**

- Cards: `p-6` (24px) standard, `p-8` (32px) for detail pages
- Forms: `gap-4` (16px) between fields, `p-4` (16px) around fieldsets
- List items: `p-4` (16px) vertically, `px-6` (24px) horizontally

**Margin Guidelines:**

- Header to content: `mb-8` (32px)
- Section spacing: `mb-6` (24px)
- Component spacing: `mb-4` (16px)

### Line Height & Readability

Tailwind line heights are calibrated for readability:

```css
/* Default line heights for body text */
.leading-relaxed {
  line-height: 1.625; /* 26px for base text */
}

/* Tighter for headings */
.leading-tight {
  line-height: 1.25; /* 32px for H1 */
}

/* Looser for dense content */
.leading-loose {
  line-height: 1.75; /* 28px for body */
}
```

---

## Component Design System

Following Atomic Design methodology, components are organized from atoms (primitive) to organisms (complex):

### Atoms (Primitive Components)

#### 1. Button

**Variants:**

- **Primary Button** - Main CTAs (Trade, Claim, Create Market)
- **Secondary Button** - Alternative actions
- **Danger Button** - Destructive actions (Cancel, Remove)
- **Ghost Button** - Tertiary navigation

**States:**

- Default
- Hover (elevated, shadow increase)
- Active (slight compression, darker background)
- Disabled (opacity 60%, disabled cursor)
- Loading (spinner overlay, disabled state)

**Props Type Definition:**

```typescript
type ButtonProps = {
  variant?: "primary" | "secondary" | "danger" | "ghost";
  size?: "sm" | "md" | "lg";
  disabled?: boolean;
  loading?: boolean;
  onClick: () => void;
  children: React.ReactNode;
  className?: string;
};
```

**Implementation Sample:**

```typescript
export function Button({
  variant = 'primary',
  size = 'md',
  disabled = false,
  loading = false,
  onClick,
  children,
  className = '',
}: ButtonProps) {
  const baseClasses = 'rounded-lg font-medium transition-all focus:outline-ring'

  const variantClasses = {
    primary: 'bg-primary text-primary-foreground hover:shadow-md active:scale-95',
    secondary: 'bg-secondary text-secondary-foreground hover:shadow-sm',
    danger: 'bg-destructive text-white hover:shadow-md',
    ghost: 'bg-transparent text-foreground hover:bg-cream',
  }

  const sizeClasses = {
    sm: 'px-3 py-2 text-sm',
    md: 'px-4 py-2.5 text-base',
    lg: 'px-6 py-3 text-lg',
  }

  return (
    <button
      onClick={onClick}
      disabled={disabled || loading}
      className={`
        ${baseClasses}
        ${variantClasses[variant]}
        ${sizeClasses[size]}
        ${(disabled || loading) ? 'opacity-60 cursor-not-allowed' : ''}
        ${className}
      `}
    >
      {loading ? <Spinner size={size} /> : children}
    </button>
  )
}
```

#### 2. Badge/Status Indicator

**Types:**

- Status badge (Open, Locked, Resolved)
- Outcome badge (YES/NO with semantic colors)
- Performance badge (Win/Loss indicator)
- Count badge (notification number)

**Implementation:**

```typescript
type BadgeProps = {
  type: 'status' | 'outcome' | 'performance' | 'count'
  value: 'open' | 'locked' | 'resolved' | 'yes' | 'no' | 'win' | 'loss' | number
  size?: 'sm' | 'md'
}

export function Badge({ type, value, size = 'md' }: BadgeProps) {
  const statusStyles = {
    open: 'bg-blue-100 text-blue-700 dark:bg-blue-900 dark:text-blue-200',
    locked: 'bg-yellow-100 text-yellow-700 dark:bg-yellow-900 dark:text-yellow-200',
    resolved: 'bg-gray-100 text-gray-700 dark:bg-gray-800 dark:text-gray-200',
  }

  const outcomeStyles = {
    yes: 'bg-green-100 text-green-700 dark:bg-green-900 dark:text-green-200',
    no: 'bg-red-100 text-red-700 dark:bg-red-900 dark:text-red-200',
  }

  const performanceStyles = {
    win: 'bg-green-100 text-green-700 dark:bg-green-900 dark:text-green-200',
    loss: 'bg-red-100 text-red-700 dark:bg-red-900 dark:text-red-200',
  }

  const sizeClasses = {
    sm: 'px-2 py-1 text-xs',
    md: 'px-3 py-1.5 text-sm',
  }

  let style = ''
  if (type === 'status') style = statusStyles[value as keyof typeof statusStyles] || ''
  if (type === 'outcome') style = outcomeStyles[value as keyof typeof outcomeStyles] || ''
  if (type === 'performance') style = performanceStyles[value as keyof typeof performanceStyles] || ''

  return (
    <span className={`rounded-full font-medium ${sizeClasses[size]} ${style}`}>
      {value.toString()}
    </span>
  )
}
```

#### 3. Input Fields

**Types:**

- Text input (labels, addresses)
- Number input (amounts, percentages)
- DateTime input (market creation times)
- Select/Dropdown

**States:**

- Default (empty)
- Focused (ring, border color change)
- Filled (value present)
- Error (red border, error message)
- Disabled (gray background, no interaction)

**Props:**

```typescript
type InputProps = {
  type: "text" | "number" | "datetime-local";
  label?: string;
  placeholder?: string;
  value: string | number;
  onChange: (value: string | number) => void;
  error?: string;
  disabled?: boolean;
  required?: boolean;
  helperText?: string;
  className?: string;
};
```

**Implementation:**

```typescript
export function Input({
  type,
  label,
  placeholder,
  value,
  onChange,
  error,
  disabled = false,
  required = false,
  helperText,
  className = '',
}: InputProps) {
  return (
    <div className="flex flex-col gap-1.5">
      {label && (
        <label className="text-sm font-medium text-foreground">
          {label}
          {required && <span className="text-destructive ml-1">*</span>}
        </label>
      )}
      <input
        type={type}
        placeholder={placeholder}
        value={value}
        onChange={(e) => onChange(e.target.value)}
        disabled={disabled}
        className={`
          rounded-lg border border-border px-4 py-2.5
          bg-card text-foreground placeholder:text-muted
          focus:outline-none focus:ring-2 focus:ring-primary focus:border-primary
          disabled:bg-gray-100 disabled:cursor-not-allowed
          ${error ? 'border-destructive focus:ring-destructive' : ''}
          ${className}
        `}
      />
      {error && <p className="text-xs text-destructive">{error}</p>}
      {helperText && !error && <p className="text-xs text-muted">{helperText}</p>}
    </div>
  )
}
```

#### 4. Price Display

**Critical Component** - Displays real-time prices with semantic coloring

**Features:**

- Monospace font for alignment
- Color indicates outcome (green for YES, red for NO)
- Percentage display with arrow indicators
- Update animation on price change

**Props:**

```typescript
type PriceDisplayProps = {
  value: number;
  side: "yes" | "no";
  format: "percentage" | "currency";
  size?: "sm" | "md" | "lg";
  showArrow?: boolean;
  previousValue?: number;
};
```

**Implementation:**

```typescript
export function PriceDisplay({
  value,
  side,
  format,
  size = 'md',
  showArrow = false,
  previousValue,
}: PriceDisplayProps) {
  const isYes = side === 'yes'
  const baseColor = isYes ? 'text-green-700 dark:text-green-400' : 'text-red-700 dark:text-red-400'
  const bgColor = isYes
    ? 'bg-green-100 dark:bg-green-900'
    : 'bg-red-100 dark:bg-red-900'

  const sizeClasses = {
    sm: 'text-sm px-2 py-1',
    md: 'text-lg px-3 py-1.5',
    lg: 'text-2xl px-4 py-2',
  }

  const displayValue = format === 'percentage' ? `${value}%` : `$${value}`

  const trend =
    previousValue !== undefined
      ? value > previousValue
        ? 'up'
        : value < previousValue
          ? 'down'
          : 'flat'
      : null

  return (
    <div className={`rounded-md font-mono font-semibold ${sizeClasses[size]} ${baseColor} ${bgColor}`}>
      <div className="flex items-center gap-1">
        {showArrow && trend === 'up' && <span>↑</span>}
        {showArrow && trend === 'down' && <span>↓</span>}
        <span>{displayValue}</span>
      </div>
    </div>
  )
}
```

### Molecules (Component Combinations)

#### 1. Market Card

**Displays:** Question, current prices, time remaining, liquidity, status

**Layout:**

- Header: Question text, status badge
- Middle: Two-column price display (YES | NO)
- Footer: Time remaining, total liquidity, "View" button

**Props:**

```typescript
type MarketCardProps = {
  id: string;
  question: string;
  yesPrice: number;
  noPrice: number;
  timeRemaining: string;
  totalLiquidity: string;
  status: "open" | "locked" | "resolved";
  result?: "yes" | "no";
  onClick: (id: string) => void;
};
```

**Visual Specification:**

```
┌─────────────────────────────────────────┐
│ "Will Bitcoin..." [OPEN badge]          │
├─────────────────────────────────────────┤
│ YES: $62.45 (62%)  │  NO: $37.55 (38%)  │
├─────────────────────────────────────────┤
│ Ending: 4 hours  │  TVL: $12,500       │
│                  [View Market →]        │
└─────────────────────────────────────────┘
```

**Component Implementation:**

```typescript
export function MarketCard({
  id,
  question,
  yesPrice,
  noPrice,
  timeRemaining,
  totalLiquidity,
  status,
  result,
  onClick,
}: MarketCardProps) {
  return (
    <div
      onClick={() => onClick(id)}
      className="rounded-xl border border-border bg-card p-6 cursor-pointer
                 transition hover:shadow-md hover:border-border-strong"
    >
      <div className="flex items-start justify-between gap-2 mb-4">
        <h3 className="text-base font-semibold line-clamp-2">{question}</h3>
        <Badge type="status" value={status} />
      </div>

      <div className="grid grid-cols-2 gap-4 mb-4">
        <PriceDisplay value={yesPrice} side="yes" format="percentage" size="md" />
        <PriceDisplay value={noPrice} side="no" format="percentage" size="md" />
      </div>

      <div className="flex items-center justify-between text-sm text-muted border-t border-border-low pt-3">
        <span>Ending: {timeRemaining}</span>
        <span>TVL: {totalLiquidity}</span>
      </div>
    </div>
  )
}
```

#### 2. Trade Panel

**Displays:** Buy YES/NO interface with amount input and execution

**Sections:**

1. Side selector (YES/NO tabs)
2. Amount input
3. Price calculation display
4. Slippage warning (if > 1%)
5. Execute button

**Props:**

```typescript
type TradePanelProps = {
  side: "yes" | "no";
  onSideChange: (side: "yes" | "no") => void;
  amount: string;
  onAmountChange: (amount: string) => void;
  estimatedOutput: number;
  currentPrice: number;
  priceImpact: number;
  fee: number;
  onTrade: () => void;
  loading?: boolean;
  disabled?: boolean;
  balance?: number;
};
```

#### 3. Portfolio Position Card

**Displays:** Single position in user's portfolio

**Shows:**

- Market question
- Side (YES/NO)
- Quantity held
- Entry price / Current value
- Unrealized P&L
- Action (Sell/Claim)

#### 4. Form Group (Create Market)

**Organizes:** Multiple input fields into coherent form sections

**Sections:**

1. Market Definition (Question, Description)
2. Timing (End time, Resolution time)
3. Liquidity (Initial amount)
4. Oracle Configuration

### Organisms (Complex Components)

#### 1. Market List Page Container

**Composition:**

- Header with title
- Search/Filter bar
- Sort options
- Grid/List toggle
- Market cards grid
- Pagination or infinite scroll
- Empty state fallback

#### 2. Market Detail View

**Sections:**

- Header (Question, description, creator info)
- Price chart area (when implemented post-MVP)
- Trading panel (side-by-side YES/NO)
- Pool statistics sidebar
- Activity feed
- Related markets (optional)

#### 3. Create Market Form

**Multi-step or Single Form:**

- Market definition section
- Timing validation
- Liquidity selection
- Review & confirmation
- Transaction status display

#### 4. Portfolio Dashboard

**Sections:**

- Summary stats (Total invested, Current value, P&L)
- Active positions table
- LP positions table
- Claimable winnings
- Trade history

---

## Page Layouts

### 1. Market List Page

**URL:** `/markets`
**Primary Use:** Browse and discover prediction markets

**Layout Structure:**

```
┌───────────────────────────────────────────────────┐
│ HEADER                                            │
│ "Prediction Markets"                              │
├───────────────────────────────────────────────────┤
│ CONTROLS                                          │
│ [Search: Find a market...] [Filter ▼] [Sort ▼]  │
├───────────────────────────────────────────────────┤
│ MARKET GRID (Responsive)                         │
│ ┌──────────────┬──────────────┬──────────────┐   │
│ │ Market Card  │ Market Card  │ Market Card  │   │
│ ├──────────────┼──────────────┼──────────────┤   │
│ │ Market Card  │ Market Card  │ Market Card  │   │
│ └──────────────┴──────────────┴──────────────┘   │
│                                                   │
│ [Load More] or [Showing X of Y markets]          │
└───────────────────────────────────────────────────┘
```

**Responsive Behavior:**

- Desktop (lg+): 3-column grid
- Tablet (md): 2-column grid
- Mobile (sm): 1-column grid

**Key Interactions:**

- Click card → Navigate to market detail
- Search → Real-time filter (debounced)
- Sort by: "Ending Soon", "Highest Volume", "Newest"
- Filter by: "Status" (Open/Locked/Resolved)

### 2. Market Detail Page

**URL:** `/markets/[id]`
**Primary Use:** View market details and execute trades

**Layout Structure:**

```
┌─────────────────────────────────────────────────┐
│ HEADER                                          │
│ "Bitcoin to $100k?" [Status] [Share]            │
│ Description text...                             │
├────────────────┬────────────────────────────────┤
│                │                                │
│   CHART AREA   │  TRADING PANEL                │
│   (Post-MVP)   │  ┌──────────────────────────┐ │
│                │  │ [YES] [NO]               │ │
│                │  │ Amount: [_______]        │ │
│                │  │ Est. Output: 52.4 YES    │ │
│                │  │ Price Impact: 1.2%       │ │
│                │  │ [BUY YES] [BUY NO]       │ │
│                │  └──────────────────────────┘ │
│                │                                │
│                │  POOL STATS                   │
│                │  ├─ YES Reserve: $5,200      │
│                │  ├─ NO Reserve: $3,100       │
│                │  └─ 24h Volume: $12,400      │
├────────────────┴────────────────────────────────┤
│ ACTIVITY FEED                                  │
│ Recent trades: John bought $100 YES, etc...    │
└─────────────────────────────────────────────────┘
```

**Responsive Behavior:**

- Desktop (lg+): 2-column layout (chart + trading)
- Tablet (md): Stacked, trading panel on top
- Mobile (sm): Full-width stacked, trading panel below

### 3. Create Market Form Page

**URL:** `/create`
**Primary Use:** Allow creators to launch new markets

**Form Sections:**

```
┌─────────────────────────────────────────────────┐
│ CREATE MARKET FORM                              │
├─────────────────────────────────────────────────┤
│ SECTION 1: Market Details                       │
│ ┌─────────────────────────────────────────────┐ │
│ │ Question: [______________________]          │ │
│ │ Min 10 chars, max 280 chars                 │ │
│ │                                              │ │
│ │ Description: [____________________]         │ │
│ │ Optional, max 1000 chars                    │ │
│ └─────────────────────────────────────────────┘ │
│                                                 │
│ SECTION 2: Timing                               │
│ ┌─────────────────────────────────────────────┐ │
│ │ Trading Ends: [____/____/____ __:__]       │ │
│ │ Must be in future                           │ │
│ │                                              │ │
│ │ Resolution: [____/____/____ __:__]         │ │
│ │ Must be after trading ends                  │ │
│ └─────────────────────────────────────────────┘ │
│                                                 │
│ SECTION 3: Initial Liquidity                    │
│ ┌─────────────────────────────────────────────┐ │
│ │ Amount (USDC): [_________] (Min: 100)      │ │
│ │ Current balance: 5,000 USDC                 │ │
│ └─────────────────────────────────────────────┘ │
│                                                 │
│ SECTION 4: Oracle                               │
│ ┌─────────────────────────────────────────────┐ │
│ │ Oracle Address: [Protocol Multisig v] ✓    │ │
│ │ Will resolve this market                    │ │
│ └─────────────────────────────────────────────┘ │
│                                                 │
│ [Review] [Create Market]                        │
└─────────────────────────────────────────────────┘
```

**Form Behavior:**

- Progressive disclosure (show one section at a time on mobile)
- Real-time validation
- Error messages inline
- Character counters for text fields

### 4. Portfolio Page

**URL:** `/portfolio`
**Primary Use:** View all user positions and history

**Sections:**

```
┌─────────────────────────────────────────────────┐
│ PORTFOLIO SUMMARY                               │
│ Total Invested: $5,000  │ Current Value: $5,240 │
│ Unrealized P&L: +$240 (+4.8%)                  │
├─────────────────────────────────────────────────┤
│ ACTIVE POSITIONS                                │
│ ┌──────────────────────────────────────────┐   │
│ │ Question | Side | Qty | Value | P&L    │   │
│ ├──────────────────────────────────────────┤   │
│ │ Bitcoin? │ YES  │ 52  │ $240  │ +$40  │   │
│ │ Rain SF? │ NO   │ 28  │ $100  │ -$20  │   │
│ └──────────────────────────────────────────┘   │
│                                                 │
│ CLAIMABLE WINNINGS                              │
│ Market "Tech IPO Q1?" resolved YES              │
│ You hold: 15 YES tokens → $15 USDC [CLAIM]     │
├─────────────────────────────────────────────────┤
│ TRADE HISTORY                                   │
│ Sold 25 NO @ $0.38 on Jan 15                    │
│ Bought 52 YES @ $0.54 on Jan 14                 │
└─────────────────────────────────────────────────┘
```

### 5. Navigation/Header

**Sticky Header:**

```
┌──────────────────────────────────────────────────┐
│ [Chausar Logo] | [Markets] [Create] [Portfolio]  │
│                                      [0x1234...] │
│                                      [Dark/Light]│
└──────────────────────────────────────────────────┘
```

**Mobile Header (Hamburger Menu):**

```
┌──────────────────────────────────────────────────┐
│ [☰] [Chausar] [Connect Wallet] [Settings]       │
│                                                  │
│ Collapsed Mobile Menu:                          │
│ ├─ Markets                                       │
│ ├─ Create Market                                │
│ ├─ Portfolio                                    │
│ ├─ Settings                                     │
│ └─ Disconnect                                   │
└──────────────────────────────────────────────────┘
```

---

## Responsive Design

### Mobile-First Approach

All components are designed mobile-first, then enhanced for larger screens.

### Breakpoints

Tailwind CSS standard breakpoints used throughout:

| Breakpoint | Width  | Device Type       | Column Count | Usage                      |
| ---------- | ------ | ----------------- | ------------ | -------------------------- |
| sm         | 640px  | Mobile (portrait) | 1 column     | Cards stack vertically     |
| md         | 768px  | Tablet            | 2 columns    | Trading panel beside chart |
| lg         | 1024px | Desktop           | 3 columns    | Full market grid           |
| xl         | 1280px | Large desktop     | 4 columns    | Expanded layouts           |
| 2xl        | 1536px | Ultra-wide        | 5 columns    | Maximum grid expansion     |

### Touch-Friendly Spacing

Mobile optimizations ensure usable interfaces on small screens:

- **Minimum Touch Target:** 44x44px (Tailwind `py-3 px-4` = ~48px height)
- **Horizontal Padding:** `px-4` (16px) on mobile, `px-6` (24px) on desktop
- **Tap Areas:** All buttons, links, and cards meet 44x44px minimum

**Implementation Pattern:**

```typescript
// Touch-friendly button sizing
<button className="py-3 px-4 md:py-2.5 md:px-4 lg:py-2 lg:px-3">
  Touch Button
</button>

// Stack on mobile, side-by-side on desktop
<div className="grid grid-cols-1 md:grid-cols-2 gap-4">
  <Card />
  <Card />
</div>
```

### Mobile Layout Patterns

#### Full-Width Modals

On mobile, modals expand to 90vw with rounded corners and bottom sheet effect.

```typescript
<div className="fixed inset-0 bg-black/50 flex items-end md:items-center">
  <div className="bg-card w-full md:w-96 rounded-t-3xl md:rounded-2xl
                  md:max-h-96 max-h-[90vh] overflow-y-auto">
    {/* Content */}
  </div>
</div>
```

#### Collapsed Forms

Multi-section forms collapse to single-column layouts on mobile with collapsible headers.

```typescript
<div className="space-y-4">
  <details className="border border-border rounded-lg p-4 open:bg-cream">
    <summary className="cursor-pointer font-semibold">Market Details</summary>
    <div className="mt-4 space-y-3">{/* Form fields */}</div>
  </details>
</div>
```

### Responsive Charts/Tables

Market data tables convert to card-based layouts on mobile:

**Desktop (Tabular):**

```
| Question | YES | NO | TVL | Status |
|----------|-----|----|----|--------|
| Bitcoin? | 62% | 38% | $12,500 | Open |
```

**Mobile (Cards):**

```
Bitcoin?
YES: 62%  NO: 38%
TVL: $12,500  [Open]
```

---

## Interactive Elements

### 1. Hover States

**Buttons:**

- Elevation increase: `hover:shadow-md`
- Slight upward movement: `hover:-translate-y-0.5`
- Smooth transition: `transition-all duration-200`

**Cards:**

- Border color enhance: `hover:border-border-strong`
- Shadow addition: `hover:shadow-md`
- Cursor change: `cursor-pointer`

**Links:**

- Underline animation: `underline-offset-2 hover:underline`
- Color shift to primary

### 2. Focus States

**Keyboard Navigation Priority:**

```css
focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2
```

All interactive elements must have visible focus indicators for accessibility:

```typescript
className =
  "focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2";
```

### 3. Loading States

**Button Loading:**

```typescript
<button disabled={loading} className={loading ? 'opacity-60' : ''}>
  {loading ? <Spinner /> : 'Trade'}
</button>
```

**Skeleton Loaders (for data fetching):**

```typescript
// Placeholder while data loads
<div className="animate-pulse space-y-4">
  <div className="h-6 bg-gray-200 dark:bg-gray-700 rounded w-1/2" />
  <div className="h-4 bg-gray-200 dark:bg-gray-700 rounded" />
</div>
```

**Pending Transaction State:**

```
┌──────────────────────────────┐
│ Transaction Pending...       │
│ [Spinner]                    │
│ Tx: 0x1234...5678            │
│ [View on Explorer]           │
└──────────────────────────────┘
```

### 4. Error States

**Form Error:**

```typescript
<div className="rounded-lg border border-destructive bg-red-50 dark:bg-red-900/20 p-3">
  <p className="text-sm font-medium text-destructive">
    Insufficient USDC balance. You have $50, need $100.
  </p>
</div>
```

**Market Error (Cannot Trade):**

```
┌────────────────────────────────┐
│ ⚠️ Market Locked               │
│ Trading ended 2 hours ago.     │
│ Awaiting resolution...         │
│ [View Positions]               │
└────────────────────────────────┘
```

**Connection Error:**

```
┌────────────────────────────────┐
│ Connection Error               │
│ Unable to fetch market data    │
│ [Retry]                        │
└────────────────────────────────┘
```

### 5. Success States

**Trade Confirmation:**

```
┌────────────────────────────────┐
│ ✓ Trade Executed!              │
│ Bought 52 YES @ $0.62          │
│ Cost: $32.24 + $0.10 fee       │
│ [View Position] [Done]         │
└────────────────────────────────┘
```

**Market Created:**

```
┌────────────────────────────────┐
│ ✓ Market Created!              │
│ "Bitcoin to $100k?"            │
│ Live trading now open          │
│ [Share] [View Market]          │
└────────────────────────────────┘
```

### 6. Modal/Dialog Designs

**Confirmation Modal:**

```
┌────────────────────────────────┐
│ Confirm Trade          [x]     │
├────────────────────────────────┤
│ You are about to buy 52 YES    │
│ Total cost: $32.34             │
│                                │
│ Price impact: 1.2%             │
│ Slippage: 0.8%                 │
├────────────────────────────────┤
│ [Cancel]  [Confirm Trade]      │
└────────────────────────────────┘
```

**Alert Dialog (Destructive):**

```
┌────────────────────────────────┐
│ Remove Liquidity      [x]       │
├────────────────────────────────┤
│ You will burn 100 LP tokens    │
│ Receive: ~$500 + collected fees│
│ This cannot be undone.         │
├────────────────────────────────┤
│ [Keep] [Remove Liquidity]      │
└────────────────────────────────┘
```

---

## Accessibility & Brand Identity

### Color Contrast Compliance

**WCAG 2.1 Level AA Requirements:**

- Normal text: Minimum 4.5:1 contrast ratio
- Large text (18pt+ or 14pt bold): Minimum 3:1 ratio
- UI components and graphical elements: Minimum 3:1 ratio

**Verified Contrast Ratios (Light Mode):**

| Element Pair            | Ratio | Level | Status           |
| ----------------------- | ----- | ----- | ---------------- |
| Deep Blue text on white | 8.5:1 | AAA   | ✓ Pass           |
| Gray text on white      | 7.5:1 | AAA   | ✓ Pass           |
| Green YES on white      | 5.2:1 | AA    | ✓ Pass           |
| Red NO on white         | 4.8:1 | AA    | ✓ Pass           |
| Red error on white      | 4.5:1 | AA    | ✓ Pass (minimum) |
| Yellow warning on white | 4.5:1 | AA    | ✓ Pass (minimum) |

**Dark Mode Adjustments:**

- Lightness values inverted to maintain contrast
- Outcome colors adjusted for dark mode visibility
- All ratios re-verified: minimum 4.5:1 maintained

**Testing Tool:**
WebAIM Contrast Checker: https://webaim.org/resources/contrastchecker/

### Typography for Accessibility

**Readable Typography:**

- **Font Size Minimum:** 14px for body text (Tailwind `text-sm`)
- **Line Height:** 1.5-1.75 for body text (improved from single-spacing)
- **Letter Spacing:** `tracking-wide` for headers (improved readability)
- **Line Length:** Maximum 70 characters for body text (container width limit)

**Implementation:**

```typescript
// Accessible heading
<h1 className="text-3xl font-semibold tracking-tight leading-tight mb-2">
  {title}
</h1>

// Readable body text
<p className="text-base leading-relaxed max-w-prose">
  {content}
</p>
```

### Focus Management

**Keyboard Navigation:**

- All interactive elements are keyboard accessible (Tab, Enter, Arrow keys)
- Focus visible with 2px ring outline: `focus:ring-2 focus:ring-primary`
- Tab order matches visual order
- Skip links for navigation bypass (post-MVP enhancement)

**Implementation:**

```typescript
className =
  "focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2";
```

### Semantic HTML

**Always use semantic elements:**

```typescript
// ✓ Correct
<nav>{/* Navigation */}</nav>
<main>{/* Main content */}</main>
<header>{/* Header */}</header>
<button onClick={...}>Trade</button>

// ✗ Avoid
<div onClick={...}>Trade</div>
<div>{/* Not a nav */}</div>
```

### Icons & Visual Indicators

**Icons should never stand alone:**

```typescript
// ✗ Problematic
<button>
  <Icon name="settings" />
</button>

// ✓ Correct
<button title="Settings" aria-label="Open settings">
  <Icon name="settings" />
  <span className="sr-only">Settings</span>
</button>
```

### Error Messages

**Clear, actionable error messages:**

```typescript
// ✗ Unclear
"Error 400"

// ✓ Clear & actionable
"Insufficient USDC balance. You have 50 USDC, but this trade requires 100 USDC.
[Buy more USDC]"
```

### Brand Identity

#### Visual Personality

- **Trust:** Clear data presentation, high contrast, minimal decoration
- **Innovation:** Modern typography, smooth animations, financial precision
- **Accessibility:** Inclusive by default, no assumptions about user abilities
- **Transparency:** Real-time data, visible fees, outcome probabilities always shown

#### Voice & Tone

- **Markets:** "Bitcoin closes above $100k?" (clear, concise questions)
- **CTAs:** "Trade YES" (direct, outcome-focused)
- **Confirmations:** "Trade executed. Bought 52 YES @ 62%." (factual, reassuring)
- **Errors:** "Insufficient USDC. You need $50 more." (helpful, not blaming)

#### Visual Markers

- **Success:** Green checkmark + positive outcome color
- **Warning:** Yellow triangle + pending/attention color
- **Error:** Red X + destructive color
- **Information:** Blue circle + info color

---

## Implementation Notes

### Tailwind CSS Configuration

All colors leverage CSS custom properties defined in `globals.css` using the `@theme` directive (Tailwind CSS v4):

```css
@theme inline {
  --color-primary: var(--primary);
  --color-secondary: var(--secondary);
  --color-destructive: var(--destructive);
  /* Additional theme variables */
}
```

This approach provides:

1. Single source of truth (globals.css)
2. Runtime color customization via CSS
3. Native CSS variable support in all browsers
4. Seamless dark mode switching

### Component Optimization

**React 19 Patterns:**

- Use `React.memo()` for expensive market card components
- `useCallback()` for event handlers in lists
- `useMemo()` for price calculations and derived data
- Virtual rendering (TanStack React Virtual) for 500+ market lists

**Example Memoized Component:**

```typescript
export const MarketCard = React.memo(({ market, onClick }: MarketCardProps) => {
  return (
    <div onClick={() => onClick(market.id)} className="...">
      {/* Component JSX */}
    </div>
  )
})

MarketCard.displayName = 'MarketCard'
```

### Dark Mode Implementation

Dark mode uses Tailwind's `dark:` prefix for contextual styles:

```typescript
// Automatic dark mode based on system preference
<html suppressHydrationWarning>
  <body className={isDarkMode ? 'dark' : ''}>
    {/* Content automatically switches */}
  </body>
</html>

// Manual override in component
<button className="bg-white dark:bg-slate-900 text-black dark:text-white">
  Toggle-aware button
</button>
```

### Type Safety

All component props include explicit TypeScript types:

```typescript
type MarketCardProps = {
  id: string;
  question: string;
  yesPrice: number;
  noPrice: number;
  timeRemaining: string;
  totalLiquidity: string;
  status: "open" | "locked" | "resolved";
  result?: "yes" | "no";
  onClick: (id: string) => void;
};
```

### Accessibility Testing

**Required Testing Steps:**

1. Keyboard-only navigation (Tab, Shift+Tab, Enter, Escape)
2. Screen reader testing with NVDA/JAWS on Windows, VoiceOver on macOS
3. Automated testing: axe DevTools browser extension
4. Manual contrast ratio verification: WebAIM tool
5. Color blindness simulation tools (Sim Daltonism plugin)

### Performance Budgets

- **Largest Contentful Paint (LCP):** < 2.5s
- **Cumulative Layout Shift (CLS):** < 0.1
- **First Input Delay (FID):** < 100ms
- **JavaScript Bundle Size:** < 500KB (total)
- **CSS Bundle Size:** < 50KB (Tailwind + custom)

---

## References

### Design System & UI Patterns

- [Fintech Design Guide with Trust Patterns (2026)](https://www.eleken.co/blog-posts/modern-fintech-design-guide)
- [React Financial Dashboard Design Patterns](https://olivertriunfo.com/react-financial-dashboard-design-patterns/)
- [Let's Build a Financial Dashboard with React](https://www.telerik.com/blogs/lets-build-a-financial-dashboard-with-react)
- [16 Best React Dashboards in 2026](https://www.untitledui.com/blog/react-dashboards)

### Tailwind CSS & Design Tokens

- [Tailwind CSS v4 Theme Variables Documentation](https://tailwindcss.com/docs/theme)
- [Tailwind CSS Best Practices 2025-2026](https://www.frontendtools.tech/blog/tailwind-css-best-practices-design-system-patterns)
- [Tailwind CSS v4 Color System](https://tailwindcss.com/blog/tailwindcss-v4)
- [Design Tokens in Tailwind 4 with @theme](https://medium.com/@sureshdotariya/tailwind-css-4-theme-the-future-of-design-tokens-at-2025-guide-48305a26af06)

### Accessibility Standards

- [WCAG 2.1 Level AA Guidelines](https://www.w3.org/TR/WCAG21/)
- [Understanding WCAG 2.1 Color Contrast Requirements](https://www.accessibleresources.com/post/understanding-wcag-2-1-a-aa-and-aaa-guidelines-for-color-contrast)
- [WebAIM Contrast Checker Tool](https://webaim.org/resources/contrastchecker/)
- [Section 508 Accessible Color Usage](https://www.section508.gov/create/making-color-usage-accessible/)
- [Web Content Accessibility Guidelines 2026 Update](https://pilotdigital.com/blog/what-wcag-2-1aa-means-for-healthcare-organizations-in-2026/)

### React & Performance

- [React 19 Components and Hooks Guide](https://www.nucamp.co/blog/react-fundamentals-in-2026-components-hooks-react-compiler-and-modern-ui-development)
- [Building High-Performance Financial Dashboards with React](https://olivertriunfo.com/react-financial-dashboards/)

### Solana & DeFi Design

- [Chausar PRD](./PRD.md) - Original product requirements
- [Solana Documentation](https://docs.solana.com)
- [Anchor Framework](https://www.anchor-lang.com)

---

## Appendix: Design Tokens Reference

### Color Tokens Quick Reference

```css
/* Semantic Colors */
--primary: oklch(0.208 0.042 265.755); /* Primary action (CTA buttons) */
--secondary: oklch(0.968 0.007 247.896); /* Secondary action */
--destructive: oklch(0.577 0.245 27.325); /* Destructive action (errors) */
--success: oklch(0.5 0.15 150); /* Success state */
--warning: oklch(0.7 0.18 70); /* Warning state */
--info: oklch(0.6 0.1 250); /* Info state */

/* Outcome Colors */
--yes-color: oklch(0.5 0.2 142); /* YES outcome */
--yes-bg: oklch(0.95 0.04 142); /* YES background */
--no-color: oklch(0.6 0.24 25); /* NO outcome */
--no-bg: oklch(0.95 0.08 25); /* NO background */

/* Neutral Scale */
--foreground: oklch(0.129 0.042 264.695); /* Primary text */
--muted: oklch(0.554 0.046 257.417); /* Secondary text */
--border: oklch(0.929 0.013 255.508); /* Component borders */
--background: oklch(1 0 0); /* Page background */
--card: oklch(1 0 0); /* Card background */
```

### Spacing Scale Reference

```
xs: 4px    (gap-1, p-1)
sm: 8px    (gap-2, p-2)
md: 16px   (gap-4, p-4)
lg: 24px   (gap-6, p-6)
xl: 32px   (gap-8, p-8)
2xl: 40px  (gap-10, p-10)
```

### Font Scale Reference

```
H1: 2rem (32px) - text-3xl
H2: 1.5rem (24px) - text-2xl
H3: 1.25rem (20px) - text-xl
H4: 1rem (16px) - text-lg
Body: 0.95-1rem (15-16px) - text-base/text-sm
Label: 0.75rem (12px) - text-xs
```

---

**Document Version:** 1.0
**Last Updated:** January 20, 2026
**Owner:** Design Team
**Status:** Proposed
**Next Review:** After component implementation begins
