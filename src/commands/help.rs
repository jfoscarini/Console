use std::{error::Error, collections::VecDeque};

use colored::Colorize;

use crate::{console::{Console}, userio::UserIO};

use super::{Commands, Command, CommandArg};

pub struct CmdHelp;
impl CmdHelp {
    fn help(console: &mut Console, _args: &mut VecDeque<String>) -> Result<(), Box<dyn Error>> {
        CmdHelp::desc();
        CmdHelp::usage();

        UserIO::print_commands(console.commands.meta.keys().into_iter()
            .map(|c| c.green().to_string())
            .collect::<Vec<_>>()
            .join(", "));

        Ok(())
    }

    pub fn help_cmd(console: &mut Console, args: &mut VecDeque<String>) -> Result<(), Box<dyn Error>> {
        let cmd = args.pop_front().unwrap();

        console.commands.print_desc(&cmd);
        console.commands.print_usage(&cmd);

        Ok(())
    }
}

impl Command for CmdHelp {
    fn register(cmds: &mut Commands) {
        cmds.register_command_meta("help", Self::desc, Self::usage);
        cmds.register_pattern("help", "help", Self::help, vec![]);
        cmds.register_pattern("help", "help_cmd", Self::help_cmd, vec![CommandArg::Some]);
    }

    fn desc() { UserIO::print_desc(String::from("Displays help message for various commands")); }
    fn usage() { UserIO::print_usage(format!("{} <?{}>", "help", "command".yellow())); }
}
