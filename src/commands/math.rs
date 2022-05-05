use std::{error::Error, collections::VecDeque};

use colored::Colorize;

use crate::{console::{Console}, userio::UserIO};

use super::{Commands, Command, CommandArg};

pub struct CmdMath;
impl CmdMath {
    fn sum(_console: &mut Console, args: &mut VecDeque<String>) -> Result<(), Box<dyn Error>> {
        let a = args.pop_front().unwrap().parse::<i32>();
        let b = args.pop_front().unwrap().parse::<i32>();

        if a.is_ok() && b.is_ok() { println!("{}", a? + b?); }
        else { UserIO::print_error(String::from("could not convert informed values to i32")); }

        Ok(())
    }

    fn sub(_console: &mut Console, args: &mut VecDeque<String>) -> Result<(), Box<dyn Error>> {
        let a = args.pop_front().unwrap().parse::<i32>();
        let b = args.pop_front().unwrap().parse::<i32>();

        if a.is_ok() && b.is_ok() { println!("{}", a? - b?); }
        else { UserIO::print_error(String::from("could not convert informed values to i32")); }

        Ok(())
    }
}

impl Command for CmdMath {
    fn register(cmds: &mut Commands) {
        cmds.register_command_meta("math", Self::desc, Self::usage);
        cmds.register_pattern("math", "math_sum", Self::sum, vec![
            CommandArg::Some,
            CommandArg::Keyword("+"),
            CommandArg::Some,
        ]);
        cmds.register_pattern("math", "math_sub", Self::sub, vec![
            CommandArg::Some,
            CommandArg::Keyword("-"),
            CommandArg::Some,
        ]);
    }

    fn desc() { UserIO::print_desc(String::from("Sum or subtract two numbers")); }
    fn usage() { UserIO::print_usage(
        format!("{} {} <+/-> {}", "math", "value_one".yellow(), "value_two".yellow())
    ); }
}
