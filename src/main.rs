extern crate rustc_serialize;
mod character_reader;
mod character;
mod gui;
use std::env;



fn main() {
    let args: Vec<String> = env::args().collect();

    let name = args.get(1).expect("Usage: WoD_Simulation CHARACTER_NAME");
    let character = character_reader::read_from(&name).unwrap();
    println!("{}, Strength: {}, Brawl: {}, Health: {}", 
        character.getName(),
        character.getAttribute("Strength").unwrap_or(0),
        character.getSkill("Brawl").unwrap_or(0),
        character.getAdvantage("Health").unwrap_or(0));

    if let Err(err) = gui::init() {
        println!("{}", err);
    }
    let mut window = gui::Window::new("layouts/character_sheet.glade", &character::AttributesNames, &character::SkillsNames);
    window.setAttributes(character.getAttributes());
    window.setSkills(character.setSkills());
    gui::run();
}