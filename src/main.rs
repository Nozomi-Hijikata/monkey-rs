mod ast;
mod evaluator;
mod object;
mod parser;
mod utils;

use parser::parse_program;
use std::io::{self, Write};

fn main() {
    println!("Welcome to the REPL!");
    println!("Type 'exit' to exit.");

    let mut input = String::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let trimmed_input = input.trim();

        if trimmed_input == "exit" {
            break;
        }

        match parse_program(trimmed_input) {
            Ok(parsed_program) => {
                println!("{:?}", parsed_program);
            }
            Err(err) => {
                println!("Error: {:?}", err);
            }
        }
    }

    println!("Goodbye!");
}
