use super::command_parsing::command_parser::CommandParser;
use super::scene::Scene;



pub struct Simulation<'a> {
    scene: Scene,
    command_parser: CommandParser<'a>,
}



impl<'a> Simulation<'a> {
    pub fn new() -> Simulation<'a> {
        let this = Simulation {scene: Scene::new(),
                               command_parser: CommandParser::new(&mut scene)};
        this
    }



    pub fn start(&'a mut self) {
        while !self.command_parser.isOver() {
            self.command_parser.proccessCommand();
        }
    }
}