use std;
use std::ops::AddAssign;
use std::convert::Into;
use std::fmt::Display;

#[derive(Clone, Copy, PartialEq)]
pub enum DamageType {
    None,
    Bashing,
    Lethal,
    Aggravated
}



impl Into<u8> for DamageType {
    fn into(self) -> u8 {
        match self {
            DamageType::None => 0,
            DamageType::Bashing => 1,
            DamageType::Lethal => 2,
            DamageType::Aggravated => 3
        }
    }
}



impl From<u8> for DamageType {
    fn from(val: u8) -> DamageType {
        match val {
            0 => DamageType::None,
            1 => DamageType::Bashing,
            2 => DamageType::Lethal,
            _ => DamageType::Aggravated
        }
    }
}



impl AddAssign<u8> for DamageType {
    fn add_assign(&mut self, rhs: u8) {
        *self = DamageType::from(self.clone() as u8 + rhs);
    }
}



pub struct Health {
    current: Vec<DamageType>,
    max: u16
}



impl Health {
    pub fn new(max: u16) -> Health {
        let current = vec!{DamageType::None; max as usize};
        Health { current: current, max: max}
    }

    pub fn setMax(&mut self, new_max: u16) {
        if new_max == self.max {
            return;

        } else if new_max > self.max {
            self.current.resize(new_max as usize, DamageType::None);
            self.max = new_max;
        
        } else {
            let old_max = self.max;
            self.max = new_max;
            for i in old_max..new_max {
                let damageType = self.current[i as usize];
                self.damageOnePoint(damageType);
            }
            self.current.resize(new_max as usize, DamageType::None);
        }
    }

    pub fn damage(&mut self, points: u16, damageType: DamageType) {
        for _ in 1..points+1 {
            self.damageOnePoint(damageType);
            println!("{}", self);
        }
    }

    fn damageOnePoint(&mut self, _damageType: DamageType) {
        let mut damageType = _damageType;
        let last = self.current.last().unwrap().clone();
        if last != DamageType::None {
            if last == DamageType::Aggravated {
                return;
            }
            if damageType as u8 <= last as u8 {
                damageType = DamageType::from(last as u8 + 1);
            }
        }
        for i in 0..self.current.len()-1 {
            if (self.current[i] as u8) < (damageType as u8) {
                *self.current.get_mut(i).unwrap() = damageType.clone();
                damageType = self.current[i+1];
            }
        }
        if (last as u8) < (damageType as u8) {
            *self.current.last_mut().unwrap() = damageType.clone();
        }
    }
}


impl Display for Health {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let mut result = Ok(());
        for i in &self.current {
            match write!(formatter, "{}", 
                         match *i { 
                            DamageType::None => ".",
                            DamageType::Bashing => "/",
                            DamageType::Lethal => "x",
                            DamageType::Aggravated => "*" }) {
                Ok(_) => continue,
                Err(e) => {result = Err(e); break;}
            }
        }
        result
    }
}
