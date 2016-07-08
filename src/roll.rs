use rand;



struct DicePool {
    dices: Vec<u16>,
}



pub fn roll(dice: u16, num: u32, reroll_value: u8) -> DicePool {
    let mut pool = Vec::new();
    for i in 1..num {
        let result = rand::random() % dice;
        pool.push(result);
        if result >= reroll_value {
            num++;
        }
    }

    DicePool {dices: pool}
}



pub fn roll10(num: u32, reroll_value: u8) -> DicePool {
    roll(10, num, reroll_value)
}



pub fn roll10(num: u32) -> DicePool {
    roll10(num)
}
