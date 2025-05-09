use crate::{
    error::ErrorCodes,
    types::{BetType, Color},
};
pub use anchor_lang::prelude::*;

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

    /// European roulette color mapping
    pub fn get_color(number: u8) -> Color {
        match number {
            0 => Color::Green,
            1 | 3 | 5 | 7 | 9 | 12 | 14 | 16 | 18 | 19 | 21 | 23 | 25 | 27 | 30 | 32 | 34 | 36 => {
                Color::Red
            }
            _ => Color::Black,
        }
    }
}

pub fn transfer<'a>(
    system_program: AccountInfo<'a>,
    from: AccountInfo<'a>,
    to: AccountInfo<'a>,
    amount: u64,
    seeds: Option<&[&[&[u8]]]>, // Use Option to explicitly handle the presence or absence of seeds
) -> Result<()> {
    let amount_needed = amount;

    if amount_needed > from.lamports() {
        msg!(
            "needed {} lamports, but only have {}",
            amount_needed,
            from.lamports(),
        );
        return Err(ErrorCodes::NotEnoughFundsToPlay.into());
    }

    let transfer_accounts = anchor_lang::system_program::Transfer {
        from: from.to_account_info(),
        to: to.to_account_info(),
    };

    let transfer_ctx = match seeds {
        Some(seeds) => CpiContext::new_with_signer(system_program, transfer_accounts, seeds),
        None => CpiContext::new(system_program, transfer_accounts),
    };

    anchor_lang::system_program::transfer(transfer_ctx, amount)
}
