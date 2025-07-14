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

/// Displays the game state.
/// Takes and processes user input.
/// Returns `true` if the game is won.
fn game_turn(state: &mut [[Player; 3]; 3], current_player: Player) -> bool {
    clear_the_screen();
    print_grid(&state);
    println!("PLAYER {}'s turn!", current_player);

    // TODO: make it so that players can't choose and overwrite non-empty slots
    let row: usize = take_position_input("input: position: row: ");
    let col: usize = take_position_input("input: position: col: ");

    state[row - 1][col - 1] = current_player;

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

fn take_position_input(prompt: &str) -> usize {
    loop {
        print!("{}", prompt);
        let _ = stdout().flush();

        let mut input: String = String::new();
        let res = io::stdin().read_line(&mut input);
        if res.is_err() {
            println!("{}", res.unwrap_err()); // TODO: add user-friendly error messages
            continue;
        }

        input = input.trim().to_string();

        let res = input.parse::<usize>();
        if res.is_err() {
            println!("{}", res.unwrap_err()); // TODO: add user-friendly error messages
            continue;
        }

        let res = res.unwrap();
        if res <= 0 {
            println!("too small")
        }
        if res >= 4 {
            println!("too big");
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
