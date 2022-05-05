use std::error::Error;

use console::Console;

mod userio;
mod commands;
mod console;

fn main() -> Result<(), Box<dyn Error>> {
    let _ = clap::Command::new("Console")
        .version(env!("CARGO_PKG_VERSION"))
        .author("João Foscarini <jfoscarini@gmail.com>")
        .about("This program reads user input and calls some function if it matches any of the commands' many possible predefined patterns.")
        .get_matches();

    Console::default()
        .register_commands()
        .run()?;

    Ok(())
}
