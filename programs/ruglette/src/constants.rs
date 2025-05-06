use anchor_lang::{prelude::*, solana_program::native_token::LAMPORTS_PER_SOL};

#[constant]
pub const SEED: &str = "anchor";

pub const EUROPEAN_ROULETTE_NUMBERS: [u8; 37] = [
    0, 32, 15, 19, 4, 21, 2, 25, 17, 34, 6, 27, 13, 36, 11, 30, 8, 23, 10, 5, 24, 16, 33, 1, 20,
    14, 31, 9, 22, 18, 29, 7, 28, 12, 35, 3, 26,
];

pub const STRAIGHT_UP_PAYOUT_MULTIPLIER: u8 = 35;

pub const SLOT_COUNT: usize = EUROPEAN_ROULETTE_NUMBERS.len();
pub const MAX_BET_AMOUNT: u64 = 100 * LAMPORTS_PER_SOL;
