use std::fmt::{self, Display, Result};
use std::io::{Write, stdin, stdout};

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
    while !game_turn(&mut state, current_player) {
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

fn take_pos_inputs(state: &[[Player; 3]; 3]) -> (usize, usize) {
    loop {
        print!("Enter your move as {{col}},{{row}} (e.g. 1,2): ");
        let _ = stdout().flush();

        let mut input_str: String = String::new();
        let _ = stdin().read_line(&mut input_str);

        let mut split = input_str.split(',');

        let col = split.next();
        let row = split.next();
        if row.is_none() || col.is_none() {
            println!("Invalid input. Try again.");
            continue;
        }

        let col = col.unwrap().trim().parse::<usize>();
        let row = row.unwrap().trim().parse::<usize>();

        if col.is_err() || row.is_err() {
            println!("Invalid input. Try again.");
            continue;
        }

        let col = col.unwrap() - 1;
        let row = row.unwrap() - 1;

        if state[row][col] != Player::None {
            println!("Slot already occupied. Try again.");
            continue;
        }

        return (row, col);
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
