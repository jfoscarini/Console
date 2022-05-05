use std::{error::Error, collections::{VecDeque, HashMap}};

use crate::{console::Console, userio::UserIO};

pub mod math;
pub mod help;
pub mod quit;

pub trait Command {
    fn register(cmds: &mut Commands);
    fn desc();
    fn usage();
}

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum CommandArg {
    Some,
    Keyword(&'static str),
}

enum CommandParseOk { Found(String, VecDeque<String>), WrongUsage(String) }
enum CommandParseError { Blank, NotFound(String) }

type CommandCall = fn(&mut Console, &mut VecDeque<String>) -> Result<(), Box<dyn Error>>;
type CommandDesc = fn();
type CommandUsage = fn();

#[derive(Default)]
pub struct Commands {
    meta: HashMap<&'static str, (CommandDesc, CommandUsage)>,
    patts: HashMap<&'static str, Vec<&'static str>>,
    calls: HashMap<&'static str, (CommandCall, Vec<CommandArg>)>,
}

impl Commands {
    pub fn execute(console: &mut Console) -> Result<(), Box<dyn Error>> {
        match console.commands.match_patterns(&mut console.userio.input) {
            Err(CommandParseError::Blank) => { /* ignore */ }
            Err(CommandParseError::NotFound(cmd)) => {
                UserIO::print_error(format!("unknown command ({})", &cmd));
            }
            Ok(CommandParseOk::WrongUsage(cmd)) => {
                UserIO::print_error(format!("unrecognized argument format ({})", &cmd));
                console.commands.print_usage(&cmd);
            }
            Ok(CommandParseOk::Found(patt, mut args)) => {
                let (handler, _) = console.commands.calls.get(patt.as_str()).unwrap();
                handler(console, &mut args)?;
            }
        }

        Ok(())
    }

    fn register_command_meta(&mut self, id: &'static str, desc: CommandDesc, usage: CommandUsage) {
        self.meta.insert(id, (desc, usage));
    }

    fn register_pattern(&mut self, cmd: &'static str, id: &'static str, call: CommandCall, args: Vec<CommandArg>) {
        match self.patts.get_mut(&cmd) {
            None => { self.patts.insert(cmd, vec![id]); }
            Some(pat) => { pat.push(id); }
        }

        self.calls.insert(id, (call, args));
    }

    fn match_patterns(&self, words: &mut VecDeque<String>) -> Result<CommandParseOk, CommandParseError> {
        let maybe_cmd = words.pop_front();
        if maybe_cmd.is_none() { return Err(CommandParseError::Blank); }
        let cmd = maybe_cmd.unwrap();

        if let Some(patts) = self.patts.get(cmd.as_str()) {

            'pat: for &pat in patts {
                let maybe_call_args = self.calls.get(pat);
                if maybe_call_args.is_none() {
                    if words.len() == 0 { return Ok(CommandParseOk::Found(pat.to_string(), words.clone())); }
                    else { continue; }
                }
                let (_, patt_args) = maybe_call_args.unwrap();

                // ensure that the number of words in the command matches with the possible pattern
                if patt_args.len() != words.len() { continue; }

                // hold arguments that are not keywords from the pattern
                let mut args_sanitized: VecDeque<String> = VecDeque::new();
                for (i, cmdarg) in patt_args.iter().enumerate() {
                    match cmdarg {
                        CommandArg::Keyword(keyword) => {
                            // ensure that the argument matches with the expected keyword
                            if keyword.ne(&words.get(i).unwrap().as_str()) { continue 'pat; }
                        }
                        CommandArg::Some => {
                            args_sanitized.push_back(words.get(i).unwrap().to_string());
                        }
                    }
                }

                return Ok(CommandParseOk::Found(pat.to_string(), args_sanitized.clone()));
            }

            return Ok(CommandParseOk::WrongUsage(cmd));
        } else { return Err(CommandParseError::NotFound(cmd)); }
    }

    fn print_desc(&self, cmd: &str) { if let Some((handler, _)) = self.meta.get(cmd) { handler(); } }
    fn print_usage(&self, cmd: &str) { if let Some((_, handler)) = self.meta.get(cmd) { handler(); } }
}
