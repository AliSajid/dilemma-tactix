// SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::{
    cmp::Ordering,
    io::{
        self,
        Write,
    },
};

use dilemma_tactix_lib::{
    Choice,
    GameGrid,
    GameOptions,
};
use rand::{
    seq::SliceRandom,
    SeedableRng,
};

fn parse_choice(choice: &str) -> Choice {
    match choice {
        "A" => Choice::Atlantis,
        "B" => Choice::Olympus,
        _ => {
            println!("Invalid choice, defaulting to A");

            Choice::Atlantis
        }
    }
}

fn get_computer_choice(seed: Option<u64>) -> Choice {
    let mut rng = rand_chacha::ChaCha12Rng::seed_from_u64(seed.unwrap_or(0));

    let choices = ["A", "B"];

    let choice = choices.choose(&mut rng).unwrap_or_else(|| &choices[0]);

    parse_choice(choice)
}

pub fn game_loop(game_options: GameOptions, game_grid: GameGrid) {
    game_grid.show_grid();

    println!("The choices available to you are: ");

    println!("A: {}", game_options.choice_atlantis());

    println!("B: {}", game_options.choice_olympus());

    let choice = read_user_input("Enter your choice (A or B): ");

    let choice = parse_choice(choice.trim());

    let computer_choice = get_computer_choice(None);

    let result = game_grid.return_score(choice, computer_choice);

    println!("You chose: {choice}");

    println!("The computer chose: {computer_choice}");

    println!(
        "Your Score: {}\nComputer Score: {}",
        result.first(),
        result.second()
    );

    match result.first().cmp(&result.second()) {
        Ordering::Greater => println!("You win!"),
        Ordering::Less => println!("The computer wins!"),
        Ordering::Equal => println!("It's a tie!"),
    }
}

fn read_user_input(prompt: &str) -> String {
    print!("{prompt}");
    match io::stdout().flush() {
        Ok(()) => (),
        Err(e) => eprintln!("Failed to flush stdout: {e}"),
    }

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => (),
        Err(e) => eprintln!("Failed to read line: {e}"),
    }

    input
}

fn main() {
    let game_options = GameOptions::builder("randomized").build();

    let game_grid = GameGrid::new(game_options);

    println!("Welcome to Dilemma Tactix!");

    loop {
        game_loop(game_options, game_grid);

        let play_again = read_user_input("Play again? (Y/N): ").trim().to_string();

        if play_again.to_lowercase() != "y" {
            break;
        }
    }

    println!("Thanks for playing!");
}
