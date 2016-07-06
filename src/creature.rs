use super::attribute::Attribute;
use super::health;
pub use health::DamageType;

use std::collections::HashMap;

pub type AdvantagesFactories = HashMap<String, &'static Fn(&Creature) -> i32>;



pub struct Creature<'a> {
    attributes: HashMap<String, Attribute>,
    skills: HashMap<String, i32>,
    advantages: HashMap<String, i32>,
    advantagesFactories: &'a AdvantagesFactories,
    health: health::Health
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
                                 health: health::Health::new(1)};

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

    pub fn setAttribute(&mut self, name: &str, value: i32) {
        if let Some(attribute) = self.attributes.get_mut(name) {
            attribute.setValue(value);
        
        } else {
            return
        }

        for i in self.attributes.get(name).unwrap().getDependAdvantages() {
            let val = self.advantagesFactories.get(i).unwrap()(&self);
            self.advantages.get_mut(i).map(|v| { *v = val });
        }
    }



    pub fn addDependAdvantage(&mut self, advantageName: &str, 
                                         attributeName: &str) {
        if let Some(attribute) = self.attributes.get_mut(attributeName) {
            attribute.addDependAdvantage(advantageName);
        }
    }



    pub fn getAttribute(&self, name: &str) -> Option<&Attribute> {
        self.attributes.get(name)
    }



    pub fn getAdvantage(&self, name: &str) -> Option<i32> {
        self.advantages.get(name).map(|v| *v )
    }
}



const healthFactory: &'static Fn(&Creature)->i32 = &|character| {
    character.getAttribute("Strength").unwrap().getValue()+3
};



pub fn initAdvantagesFactories() -> AdvantagesFactories {
    let mut factories = AdvantagesFactories::new();
    factories.insert("Health".to_string(), healthFactory);
    factories
}