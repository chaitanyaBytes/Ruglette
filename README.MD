# Ruglette

A decentralized roulette game built on Solana using Anchor framework. Ruglette provides a secure and transparent way to play roulette on the blockchain.

## Overview

Ruglette is a Solana-based roulette game that implements a fair and transparent betting system. The game uses Switchboard's on-demand randomness for provably fair results and includes features like house fees, bet limits, and multiple bet types.

## Features

- 💰 Configurable minimum and maximum bets
- 🏦 House fee system with configurable basis points
- 🔒 Provably fair using Switchboard's on-demand randomness
- ⏱️ Round-based gameplay with time tracking
- 💎 Solana native integration

## Prerequisites

- Rust (latest stable version)
- Solana CLI tools
- Anchor Framework
- Node.js and npm/yarn
- Switchboard SDK

## Installation

1. Clone the repository:
```bash
git clone https://github.com/chaitanyabytes/ruglette.git
cd ruglette
```

2. Install dependencies:
```bash
npm install
# or
yarn install
```

3. Build the program:
```bash
anchor build
```

## Program Structure

The project is organized into several key modules:

- `lib.rs`: Main program entry point and instruction definitions
- `constants.rs`: Game constants and configuration
- `error.rs`: Custom error handling
- `instructions/`: Game instruction implementations
- `state/`: Program state management
- `types.rs`: Custom type definitions
- `utils.rs`: Utility functions

## Game Flow

1. **Initialize Game**
   - Set up game parameters (min/max bets, house fee)
   - Create house vault

2. **Initialize Round**
   - Create a new round for a player
   - Set round start time

3. **Place Bets**
   - Players can place multiple bets
   - Supports various bet types (straight, black, etc.)

4. **Wheel Spin**
   - Trigger wheel spin using Switchboard randomness
   - Verify randomness

5. **Settle Bets**
   - Calculate and distribute winnings
   - Update house vault

## Testing

Run the test suite:
```bash
anchor test
```

The test suite includes:
- Game initialization
- Round management
- Bet placement
- Wheel spinning
- Randomness verification
- Bet settlement

## Security

- Uses Switchboard's on-demand randomness for provably fair results
- Implements proper access controls
- Includes house fee system
- Bet limits to prevent excessive losses

<!-- ## License

[Add your license here] -->

