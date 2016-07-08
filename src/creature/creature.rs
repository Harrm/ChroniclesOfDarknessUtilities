use super::attribute::Attribute;
use super::health;
use super::advantages_factories::AdvantagesFactories;
use std::collections::HashMap;

pub use super::health::DamageType;



pub struct Creature<'a> {
    attributes: HashMap<String, Attribute>,
    skills: HashMap<String, i32>,
    advantages: HashMap<String, i32>,
    advantagesFactories: &'a AdvantagesFactories,
    health: health::Health,
    size: u8
}



impl<'a> Creature<'a> {
    pub fn new(advantagesFactories: &'a AdvantagesFactories) -> Creature<'a> {
        let mut attributes = HashMap::new();
        attributes.insert("Strength".to_string(), Attribute::new(1));
        attributes.insert("Dexterity".to_string(), Attribute::new(1));
        attributes.insert("Intelligence".to_string(), Attribute::new(1));
        
        let mut skills = HashMap::new();
        skills.insert("Medicine".to_string(), -3);
        skills.insert("Armory".to_string(), -1);
        skills.insert("Empathy".to_string(), -1);

        let mut advantages = HashMap::new();
        let mut this = Creature {attributes: attributes, skills: skills, 
                                 advantages: advantages, 
                                 advantagesFactories: advantagesFactories,
                                 health: health::Health::new(1),
                                 size: 5};

        for (name, factoryMethod) in advantagesFactories {
            let val = factoryMethod(&this);
            this.advantages.insert(name.to_owned(), val);
        }
        this.health.setMax(this.advantages["Health"] as u16);
        this
    }



    pub fn damage(&mut self, damage: u16, type_: DamageType) {
        self.health.damage(damage, type_);
    }



    pub fn setAttribute(&mut self, name: &str, value: u8) {
        if let Some(attribute) = self.attributes.get_mut(name) {
            attribute.setValue(value);
        
        } else {
            return
        }
    }



    pub fn getAttribute(&self, name: &str) -> Option<u8> {
        self.attributes.get(name).map(|v| v.getValue())
    }



    pub fn getAdvantage(&self, name: &str) -> Option<i32> {
        self.advantagesFactories.get(name).map(|f| f(&self) )
    }



    pub fn getSize(&self) -> u8 {
        self.size
    }



    pub fn setSize(&mut self, size: u8) {
        self.size = size;
    }
}
