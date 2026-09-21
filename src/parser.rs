use std::iter::Peekable;
use std::vec::IntoIter;
use std::{env, path::PathBuf};

use crate::builtin::BuiltinCommand;
use crate::lexer::Token;
use crate::executable::{get_executable_path, ExecutableCommand};

pub(crate) struct Redirection {
    path: String
}

struct Command {
    name: String,
    args: Vec<String>,
    redirection: Redirection
}

struct Pipeline {
    commands: Vec<Command>
}

struct Parser {
    tokens: Peekable<IntoIter<Token>>
}

enum ParseError {
    UnexpectedToken,
    ExpectedCommand,
    ExpectedRedirectTarget
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        Parser { 
            tokens: tokens.into_iter().peekable()
        }
    }


    fn parse(tokens: Vec<Token>) -> Pipeline {
        let mut parser = Parser::new(tokens);
        parser.parse_pipeline()
    }

    fn parse_pipeline(&self) -> Result<Pipeline, ParseError> {
        let mut commands: Vec<Command> = vec![self.parse_command()?];
        while Some(&Token::Pipe) != self.tokens.peek() {
            commands.push(self.parse_command()?);
        }
        Ok(Pipeline { commands })
    }


    fn parse_command(&mut self) -> Result<Command, ParseError> {
        let name: Token =  self.tokens.next().ok_or(ParseError::ExpectedCommand)?;
        let args: Vec<String> = Vec::new();
        let redirection: Option<Redirection> = None;
       loop {
            match self.tokens.peek() {
                Some(&Token::Word(word)) => {
                    args.push(word);
                }
                Some(&Token::RedirectOutput) => {
                    self.tokens.next(); // consume the > token
                    let Some(path: String) = self.tokens.next().ok_or(ParseError::ExpectedRedirectTarget)? else {
                        return Err(ParseError::ExpectedRedirectTarget);
                    }
                    let redirection = 
                }
           
       }}
    }


}

pub fn parse(tokens: Vec<Token>) -> Command {
    let command = tokens.
    let args: Vec<String> = tokens.map(|s| s.to_string()).collect();
    match (command, args) {
        ("exit", _) => Command::Builtin(BuiltinCommand::Exit),
        ("echo", args) => Command::Builtin(BuiltinCommand::Echo { args }),
        ("pwd", _) => Command::Builtin(BuiltinCommand::Pwd),
        ("cd", args) => parse_cd(&args),
        ("type", args) => Command::Builtin(BuiltinCommand::Type {
            arg: args.first().cloned().unwrap_or_default(),
        }),
        (command, args) => parse_executable(command, args),
    }
}



fn parse_cd(args: &Vec<String>) -> Command {
    let new_dir_str = args.first();
    let home_dir = PathBuf::from(env::var("HOME").unwrap());
    match new_dir_str {
        Some(new_dir) if *new_dir == "~" => Command::Builtin(BuiltinCommand::Cd { new_dir: home_dir }),
        Some(new_dir) => Command::Builtin(BuiltinCommand::Cd { new_dir: PathBuf::from(new_dir) }),
        None => Command::Builtin(BuiltinCommand::Cd { new_dir: home_dir }),
    }
}

fn parse_executable(command: &str, args: Vec<String>) -> Command {
    if let Some(path) = get_executable_path(command) {
        Command::Executable(ExecutableCommand {
            name: command.to_string(),
            path: path.to_string_lossy().to_string(),
            args,
        })
    } else {
        Command::Unknown {
            command: command.to_string(),
        }
    }
}

