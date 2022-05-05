use std::{error::Error, collections::VecDeque};

use crate::{console::Console, userio::UserIO};

use super::{Commands, Command};

pub struct CmdQuit;
impl CmdQuit {
    pub fn quit(console: &mut Console, _args: &mut VecDeque<String>) -> Result<(), Box<dyn Error>> {
        console.quitting = true;

        Ok(())
    }
}

impl Command for CmdQuit {
    fn register(cmds: &mut Commands) {
        cmds.register_command_meta("quit", Self::desc, Self::usage);

        cmds.register_pattern("quit", "quit", Self::quit, vec![]);
    }

    fn desc() { UserIO::print_desc(String::from("Quits the console")); }
    fn usage() { UserIO::print_usage(String::from("quit")); }
}
