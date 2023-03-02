use std::path::PathBuf;

use clap::{arg, command, value_parser};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

use lispers_common::{backend::DefaultBackend, symbol::SymbolUsize};
use lispers_backend::Interpreter;

type Symbol = SymbolUsize;
type Backend = DefaultBackend<Symbol>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let matches = command!()
    .arg(
      arg!(
        -i --input <FILE> "Execute file before starting the REPL"
      )
      .required(false)
      .value_parser(value_parser!(PathBuf))
    )
    .get_matches();

  let mut interpreter: Interpreter<Symbol, Backend> = Interpreter::new();
  let env = interpreter.default_env();

  if let Some(input_path) = matches.get_one::<PathBuf>("input") {
    if let Err(err) = interpreter.eval_file(env.clone(), input_path) {
      eprintln!("{}", err);
      std::process::exit(1);
    }
  }

  let mut rl = DefaultEditor::new()?;

  loop {
    match rl.readline(">>> ") {
      Ok(line) => {
        if let Err(err) = rl.add_history_entry(line.as_str()) {
          eprintln!("ReadlineError: {:?}", err);
        }

        match interpreter.eval_string(env.clone(), &line) {
          Ok(value) => {
            println!("{}", interpreter.format_value(&value));
          },
          Err(err) => {
            eprintln!("{}", err);
          },
        }
      },
      Err(ReadlineError::Interrupted) => {},
      Err(ReadlineError::Eof) => {
        break;
      },
      Err(err) => {
        eprintln!("ReadlineError: {:?}", err);
      }
    }
  }

  Ok(())
}
