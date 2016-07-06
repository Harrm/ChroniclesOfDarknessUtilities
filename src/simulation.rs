use super::command_parser;
use super::command_parser::Command;
use super::creature;
use super::scene;
use std;
use std::io;
use std::io::Write;


pub struct Simulation<'a> {
    scene: scene::Scene<'a>,
    advantagesFactories: creature::AdvantagesFactories
}



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



impl<'a> Simulation<'a> {
    pub fn new() -> Simulation<'a> {
        let advantagesFactories = creature::initAdvantagesFactories();
        let mut scene = scene::Scene::new();
        let mut this = Simulation 
            {scene: scene, advantagesFactories: advantagesFactories};
        this.scene.addCreature(creature::Creature::new(&this.advantagesFactories));
        let mut creature = this.scene.getMutCreature(0);
        creature.addDependAdvantage("Health", "Strength");
        creature.setAttribute("Strength", 7);
        this
    }



    pub fn start(&'a mut self) {
        let mut guess = String::new();
        let mut creature = self.scene.getMutCreature(0);
        loop {
            print!(">> ");
            io::stdout().flush();
            io::stdin().read_line(&mut guess).expect("Failed to read line");
            if guess.trim().is_empty() {
                continue;
            }
            let command = command_parser::proccessCommand(&guess);
            guess.clear();

            match command {
            Some(c) => match c {
                Command::Exit => break,
                Command::Print(s) => println!("{}", s),
                Command::PrintAttribute(s) => {
                    println!("{}", creature.getAttribute(&s).map_or(
                        "Invalid attribute name".to_string(), 
                        |attr| { attr.getValue().to_string() }));
                },
                Command::PrintAdvantage(s) => {
                    println!("{}", creature.getAdvantage(&s).map_or(
                        "Invalid advantage name".to_string(), 
                        |attr| { attr.to_string() }));
                },
                Command::SetAttribute(s, i) => {
                    creature.setAttribute(&s, i);
                },
                Command::Damage(u) => {
                    creature.damage(u, creature::DamageType::Bashing);
                },
                Command::Help => println!("{}", HELP_STR)
            },
            None => println!("Unknown command")
            }
        }
    }
}