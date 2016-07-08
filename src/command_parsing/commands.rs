use super::command_parser::CommandParser;



pub trait Command {
    fn execute(&self, parser: &mut CommandParser);

    fn parse(args: &Vec<&str>) -> Box<Command> where Self: Sized;
}



pub struct ExitCommand {}

pub struct PrintCommand {text: String}

pub struct RollCommand {pool: u16, reroll_value: u8}

pub struct CheckCommand {creature: String, attribute: String, second_parameter: String,
                         modifier: i16, reroll_value: u8}

pub struct PrintAdvantageCommand {creature: String, advantage: String}

pub struct DamageCommand {creature: String, value: u16, }

pub struct HelpCommand {text: &'static str}



impl Command for PrintCommand  {
    fn execute(&self, parser: &mut CommandParser) {
        println!("{}", self.text);
    }

    fn parse(args: &Vec<&str>) -> Box<Command> {
        let text = String::new();
        for word in args.iter() {
            text += word;
            text += " ";
        }
        Box::new(PrintCommand {text: text})
    }
}



impl Command for RollCommand  {
    fn execute(&self, parser: &mut CommandParser) {
        println!("Roll");
    }

    fn parse(args: &Vec<&str>) -> Box<Command> {
        if(args.size() > 1) {
            if let Some(reroll_value) = u8::parse(args[1]) {
                RollCommand {pool: u16, reroll_value: u8}
            }
        
        } else if(args.size() > 0) {
            if let Some(pool) = u16::parse(args[0]) {
            }
        }
        Box::new( RollCommand {value: 10, reroll_value: 10} )
    }
}



impl Command for CheckCommand  {
    fn execute(&self, parser: &mut CommandParser) {

    }

    fn parse(args: &Vec<&str>) -> Box<Command> {
        for word in args.iter() {
            print!("{}", word);
        }
        Box::new( CheckCommand {creature: "Anonymous".to_string(), 
                                attribute: "Strength".to_string(), 
                                second_parameter: "Brawl".to_string(), 
                                modifier: 0, 
                                reroll_value: 10})
    }
}



impl Command for PrintAdvantageCommand  {
    fn execute(&self, parser: &mut CommandParser) {

    }

    fn parse(args: &Vec<&str>) -> Box<Command> {
        for word in args.iter() {
            print!("{}", word);
        }
        Box::new( PrintAdvantageCommand {creature: "Anonymous".to_string(), 
                                         advantage: "Health".to_string()} )
    }
}



impl Command for DamageCommand  {
    fn execute(&self, parser: &mut CommandParser) {

    }

    fn parse(args: &Vec<&str>) -> Box<Command> {
        for word in args.iter() {
            print!("{}", word);
        }
        Box::new( DamageCommand {creature: "Anonymous".to_string(), value: 1} )
    }
}



impl Command for HelpCommand  {
    fn execute(&self, parser: &mut CommandParser) {
        println!("{}", self.text);
    }

    fn parse(args: &Vec<&str>) -> Box<Command> {
        static HELP_STR: &'static str = "
                    COMMANDS:
                    exit - quit the simulation
                    print PHRASE - output some phrase on screen
                    attribute NAME - print value of some attribute
                    advantage NAME - print value of some advantage
                    set_attribute ATTR VAL - set value of some attriburte
                    damage VAL - recieve some damage
                    help - print this list
                    ";
        Box::new( HelpCommand { text: HELP_STR } )
    }
}
