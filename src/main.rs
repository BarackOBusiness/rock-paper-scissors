mod game;

use std::cmp::Ordering;
use std::io::{self, Write};

use crate::game::{HandShape, Player};

fn main() {
    println!("Welcome to the game of rock paper scissors.");
    println!("In this program you will be playing against a computer.");

    print!("Enter your name here: ");
    let _ = io::stdout().flush();

    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let player: Player = Player::new(&buffer.trim().to_string());
    let computer: Player = Player::new(&"Computer".to_string());

    println!("The game goes to the best of n rounds.");
    print!("How many would you like that to be? ");
    io::stdout().flush().unwrap();

    buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let buffer: u32 = buffer.trim().parse().unwrap();

    let winner = game_loop(player, computer, buffer);

    println!("And the winner is:\n{}", winner.name());
}

// So if something takes ownership of another var, the original doesn't need to be mutable
// if the parameter is mutable?  Interesting.
fn game_loop(mut p: Player, mut cpu: Player, rounds: u32) -> Player {
    let score_to: u32 = rounds / 2;

    while p.score() <= score_to && cpu.score() <= score_to {
        cpu.select(HandShape::random());
        let mut buffer: String = String::new();

        loop {
            println!("Rock! Paper! Scissors! Shoot!");
            io::stdin().read_line(&mut buffer).unwrap();

            let process: String = buffer.to_lowercase();
            let process: &str = process.trim();

            buffer.clear();

            match HandShape::resolve_from(process) {
                Ok(x) => {
                    p.select(x);
                    break;
                }
                Err(e) => println!("{}", e),
            }
        }

        println!("{}", cpu.selection);

        match p.selection.partial_cmp(&cpu.selection).unwrap() {
            Ordering::Equal => tie(),
            Ordering::Greater => win(&mut p, &cpu),
            Ordering::Less => loss(&p, &mut cpu),
        }
    }

    if p.score() > cpu.score() {
        p
    } else {
        cpu
    }
}

fn tie() {
    println!("You both shoot the same hand.");
}

fn win(p: &mut Player, cpu: &Player) {
    println!(
        "Your {} beats your opponent's {}",
        p.selection, cpu.selection
    );
    p.incr_score();
}

fn loss(p: &Player, cpu: &mut Player) {
    println!(
        "Your opponent's {} beats your {}",
        cpu.selection, p.selection
    );
    cpu.incr_score();
}
