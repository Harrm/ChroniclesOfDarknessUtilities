use super::Character;
use std;
pub type Factory = Fn(&Character)->u8;



const health: &'static Factory = &|c: &Character| {
    c.getAttribute("Stamina").unwrap_or(0) + c.getSize()
};
const willpower: &'static Factory = &|c: &Character| {
    c.getAttribute("Resolve").unwrap_or(0) + 
    c.getAttribute("Composure").unwrap_or(0)
};
const initiative: &'static Factory = &|c: &Character| {
    c.getAttribute("Dexterity").unwrap_or(0) +
    c.getAttribute("Composure").unwrap_or(0)
};
const speed: &'static Factory = &|c: &Character| {
    c.getAttribute("Stamina").unwrap_or(0) +
    c.getAttribute("Strength").unwrap_or(0) + 5
};
const defence: &'static Factory = &|c: &Character| {
    std::cmp::min(c.getAttribute("Wits").unwrap_or(0),
                  c.getAttribute("Dexterity").unwrap_or(0))
};



pub fn get(name: &str) -> Option<&Factory> {
    match name {
        "Health" => Some(health),
        "Speed" => Some(speed),
        "Willpower" => Some(willpower),
        "Defence" => Some(defence),
        "Initiative" => Some(initiative),
        _ => None,
    }
}