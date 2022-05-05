use std::{error::Error, collections::VecDeque};

use colored::Colorize;
use rustyline::{Editor, config::Configurer, error::ReadlineError};

pub enum UserIOError {
    Interrupted,
    Error(Box<dyn Error>),
}

pub struct UserIO {
    rl: Editor<()>,
    pub input: VecDeque<String>,
}

impl UserIO {
    pub fn print_desc(line: String) { println!("{} {}", "- :".bold(), line); }
    pub fn print_usage(line: String) { println!("{} {}", "USAGE:".bold(), line); }
    pub fn print_commands(line: String) { println!("{} [{}]", "COMMANDS:".bold(), line); }
    pub fn print_error(err: String) { println!("{} {}", "ERROR:".red(), err); }

    pub fn read_command(&mut self) -> Result<(), UserIOError> {
        match self.rl.readline(format!("{} ", ">>".bold()).as_str()) {
            Err(ReadlineError::Interrupted) => { Err(UserIOError::Interrupted) }
            Err(e) => { Err(UserIOError::Error(Box::new(e))) }
            Ok(line) => {
                self.input = line.to_lowercase()
                    .trim()
                    .split(" ")
                    .map(|s| s.to_string())
                    .filter(|s| !s.is_empty())
                    .collect();

                Ok(())
            }
        }
    }
}

impl Default for UserIO {
    fn default() -> Self {
        let mut rl = Editor::<()>::new();
        rl.set_auto_add_history(true);
        rl.set_history_ignore_dups(true);

        Self {
            rl,
            input: VecDeque::new(),
        }
    }
}
