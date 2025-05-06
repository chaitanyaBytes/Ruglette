use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Red,
    Black,
    Green,
}

#[repr(u8)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum BetType {
    Straight = 0, // Single number
    Split = 1,    // Two numbers
    Street = 2,   // Three numbers in a row
    Corner = 3,   // Four numbers in a square
    Sixline = 4,  // Six numbers (two adjacent rows)
    Dozen = 5,    // 1st, 2nd, or 3rd dozen
    Column = 6,   // 1st, 2nd, or 3rd column
    Red = 7,      // All red numbers
    Black = 8,    // All black numbers
    Odd = 9,      // All odd numbers
    Even = 10,    // All even numbers
    Low = 11,     // 1-18
    High = 12,    // 19-36
}

impl BetType {
    pub fn multiplier(&self) -> u8 {
        match self {
            BetType::Straight => 35, // Pays 35:1
            BetType::Split => 17,    // Pays 17:1
            BetType::Street => 11,   // Pays 11:1
            BetType::Corner => 8,    // Pays 8:1
            BetType::Sixline => 5,   // Pays 5:1
            BetType::Dozen => 2,     // Pays 2:1
            BetType::Column => 2,    // Pays 2:1
            BetType::Red => 1,       // Pays 1:1
            BetType::Black => 1,     // Pays 1:1
            BetType::Odd => 1,       // Pays 1:1
            BetType::Even => 1,      // Pays 1:1
            BetType::Low => 1,       // Pays 1:1
            BetType::High => 1,      // Pays 1:1
        }
    }

    pub fn valid_targets(&self) -> usize {
        match self {
            BetType::Straight => 1,
            BetType::Split => 2,
            BetType::Street => 3,
            BetType::Corner => 4,
            BetType::Sixline => 6,
            BetType::Dozen => 12,
            BetType::Column => 12,
            BetType::Red
            | BetType::Black
            | BetType::Odd
            | BetType::Even
            | BetType::Low
            | BetType::High => 18,
        }
    }
}
