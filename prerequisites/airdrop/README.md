# Turbin3 Builder's Cohort Q1

This repository contains scripts for completing the Turbin3 Builder's Cohort Q1 prerequisites. The scripts handle wallet creation, SOL airdrop, transfers, and enrollment in the Turbin3 program.

## Setup

1. Install dependencies:
   ```sh
   yarn install
   ```

2. Create a new wallet:
   ```sh
   yarn keygen
   ```
   Save the output as dev-wallet.json in the project root.
   Use ```yarn convert``` when needed.

## Available Scripts

1. Request Airdrop
   Get 2 SOL from devnet:
   ```sh
   yarn airdrop
   ```

2. Transfer SOL
   Transfer your SOL to the Turbin3 wallet:
   ```sh
   yarn transfer
   ```

3. Enroll in Turbin3
   Register your GitHub username with the program to submit the assignment on chain:
   ```sh
   yarn enroll
   ```

## Technical Details

### Wallet Management
- Uses @solana/web3.js for keypair generation and management
- Stores wallet data in JSON format
- Supports conversion between Base58 and byte array formats

### Program Interaction
- Program ID: WBAQSygkwMox2VuWKU133NxFrpDZUBdvSBeaBEue2Jq
- Uses Anchor framework for program interaction
- PDA (Program Derived Address) seeds:
  - Constant: "prereq"
  - Account: Signer's public key

### Security
- .gitignore configured to prevent wallet files from being committed
- Private keys stored locally in:
  - dev-wallet.json
  - Turbin3-wallet.json

### Error Handling
All scripts include try-catch blocks with detailed error logging, including:
- Transaction errors
- Anchor program errors
- Network connection issues

### Environment
- Network: Solana Devnet
- Required Node.js packages:
  - @coral-xyz/anchor: ^0.30.1
  - @solana/web3.js: 1
  - bs58: ^6.0.0

## Project Structure
```
airdrop/
├── .editorconfig          # Editor configuration
├── .gitignore            # Git ignore rules
├── package.json          # Project dependencies and scripts
├── README.md            # This file
├── transfer.ts          # SOL transfer script
├── dev-wallet.json      # Development wallet keypair
├── Turbin3-wallet.json  # Turbin3 program wallet
├── enroll.ts           # Program enrollment script
└── Turbin3_prereq.ts   # Program IDL definition
```

## Program IDL
The program interface includes:
- Instructions:
  - complete: Register GitHub username
  - update: Update existing registration
- Accounts:
  - Q2Prereq2024: Stores GitHub and public key 
  - IDL from https://explorer.solana.com/address/WBAQSygkwMox2VuWKU133NxFrpDZUBdvSBeaBEue2Jq/anchor-program?cluster=devnet
- Error Handling:
  - InvalidGithubAccount: Validates GitHub account format

Feel free to use the scripts as a reference for your own projects!