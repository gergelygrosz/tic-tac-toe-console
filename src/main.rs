mod state;

use crate::state::GameState;
use crate::state::Player;
use crate::state::print_grid;

use std::collections::HashSet;
use std::io::{Write, stdin, stdout};

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
    print_grid(state);
    println!("PLAYER {current_player}'s turn!");

    let pos: (usize, usize) = take_pos_inputs(state);
    state[pos.0][pos.1] = current_player;

    let winner = check_victory(state);
    if winner != Player::None {
        print_grid(state);
        println!("PLAYER {winner} won! Congratulations!");
        return true;
    }

    if is_draw(state) {
        print_grid(state);
        println!("This game is a draw!");
        return true;
    }

    false
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

fn is_draw(state: &GameState) -> bool {
    let mut unwinnables = 0;

    // horizontal checks
    for row in state {
        let mut horizontal_set = HashSet::new();
        horizontal_set.insert(row[0]);
        horizontal_set.insert(row[1]);
        horizontal_set.insert(row[2]);
        if horizontal_set.len() == 3
            || (horizontal_set.len() == 2 && !horizontal_set.contains(&Player::None))
        {
            unwinnables += 1;
        }
    }

    // vertical checks
    for col in 0..3 {
        let mut vertical_set = HashSet::new();
        vertical_set.insert(state[0][col]);
        vertical_set.insert(state[1][col]);
        vertical_set.insert(state[2][col]);
        if vertical_set.len() == 3
            || (vertical_set.len() == 2 && !vertical_set.contains(&Player::None))
        {
            unwinnables += 1;
        }
    }

    // diagonal checks
    // upper-left -> bottom-right
    let mut diagonal_set1 = HashSet::new();
    diagonal_set1.insert(state[0][0]);
    diagonal_set1.insert(state[1][1]);
    diagonal_set1.insert(state[2][2]);
    if diagonal_set1.len() == 3
        || (diagonal_set1.len() == 2 && !diagonal_set1.contains(&Player::None))
    {
        unwinnables += 1;
    }

    // upper-right -> bottom-left
    let mut diagonal_set2 = HashSet::new();
    diagonal_set2.insert(state[0][2]);
    diagonal_set2.insert(state[1][1]);
    diagonal_set2.insert(state[2][0]);
    if diagonal_set1.len() == 3
        || (diagonal_set2.len() == 2 && !diagonal_set2.contains(&Player::None))
    {
        unwinnables += 1;
    }

    // the total amount of win positions is 8
    unwinnables == 8
}
