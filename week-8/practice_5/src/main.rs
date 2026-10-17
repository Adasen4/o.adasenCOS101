use std::io::{self, Write};

fn main() {
    let mut cities: Vec<String> = Vec::new();

    loop {
        print!("Enter a preferred city (or 'done' to finish): ");
        io::stdout().flush().expect("Could not flush prompt");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Could not read input");
        let new_city = input.trim();

        if new_city.eq_ignore_ascii_case("done") {
            break;
        }
        if !new_city.is_empty() {
            cities.push(new_city.to_string());
        }
    }

    println!("Your preferred cities: {:?}", cities);
}
