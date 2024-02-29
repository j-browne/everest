use everest::{evaluate, parse};
use std::io::stdin;

fn main() {
    for line in stdin().lines() {
        let line = line.unwrap_or_else(|e| {
            panic!("IO Error: {e}");
        });

        if line.is_empty() {
            continue;
        }

        let (input, parsed_line) = match parse(&line) {
            Ok((input, parsed_line)) => (input, parsed_line),
            Err(e) => {
                println!("  ERROR: {e}");
                continue;
            }
        };

        if !input.is_empty() {
            println!("  WARNING: input remaining {input:?}");
        }

        let result = evaluate(parsed_line);
        match result {
            Ok(result) => println!("{result}"),
            Err(e) => println!("  ERROR: {e}"),
        }
    }
}
