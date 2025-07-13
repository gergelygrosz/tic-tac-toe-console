use std::io::{self, Write, stdout};

use grid::Player;

mod grid;

fn main() {
    let mut state: [[Player; 3]; 3] = [[Player::None; 3]; 3];
    grid::print_grid(&state);

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
    // TODO: clear stdout display at the start of every turn
    println!("PLAYER {}'s turn!", current_player);

    // TODO: make it so that players can't choose and overwrite non-empty slots
    let row: usize = take_position_input("input: position: row: ");
    let col: usize = take_position_input("input: position: col: ");

    state[row - 1][col - 1] = current_player;
    grid::print_grid(&state);

    if is_victory(&state, Player::O) {
        println!("PLAYER {} won! Congratulations!", Player::O);
        return true;
    };

    if is_victory(&state, Player::X) {
        println!("PLAYER {} won! Congratulations!", Player::X);
        return true;
    }

    false
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
