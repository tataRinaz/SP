use node::Context;
use parser::statement;
use rustyline;
use rustyline::error::ReadlineError;
use rustyline::Editor;
mod node;
mod parser;

fn main() {
    let mut context = Context::default();

    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => match statement(line.as_bytes()) {
                Ok((b"", ast)) => {
                    rl.add_history_entry(line.as_str());
                    println!("Line: {:?}", ast);
                    println!("Evaluated: {:?}", ast.evaluate(&mut context));
                }
                Ok((input, ast)) => {
                    println!("Parsing incomplete {:?}", std::str::from_utf8(input));
                    println!("Line: {:?}", ast);
                }
                Err(error) => {
                    println!("{:?}", error);
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("history.txt").unwrap();
}
