pub struct Attribute {
    value: i32,
    dependAdvantages: Vec<String>
}



impl Attribute {
    pub fn new(value: i32) -> Attribute {
        Attribute {value: value, dependAdvantages: Vec::new()}
    }

    pub fn setValue(&mut self, value: i32) {
        self.value = value;
    }

    pub fn getValue(&self) -> i32 {
        self.value
    }

    pub fn addDependAdvantage(&mut self, advantage: &str) {
        self.dependAdvantages.push(advantage.to_string());
    }

    pub fn getDependAdvantages(&self) -> &Vec<String> {
        &self.dependAdvantages
    }
}