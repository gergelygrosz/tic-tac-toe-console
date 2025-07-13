use std::fmt::{self, Display, Result};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Player {
    None = 0,
    O = 1,
    X = 2,
}

impl Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            match self {
                Player::None => String::from(" "),
                Player::O => String::from("O"),
                Player::X => String::from("X"),
            }
        )
    }
}

pub fn print_grid(grid: &[[Player; 3]; 3]) {
    for (row_idx, row) in grid.iter().enumerate() {
        for (slot_idx, slot) in row.iter().enumerate() {
            print!("{slot}");
            if slot_idx != 2 {
                // U+2502 "Box Drawings Light Vertical"
                print!("│");
            }
        }

        println!();
        if row_idx != 2 {
            // U+2500 "Box Drawings Light Horizontal" and
            // U+253C "Box Drawings Light Vertical and Horizontal"
            println!("─┼─┼─");
        }
    }
}
