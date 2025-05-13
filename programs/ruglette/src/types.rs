use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Red,
    Black,
    Green,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, InitSpace, Debug)]
pub enum GameStatus {
    AcceptingBets,
    WaitingForVRF,
    ResultReady,
    BetsSettled,
    PayoutsProcessed,
    Error,
}

#[repr(u8)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, InitSpace)]
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
