# Chausar

Decentralized prediction market on Solana. Create binary (Yes/No) markets, trade positions via AMM, and receive automatic payouts based on oracle-resolved outcomes.

## Features

- **Permissionless** - Anyone can create prediction markets
- **Non-custodial** - Users control funds via wallet
- **AMM-based** - Instant trading with constant product formula
- **Automatic settlement** - Trustless payouts upon resolution

## Getting Started

```bash
npm install
npm run dev
```

Open [http://localhost:3000](http://localhost:3000) and connect your wallet.

## Stack

| Layer          | Technology                              |
| -------------- | --------------------------------------- |
| Frontend       | Next.js 16, React 19, TypeScript        |
| Styling        | Tailwind CSS v4                         |
| Solana Client  | `@solana/client`, `@solana/react-hooks` |
| Program Client | Codama-generated, `@solana/kit`         |
| Program        | Anchor (Rust)                           |

## Project Structure

```
├── app/
│   ├── components/
│   │   └── providers.tsx        # Solana client setup
│   ├── generated/prediction/    # Codama-generated program client
│   └── page.tsx
├── anchor/
│   └── programs/prediction/     # Prediction market program (Rust)
└── codama.json                  # Client generation config
```

## Development

```bash
npm run dev              # Start Next.js dev server
npm run build            # Production build
npm run lint             # ESLint check
npm run format           # Prettier format
```

## Anchor Program

Requires [Rust](https://rustup.rs/), [Solana CLI](https://solana.com/docs/intro/installation), and [Anchor](https://www.anchor-lang.com/docs/installation).

```bash
npm run anchor-build     # Build program
npm run anchor-test      # Run tests with LiteSVM
npm run setup            # Build + regenerate TypeScript client
```

## Deploying

1. Configure Solana CLI for devnet:
   ```bash
   solana config set --url devnet
   solana airdrop 2
   ```

2. Build and deploy:
   ```bash
   cd anchor
   anchor build
   anchor keys sync
   anchor build
   anchor deploy
   cd ..
   ```

3. Regenerate the client:
   ```bash
   npm run setup
   ```

## Documentation

- [PRD.md](./PRD.md) - Product requirements and specifications

## Learn More

- [Solana Docs](https://solana.com/docs)
- [Anchor Docs](https://www.anchor-lang.com/docs)
- [Codama](https://github.com/codama-idl/codama)
