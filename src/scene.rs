use super::creature::Creature;

pub struct Scene<'a> {
    creatures: Vec<Creature<'a>>
}



impl<'a> Scene<'a> {
    pub fn new() -> Scene<'a> {
        Scene { creatures: Vec::new() }
    }

    pub fn addCreature(&mut self, creature: Creature<'a>) {
        self.creatures.push(creature);
    }

    pub fn getCreature(&self, index: usize) -> &Creature {
        &self.creatures[index]
    }

    pub fn getMutCreature(&mut self, index: usize) -> &'a mut Creature {
        &mut self.creatures[index]
    }

}
