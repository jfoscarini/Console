use std::error::Error;

use crate::{commands::{Commands, Command, help::CmdHelp, quit::CmdQuit, math::CmdMath}, userio::{UserIO, UserIOError}};

#[derive(Default)]
pub struct Console {
    pub userio: UserIO,
    pub commands: Commands,

    pub quitting: bool,
}

impl Console {
    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            if self.quitting { break; }

            match self.userio.read_command() {
                Err(UserIOError::Interrupted) => { break; }
                Err(UserIOError::Error(e)) => { return Err(e); }
                Ok(_) => { Commands::execute(self)?; }
            }
        }

        Ok(())
    }

    pub fn register_commands(&mut self) -> &mut Self {
        CmdMath::register(&mut self.commands);
        CmdHelp::register(&mut self.commands);
        CmdQuit::register(&mut self.commands);

        self
    }
}
