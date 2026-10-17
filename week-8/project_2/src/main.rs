// Project II: The EY Interview
// Find the candidate with the highest number of years of programming experience.
// The candidate records are stored in a vector of tuples: (name, years).

use std::io::{self, Write};

fn read_line(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Could not display prompt");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read input");
    input.trim().to_string()
}

fn main() {
    println!("Welcome to the Ernst and Young Global Limited software");
    let number_of_candidates: usize = loop {
        let input = read_line("\nHow many candidates are being interviewed? ");
        match input.parse() {
            Ok(number) if number > 0 => break number,
            _ => println!("Please enter a whole number greater than zero."),
        }
    };

    let mut candidates: Vec<(String, u32)> = Vec::new();

    // Collect each candidate's name and years of programming experience.
    for index in 0..number_of_candidates {
        println!("\nCandidate {}", index + 1);

        let name = loop {
            let input = read_line("Name: ");
            if !input.is_empty() {
                break input;
            }
            println!("Name cannot be empty.");
        };

        let years: u32 = loop {
            let input = read_line("Years of programming experience: ");
            match input.parse() {
                Ok(years) => break years,
                Err(_) => println!("Please enter a whole number of years."),
            }
        };

        candidates.push((name, years));
    }

    // Walk through the vector and retain the candidate with the highest years.
    let mut most_experienced = &candidates[0];
    for candidate in &candidates[1..] {
        if candidate.1 > most_experienced.1 {
            most_experienced = candidate;
        }
    }

    println!(
        "\nThe candidates with the most programming experience ({} years) are:",
        most_experienced.1
    );

    // Print every candidate whose experience matches the highest value.
    for (name, years) in &candidates {
        if *years == most_experienced.1 {
            println!("{}", name);
        }
    }
}
