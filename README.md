# Title

Gym Membership Tokenizer

# Description

This project tokenizes gym and co-working memberships on Stellar using a Soroban smart contract and a Node.js/TypeScript backend.
It exists to make long-term subscriptions flexible, liquid, and easily transferable on the secondary market.

# Project Vision

Build a trustworthy Real-World Asset (RWA) platform where users can freely trade unused gym days, eliminating wasted subscriptions.
The long-term goal is to create a Stellar-native "Fungible Time" standard for gyms, co-working spaces, and other time-based service providers, allowing them to secure upfront capital while offering users ultimate flexibility.

# Features

* On-chain token logic (Mint, Transfer, Check-in) powered by Soroban Smart Contracts
* Hybrid Web2/Web3 architecture with a Node.js/Express backend
* Soroban Event Indexer to automatically sync blockchain events (check-ins, transfers) to a database
* PostgreSQL persistence (via Prisma ORM) for user profiles and fast transaction history querying
* Secure authorization using Stellar's native `require_auth` for all state-changing actions
* Public API for frontend and mobile app integration

# Getting Started

This section explains how to run the backend and frontend locally for development.

Prerequisites:

* Node.js and a package manager (`npm` or `pnpm`)
* PostgreSQL database
* Rust toolchain and Cargo (if you plan to compile or modify the Soroban contract)
* Optional: `stellar` CLI for deploying or invoking the contract manually

Backend (development):

PowerShell example:

```powershell
cd backend
# $env:DATABASE_URL = "postgresql://user:password@localhost:5432/gym_token"
# $env:PORT = "3000"
npm install
npx prisma db push
npm run dev

```

Bash example:

```bash
cd backend
export DATABASE_URL="postgresql://user:password@localhost:5432/gym_token"
export PORT=3000
npm install
npx prisma db push
npm run dev

```

After the server starts you should see a log like `Backend Server đang chạy tại http://localhost:3000` and the following endpoints will be available:

* `GET /api/users/:walletAddress` — user profile and transaction history
* `POST /api/users` — register a new Web2 user profile

Frontend (development):

Create a `.env` in the `frontend` folder or set env var `VITE_API_BASE` to your backend URL (example: `http://localhost:3000`). Then:

```bash
cd frontend
npm install
npm run dev

```

Open `http://localhost:5173` (Vite default) in your browser to view the app.

Quick test using curl (after backend is running):

```bash
curl http://127.0.0.1:3000/api/users/GDYOURWALLETADDRESSHERE

```

# Contract

Contract link: https://stellar.expert/explorer/testnet/contract/CD2MKGF5JUZPIA6VCVYEIGCF4XJ4RR4ROJ3WUC2LVGE7PC3EX3FK3T5K

Contract's screenshot

![Contract screenshot placeholder](./assets/contract_screenshot.png)

# Future scopes

* Add a dedicated frontend DEX interface for users to easily list and buy tokenized gym days
* Implement a Smart Contract royalty fee system so gym owners earn a percentage of secondary market trades
* Integrate biometric (FaceID) mapping to wallet addresses for seamless physical check-ins
* Improve deployment automation and admin dashboard for managing multiple gym branches

# Profile

Name: Dat To Thanh
Skills: Rust, Soroban, Node.js, TypeScript, PostgreSQL, Prisma, Web3 architecture
Focus: Building transparent, utility-driven Real-World Asset (RWA) infrastructure on Stellar