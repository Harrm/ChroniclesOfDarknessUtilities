use super::creature::Creature;

pub struct Scene {
    creatures: Vec<Creature>
}



impl Scene {
    pub fn new() -> Scene {
        Scene { creatures: Vec::new() }
    }

    pub fn addCreature(&mut self, creature: Creature) {
        self.creatures.push(creature);
    }

    pub fn getCreature(&self, index: usize) -> &Creature {
        &self.creatures[index]
    }

    pub fn getMutCreature(&mut self, index: usize) -> &mut Creature {
        &mut self.creatures[index]
    }

}
