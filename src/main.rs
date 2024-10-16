// Ohce
// Liz Hardee October 16 2024

// Echo but in reverse
use std::env;

fn main() {
    // grab arguments from the command line
    let args: Vec<String> = env::args().skip(1).collect();
    println!("{}", reverse(&*args[0]));
}

// reverse input and return reverse string
fn reverse(input: &str) -> String {
    input.to_string().chars().rev().collect()
} 