use std::collections::HashSet;
use std::fmt::{self, Display, Result};
use std::io::{Write, stdin, stdout};

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

type GameState = [[Player; 3]; 3];

fn main() {
    let mut state: GameState = [[Player::None; 3]; 3];
    print_grid(&state);

    let mut current_player = Player::O;
    while !game_turn(&mut state, current_player) {
        current_player = match current_player {
            Player::O => Player::X,
            Player::X => Player::O,
            Player::None => panic!(),
        }
    }

    print!("Press ENTER to exit...");
    let _ = stdout().flush();

    let mut wait = String::new();
    let _ = stdin().read_line(&mut wait);
}

/// Displays the game state.
/// Takes and processes user input.
/// Returns `true` if the game is won.
fn game_turn(state: &mut GameState, current_player: Player) -> bool {
    print_grid(&state);
    println!("PLAYER {}'s turn!", current_player);

    let pos: (usize, usize) = take_pos_inputs(&state);
    state[pos.0][pos.1] = current_player;

    let winner = check_victory(state);
    if winner != Player::None {
        print_grid(&state);
        println!("PLAYER {} won! Congratulations!", winner);
        return true;
    }

    false
}

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

fn take_pos_inputs(state: &GameState) -> (usize, usize) {
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

        if col > 2 || row > 2 {
            println!("Too big. Try again.");
            continue;
        }

        if state[row][col] != Player::None {
            println!("Slot already occupied. Try again.");
            continue;
        }

        return (row, col);
    }
}

fn check_victory(state: &GameState) -> Player {
    // horizontal checks
    for row in state {
        let mut horizontal_set = HashSet::new();
        horizontal_set.insert(row[0]);
        horizontal_set.insert(row[1]);
        horizontal_set.insert(row[2]);
        if horizontal_set.len() == 1 && !horizontal_set.contains(&Player::None) {
            return *horizontal_set.iter().next().unwrap();
        }
    }

    // vertical checks
    for col in 0..3 {
        let mut vertical_set = HashSet::new();
        vertical_set.insert(state[0][col]);
        vertical_set.insert(state[1][col]);
        vertical_set.insert(state[2][col]);
        if vertical_set.len() == 1 && !vertical_set.contains(&Player::None) {
            return *vertical_set.iter().next().unwrap();
        }
    }

    // diagonal checks
    // upper-left -> bottom-right
    let mut diagonal_set1 = HashSet::new();
    diagonal_set1.insert(state[0][0]);
    diagonal_set1.insert(state[1][1]);
    diagonal_set1.insert(state[2][2]);
    if diagonal_set1.len() == 1 && !diagonal_set1.contains(&Player::None) {
        return *diagonal_set1.iter().next().unwrap();
    }

    // upper-right -> bottom-left
    let mut diagonal_set2 = HashSet::new();
    diagonal_set2.insert(state[0][2]);
    diagonal_set2.insert(state[1][1]);
    diagonal_set2.insert(state[2][0]);
    if diagonal_set2.len() == 1 && !diagonal_set2.contains(&Player::None) {
        return *diagonal_set2.iter().next().unwrap();
    }

    Player::None
}
