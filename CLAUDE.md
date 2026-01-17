# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Chausar is a decentralized prediction market on Solana. See PRD.md for full requirements.

## Common Commands

```bash
# Development
npm run dev              # Start Next.js dev server (http://localhost:3000)
npm run build            # Production build
npm run lint             # ESLint check
npm run format           # Prettier format
npm run format:check     # Check formatting
npm run ci               # Build + lint + format check

# Anchor program (requires Rust, Solana CLI, Anchor)
npm run anchor-build     # Build Anchor program (cd anchor && anchor build)
npm run anchor-test      # Run Rust tests with LiteSVM (cd anchor && anchor test --skip-deploy)
npm run setup            # Build program + regenerate TypeScript client
npm run codama:js        # Regenerate TypeScript client from IDL
```

## Architecture

### Frontend → Program Client → On-Chain Program

1. **Next.js App** (`app/`) - React 19 frontend with Tailwind CSS v4
2. **Codama-generated client** (`app/generated/`) - Type-safe TypeScript bindings auto-generated from Anchor IDL
3. **Anchor Program** (`anchor/programs/`) - Rust smart contracts deployed to devnet

### Key Files

- `app/components/providers.tsx` - Solana client setup and wallet connection via `@solana/react-hooks`
- `codama.json` - Configuration for TypeScript client generation from Anchor IDL

### Client Generation Flow

When you modify the Anchor program:
1. `npm run anchor-build` - Compiles Rust, generates IDL at `anchor/target/idl/`
2. `npm run codama:js` - Codama reads IDL, outputs TypeScript client to `app/generated/`

The generated client in `app/generated/` should not be manually edited - it's overwritten on regeneration.

### Solana Libraries

- `@solana/kit` - Core Solana primitives (accounts, transactions)
- `@solana/client` - RPC client
- `@solana/react-hooks` - React hooks for wallet connection with auto-discovery
