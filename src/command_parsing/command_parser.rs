use super::commands::Command;
use super::commands::{PrintCommand, DamageCommand, RollCommand, 
                      CheckCommand, PrintAdvantageCommand, HelpCommand, ExitCommand};
use super::super::scene::Scene;
use std::io;



pub struct CommandParser<'a> {
    is_over: bool,
    scene: &'a mut Scene,
}



impl<'a> CommandParser<'a> {
    pub fn new(scene: &'a mut Scene) -> CommandParser<'a> {
        CommandParser {scene: scene, is_over: false}
    }



    pub fn isOver(&self) -> bool {
        self.is_over
    }



    pub fn over(&mut self) {
        self.is_over = true;
    }



    pub fn proccessCommand(&mut self) {
        if let Some(input) = Self::readCommand() {
            Self::parseCommand(input).execute();
        }
    }
    
    

    fn readCommand() -> Option<String> {
        print!(">> ");
        io::stdout().flush();
        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Failed when read line");
        
        if command.trim().is_empty() {
            None

        } else {
            Some(command)
        }
    }
    
    
    
    fn parseCommand(command: &str) -> Box<Command> {
        let mut args: Vec<&str> = Vec::new();
        for arg in command.split_whitespace() {
            args.push(arg);
        }
        Box::new(match args[0] {
            "exit" => ExitCommand {},
            "print" => PrintCommand::parse(&args),
            "check" => CheckCommand::parse(&args),
            "roll" => RollCommand::parse(&args),
            "advantage" => PrintAdvantageCommand::parse(&args),
            "damage" => DamageCommand::parse(&args),
            _ => HelpCommand {}
        })
    }
}