![Counter Program Logo](https://camo.githubusercontent.com/590ccfb4e70a27673047ee879ed409981c05b2da403e60b4aaa7961ccdb46001/68747470733a2f2f7062732e7477696d672e636f6d2f6d656469612f46565556614f3958454141756c764b3f666f726d61743d706e67266e616d653d736d616c6c)

# 🧮 Counter Program (Solana Anchor)

Welcome to the **Counter Program**! This is a simple, elegant on-chain counter built with [Anchor](https://project-serum.github.io/anchor/) for the Solana blockchain. It demonstrates basic account management, authority checks, and state updates—all in a beginner-friendly way.

---

## ✨ Features
- 🚀 **Initialize**: Create a new counter account for a user.
- ➕ **Increment**: Increment the counter value (only by the account authority).
- 🛡️ **Delegate**: Placeholder for delegation logic (currently only checks authority).

---

## 🆔 Program ID
```txt
BTHp3sf89ZWMGiMTSaZSWf8UC2a1oGwKtF2ZVhQgwa6q
```

---

## 🗂️ Account Structure
- **Counter**
  - `count` (`u64`): The current value of the counter.
  - `authority` (`Pubkey`): The public key of the user who can increment or delegate.

---

## 📝 Instructions

### 1️⃣ Initialize
Creates a new counter account for the user.
- **Accounts:**
  - `[signer]` User (payer & authority)
  - `[writable]` Counter (PDA)
  - `[]` System Program

### 2️⃣ Increment
Increments the counter by 1. Only the authority can call this.
- **Accounts:**
  - `[signer]` User (must match authority)
  - `[writable]` Counter (PDA)

### 3️⃣ Delegate
Placeholder for delegation logic. Only the authority can call this.
- **Accounts:**
  - `[signer]` User (must match authority)
  - `[writable]` Counter (PDA)

---

## 🛠️ Building & Deploying
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

---

## 🧪 Testing
Run the TypeScript tests:
```bash
yarn test
```

---

## 📁 Directory Structure
- `programs/counter/` — Rust source code for the on-chain program
- `tests/` — TypeScript tests
- `migrations/` — Deployment scripts

---

## 📦 Requirements
- [Anchor/Solana Playground CLI](https://beta.solpg.io/)
- Node.js & Yarn

---

## 📄 License
MIT 