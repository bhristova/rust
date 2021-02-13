use std::io::stdin;

mod board;
mod game_4x4;

fn main() {
    let dimensions = 5;
    let players = get_players();
    let mut g = game_4x4::Game4x4::new(dimensions, players.0, players.1);

    let mut iteration = 0_i32;

    let user_first = if players.0 == 'X' { true } else { false };

    if !user_first {
        g.computer_turn(iteration);
        iteration = iteration + 1;
    }

    loop {

        loop {
            g.human_turn();
            if g.game_over() {
                println!("human wins!");
                break;
            }

            g.computer_turn(iteration);
            if g.game_over() {
                println!("AI wins!");
                break;
            }
            iteration = iteration + 1;
        }

        break;
    }
}

fn get_players() -> (char, char) {
    loop {
        println!("Who starts first, write h for human and c for computer");
        let mut buffer = String::new();
        match stdin().read_line(&mut buffer) {
            Ok(_) => {
                match buffer.trim_end() {
                    "h" => return ('X', 'O'),
                    "c" => return ('O', 'X'),
                    _ => println!("Try again"),
                }
            },
            _ => println!("Try again")
        }
    }
}