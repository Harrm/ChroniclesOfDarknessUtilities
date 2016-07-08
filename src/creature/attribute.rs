pub struct Attribute {
    value: u8,
}



impl Attribute {
    pub fn new(value: u8) -> Attribute {
        Attribute {value: value}
    }

    pub fn setValue(&mut self, value: u8) {
        self.value = value;
    }

    pub fn getValue(&self) -> u8 {
        self.value
    }
}