use std::fmt::{self, Display, Result};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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

pub type GameState = [[Player; 3]; 3];

pub fn print_grid(grid: &GameState) {
    clear_the_screen();
    println!("   1 2 3");
    // U+256D "Box Drawings Light Arc Down and Right" and
    // U+2500 "Box Drawings Light Horizontal" and
    // U+252C "Box Drawings Light Down and Horizontal" and
    // U+256E "Box Drawings Light Arc Down and Left"
    println!("  ╭─┬─┬─╮");
    for (row_idx, row) in grid.iter().enumerate() {
        // U+2502 "Box Drawings Light Vertical"
        print!("{} │", row_idx + 1);
        for (slot_idx, slot) in row.iter().enumerate() {
            print!("{slot}");
            if slot_idx != 2 {
                // U+2502 "Box Drawings Light Vertical"
                print!("│");
            }
        }
        // U+2502 "Box Drawings Light Vertical"
        print!("│");

        println!();
        if row_idx != 2 {
            // U+2500 "Box Drawings Light Horizontal" and
            // U+253C "Box Drawings Light Vertical and Horizontal"
            println!("  ├─┼─┼─┤");
        }
    }
    // U+2570 "Box Drawings Light Arc Up and Right" and
    // U+2500 "Box Drawings Light Horizontal" and
    // U+252C "Box Drawings Light Down and Horizontal" and
    // U+256F "Box Drawings Light Arc Up and Left"
    println!("  ╰─┴─┴─╯");
}

fn clear_the_screen() {
    print!("\x1Bc");
}
