# Task: Enhanced Wallet Integration

Metadata:
- Dependencies: task-27 (Navigation component)
- Provides: Enhanced wallet connection with persistence
- Size: Small (2-3 files)

## Implementation Content
Enhance wallet integration with:
- Wallet connection persistence across sessions
- Support for multiple wallets (Phantom, Solflare)
- Display connected wallet address
- Handle disconnect gracefully
- Auto-reconnect on page load

## Target Files
- [ ] `app/components/providers.tsx` (update)
- [ ] `app/lib/wallet.ts` (new)
- [ ] `app/hooks/useWallet.ts` (new utility hook)

## Implementation Steps (TDD: Red-Green-Refactor)

### 1. Red Phase
- [ ] Review existing wallet setup in providers.tsx
- [ ] Define wallet persistence requirements

### 2. Green Phase

#### Wallet Utility Functions
```typescript
// app/lib/wallet.ts
const WALLET_STORAGE_KEY = "chausar_connected_wallet";

export interface StoredWalletInfo {
  walletName: string;
  connectedAt: number;
}

export function saveWalletConnection(walletName: string): void {
  if (typeof window === "undefined") return;

  const info: StoredWalletInfo = {
    walletName,
    connectedAt: Date.now(),
  };

  try {
    localStorage.setItem(WALLET_STORAGE_KEY, JSON.stringify(info));
  } catch (e) {
    console.warn("Failed to save wallet connection", e);
  }
}

export function getStoredWalletConnection(): StoredWalletInfo | null {
  if (typeof window === "undefined") return null;

  try {
    const stored = localStorage.getItem(WALLET_STORAGE_KEY);
    if (!stored) return null;

    const info = JSON.parse(stored) as StoredWalletInfo;

    // Consider connections older than 7 days as expired
    const SEVEN_DAYS = 7 * 24 * 60 * 60 * 1000;
    if (Date.now() - info.connectedAt > SEVEN_DAYS) {
      clearWalletConnection();
      return null;
    }

    return info;
  } catch (e) {
    console.warn("Failed to read stored wallet connection", e);
    return null;
  }
}

export function clearWalletConnection(): void {
  if (typeof window === "undefined") return;

  try {
    localStorage.removeItem(WALLET_STORAGE_KEY);
  } catch (e) {
    console.warn("Failed to clear wallet connection", e);
  }
}

export function truncateAddress(address: string, chars = 4): string {
  if (!address) return "";
  return `${address.slice(0, chars)}...${address.slice(-chars)}`;
}
```

#### Enhanced Wallet Hook
```typescript
// app/hooks/useWallet.ts
"use client";

import { useCallback, useEffect, useState } from "react";
import { useWalletConnection } from "@solana/react-hooks";
import {
  saveWalletConnection,
  getStoredWalletConnection,
  clearWalletConnection,
  truncateAddress,
} from "@/lib/wallet";

export interface UseWalletReturn {
  isConnected: boolean;
  isConnecting: boolean;
  address: string | null;
  displayAddress: string;
  walletName: string | null;
  connect: () => Promise<void>;
  disconnect: () => void;
  error: Error | null;
}

export function useWallet(): UseWalletReturn {
  const { wallet, status, connect: baseConnect, disconnect: baseDisconnect } = useWalletConnection();
  const [error, setError] = useState<Error | null>(null);
  const [walletName, setWalletName] = useState<string | null>(null);

  const isConnected = status === "connected" && !!wallet;
  const isConnecting = status === "connecting";
  const address = wallet?.address?.toString() ?? null;
  const displayAddress = address ? truncateAddress(address) : "";

  // Auto-reconnect on mount if previously connected
  useEffect(() => {
    const storedConnection = getStoredWalletConnection();
    if (storedConnection && status === "disconnected") {
      // Attempt to reconnect
      baseConnect().catch((e) => {
        console.warn("Auto-reconnect failed", e);
        clearWalletConnection();
      });
    }
  }, [status, baseConnect]);

  // Save connection when connected
  useEffect(() => {
    if (isConnected && wallet) {
      // Try to get wallet name from the wallet object
      const name = (wallet as any).name || "Unknown Wallet";
      setWalletName(name);
      saveWalletConnection(name);
    }
  }, [isConnected, wallet]);

  const connect = useCallback(async () => {
    setError(null);
    try {
      await baseConnect();
    } catch (e) {
      const error = e instanceof Error ? e : new Error("Failed to connect wallet");
      setError(error);
      throw error;
    }
  }, [baseConnect]);

  const disconnect = useCallback(() => {
    clearWalletConnection();
    setWalletName(null);
    setError(null);
    baseDisconnect();
  }, [baseDisconnect]);

  return {
    isConnected,
    isConnecting,
    address,
    displayAddress,
    walletName,
    connect,
    disconnect,
    error,
  };
}
```

#### Update Providers (if needed)
```typescript
// app/components/providers.tsx - ensure proper configuration
"use client";

import { ReactNode } from "react";
import { SolanaProvider } from "@solana/react-hooks";
import { PROGRAM_ID, RPC_ENDPOINT } from "@/lib/config";

interface ProvidersProps {
  children: ReactNode;
}

export function Providers({ children }: ProvidersProps) {
  return (
    <SolanaProvider
      rpcEndpoint={RPC_ENDPOINT}
      config={{
        // Enable wallet auto-discovery
        autoConnect: true,
        // Supported wallets will be auto-detected
      }}
    >
      {children}
    </SolanaProvider>
  );
}
```

### 3. Refactor Phase
- [ ] Add error boundary for wallet errors
- [ ] Add reconnection retry logic
- [ ] Improve error messages

## Completion Criteria
- [ ] Wallet connection persists across page refreshes
- [ ] Phantom wallet connects successfully
- [ ] Solflare wallet connects successfully
- [ ] Disconnect clears stored state
- [ ] Auto-reconnect works on page load
- [ ] Build passes
- [ ] Operation verified: L1 (functional wallet)

## Notes
- Impact scope: Used by all components requiring wallet
- Constraints: Must work with @solana/react-hooks
- LocalStorage used for persistence (7-day expiry)
