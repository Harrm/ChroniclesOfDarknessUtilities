use super::creature::Creature;
use std::collections::HashMap;

pub type Factory = &'static Fn(&Creature) -> i32;
pub type AdvantagesFactories = HashMap<String, Factory>;



const HealthFactory: Factory = &|character| {
    (character.getSize() + character.getAttribute("Strength").unwrap_or(0)) as i32
};



pub fn initAdvantagesFactories() ->&'static AdvantagesFactories {
    static mut factories: AdvantagesFactories = AdvantagesFactories::new();
    factories.insert("Health".to_string(), HealthFactory);
    &factories
}