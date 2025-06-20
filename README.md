# Counter Program (Solana Anchor)

This is a simple on-chain counter program built using the [Anchor](https://project-serum.github.io/anchor/) framework for the Solana blockchain. The program demonstrates basic account management, authority checks, and state updates.

## Features
- **Initialize**: Create a new counter account for a user.
- **Increment**: Increment the counter value (only by the account authority).
- **Delegate**: Placeholder for delegation logic (currently only checks authority).

## Program ID
```
BTHp3sf89ZWMGiMTSaZSWf8UC2a1oGwKtF2ZVhQgwa6q
```

## Account Structure
- **Counter**
  - `count` (u64): The current value of the counter.
  - `authority` (Pubkey): The public key of the user who can increment or delegate.

## Instructions
### 1. Initialize
Creates a new counter account for the user.
- Accounts:
  - `[signer]` User (payer & authority)
  - `[writable]` Counter (PDA)
  - `[]` System Program

### 2. Increment
Increments the counter by 1. Only the authority can call this.
- Accounts:
  - `[signer]` User (must match authority)
  - `[writable]` Counter (PDA)

### 3. Delegate
Placeholder for delegation logic. Only the authority can call this.
- Accounts:
  - `[signer]` User (must match authority)
  - `[writable]` Counter (PDA)

## Building & Deploying
1. **Install dependencies:**
   ```bash
   yarn install
   ```
2. **Build the program:**
   ```bash
   anchor build
   ```
3. **Deploy to localnet:**
   ```bash
   anchor deploy
   ```

## Testing
Run the TypeScript tests:
```bash
yarn test
```

## Directory Structure
- `programs/counter/` - Rust source code for the on-chain program
- `tests/` - TypeScript tests
- `migrations/` - Deployment scripts

## Requirements
- [Anchor/Solana Playground CLI](https://beta.solpg.io/)
- Node.js & Yarn

## License
MIT 