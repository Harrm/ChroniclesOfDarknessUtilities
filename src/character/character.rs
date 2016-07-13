// use std;
use std::collections::HashMap;
use super::advantages_factories;



pub struct Character {
    name: String, 
    experience: u16,
    attributes: HashMap<String, u8>,
    skills: HashMap<String, u8>,
    size: u8
}



pub static AttributesNames: [&'static str; 9] = 
                ["Strength", "Dexterity", "Stamina",
                 "Intelligence", "Wits", "Resolve",
                 "Presence", "Manipulation", "Composure"];
pub static SkillsNames: [&'static str; 24] = 
                ["Academics", "Science", "Medicine", "Investigation", "Occult",
                 "Politics", "Craft", "Computer",
                 "Weaponry", "Firearms", "Brawl", "Stealth", "Drive", 
                 "Survive", "Athletics", "Larceny",
                 "Persuasion", "Intimidation", "Socialize", "Streetwise",
                 "Anilmal Ken", "Empathy", "Expression", "Subterfuge"];
pub static AdvantagesNames: [&'static str; 5] = 
                ["Health", "Defence", "Initiative", "Speed", "Willpower"];

impl Character {
    pub fn getName(&self) -> &str {
       &self.name
    }

    pub fn getAttributes(&self) -> &HashMap<String, u8> {
        &self.attributes
    }

    pub fn getAttribute(&self, name: &str) -> Option<u8> {
        self.attributes.get(name).map(|v| *v)
    }

    pub fn setSkills(&self) -> &HashMap<String, u8> {
        &self.skills
    }

    pub fn getSkill(&self, name: &str) -> Option<u8> {
        self.skills.get(name).map(|v| *v)
    }

    pub fn getSize(&self) -> u8 {
        self.size
    }

    pub fn getAdvantage(&self, name: &str) -> Option<u8> {
        advantages_factories::get(name).map(|f| f(&self))
    }
}



use rustc_serialize::Decodable;
use rustc_serialize::Decoder;



impl Decodable for Character {
    fn decode<D: Decoder>(d: &mut D) -> Result<Character, D::Error> {
        d.read_struct("Character", 5, |d| {
            let name = try!(d.read_struct_field("name", 0, |d| d.read_str()));
            let size = d.read_struct_field("size", 1, |d| Ok(d.read_u8().unwrap_or(5))).ok().unwrap();
            let experience = d.read_struct_field("experience", 2, |d| Ok(d.read_u16().unwrap_or(0))).ok().unwrap();
            let attributes = try!(d.read_struct_field("attributes", 3, 
                |d| d.read_map(|d, size| {
                    let mut map = HashMap::new();
                    for i in 0..size {
                        let key = try!(d.read_map_elt_key(i, |d| d.read_str()));
                        let value = try!(d.read_map_elt_val(i, |d| d.read_u8()));
                        map.insert(key, value);
                    }
                    Ok(map)
                })
            ));
            let skills = try!(d.read_struct_field("skills", 4, 
                |d| d.read_map(|d, size| {
                    let mut map = HashMap::new();
                    for i in 0..size {
                        let key = try!(d.read_map_elt_key(i, |d| d.read_str()));
                        let value = try!(d.read_map_elt_val(i, |d| d.read_u8()));
                        map.insert(key, value);
                    }
                    Ok(map)
                })
            ));
            Ok(Character{ name: name, attributes: attributes, skills: skills,
                          size:size, experience: experience })
        })
    }
}
