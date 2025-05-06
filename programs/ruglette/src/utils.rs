use crate::types::Color;

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
