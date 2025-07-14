use std::fmt::{self, Display, Result};
use std::io::{self, Write, stdout};

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

fn main() {
    let mut state: [[Player; 3]; 3] = [[Player::None; 3]; 3];
    print_grid(&state);

    let mut current_player = Player::O;
    loop {
        let end = game_turn(&mut state, current_player);
        if end {
            break;
        }

        current_player = match current_player {
            Player::O => Player::X,
            Player::X => Player::O,
            Player::None => panic!(),
        }
    }
}

/// Displays the game state.
/// Takes and processes user input.
/// Returns `true` if the game is won.
fn game_turn(state: &mut [[Player; 3]; 3], current_player: Player) -> bool {
    clear_the_screen();
    print_grid(&state);
    println!("PLAYER {}'s turn!", current_player);

    let pos: (usize, usize) = take_pos_inputs(&state);
    state[pos.0][pos.1] = current_player;

    if is_victory(&state, Player::O) {
        clear_the_screen();
        print_grid(&state);
        println!("PLAYER {} won! Congratulations!", Player::O);
        return true;
    };

    if is_victory(&state, Player::X) {
        clear_the_screen();
        print_grid(&state);
        println!("PLAYER {} won! Congratulations!", Player::X);
        return true;
    }

    false
}

fn clear_the_screen() {
    print!("\x1Bc");
}

pub fn print_grid(grid: &[[Player; 3]; 3]) {
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

/// Returns the INDEX!!!
fn take_pos_inputs(state: &[[Player; 3]; 3]) -> (usize, usize) {
    loop {
        let row = take_one_input("enter the row:") - 1;
        let col = take_one_input("enter the column:") - 1;

        if state[row][col] == Player::None {
            return (row, col);
        }

        println!("already occupied. try again.")
    }
}

fn take_one_input(prompt: &str) -> usize {
    loop {
        print!("{}", prompt);
        let _ = stdout().flush();

        let mut input: String = String::new();
        let res = io::stdin().read_line(&mut input);
        if res.is_err() {
            println!("error while reading console. try again.");
            continue;
        }

        input = input.trim().to_string();

        let res = input.parse::<usize>();
        if res.is_err() {
            println!("invalid input. try again.");
            continue;
        }

        let res = res.unwrap();
        if res < 1 {
            println!("too small. try again.");
            continue;
        }
        if res > 3 {
            println!("too big. try again.");
            continue;
        }

        return res;
    }
}

/// Returns `true` if the given player won the game. Otherwise, returns `false``.
fn is_victory(state: &[[Player; 3]; 3], player: Player) -> bool {
    // horizontal checks
    for row in state {
        if row[0] == player && row[1] == player && row[2] == player {
            return true;
        }
    }

    // vertical checks
    for col in 0..3 {
        if state[0][col] == player && state[1][col] == player && state[2][col] == player {
            return true;
        }
    }

    // diagonal checks
    // upper-left -> bottom-right
    if state[0][0] == player && state[1][1] == player && state[2][2] == player {
        return true;
    }

    // upper-right -> bottom-left
    if state[0][2] == player && state[1][1] == player && state[2][0] == player {
        return true;
    }

    false
}
