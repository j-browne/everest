use everest::{evaluate, parse};
use rustyline::{error::ReadlineError, history::MemHistory, Config, Editor};

fn main() {
    let mut rl: Editor<(), MemHistory> =
        Editor::with_history(Config::default(), MemHistory::new()).unwrap();

    loop {
        match rl.readline("") {
            Ok(line) => {
                if line.is_empty() {
                    continue;
                }

                let _ = rl.add_history_entry(line.clone());

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

                match evaluate(parsed_line) {
                    Ok(result) => println!("{result}"),
                    Err(e) => println!("  ERROR: {e}"),
                }

                continue;
            }
            Err(ReadlineError::Eof) => break,
            Err(ReadlineError::Interrupted) => continue,
            Err(e) => {
                println!("Readline Error: {e}");
                break;
            }
        }
    }
}
