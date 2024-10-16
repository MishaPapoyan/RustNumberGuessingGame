use std::io::stdin;
use rand::Rng;
use colored::*;

struct HighScore {
    name: String,
    attempts: i32,
}

impl HighScore {
    fn add_score(name: String, attempts: i32) -> HighScore {
        HighScore {
            name,
            attempts,
        }
    }
}

fn main() {
    let mut scores: Vec<HighScore> = Vec::new();

    loop {
        println!("Enter your name (or type 'exit' to quit):");
        let mut player_name = String::new();
        stdin().read_line(&mut player_name).expect("Failed to read line");
        let player_name = player_name.trim().to_string();

        // Check for exit condition
        if player_name.to_lowercase() == "exit" {
            println!("Do you want to see the leaderboard? (y/n)");

            let mut line_for_table = String::new();
            stdin().read_line(&mut line_for_table).expect("Failed to read line");
            let line_for_table = line_for_table.trim().to_string();
            if line_for_table.to_lowercase() == "y" {
                // Sort scores and display
                scores.sort_by(|a, b| a.attempts.cmp(&b.attempts));
                println!("\nFinal Scores:");
                for score in &scores {
                    println!("{}: {} attempts", score.name.green(), score.attempts.to_string().red());
                }
                // Display the winner after the loop
                if let Some(winner) = scores.first() {
                    println!("{} {}", "The winner is".blue(), winner.name.green());
                }
                break;
            } else {
                break;
            }
        }

        let number: i32 = rand::thread_rng().gen_range(1..=100);
        let mut attempts: i32 = 0;

        println!("Pick a number between 1 and 100 >>>");

        loop {
            let mut line = String::new();
            let input = stdin().read_line(&mut line);
            let guess: Option<i32> = input.ok().map_or(None, |_| line.trim().parse().ok());

            match guess {
                None => println!("Please input a valid number."),
                Some(n) if n == number => {
                    attempts += 1;
                    println!("Congratulations, you guessed the number {} in {} attempts!", number, attempts);

                    let score = HighScore::add_score(player_name.clone(), attempts);
                    scores.push(score);
                    break;
                }
                Some(n) if n < number => {
                    attempts += 1;
                    println!("Attempt {}: Too low!", attempts);
                }
                Some(n) if n > number => {
                    attempts += 1;
                    println!("Attempt {}: Too high!", attempts);
                }
                Some(_) => println!("An error occurred."),
            }
        }
    }
}
