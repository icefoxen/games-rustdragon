use std::fmt;


use super::bounded_number::BoundedNumber;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Team {
    Player,
    Monster
}

#[derive(Debug, Clone, PartialEq)]
pub struct Character {
    pub name: String,
    pub hp: BoundedNumber,
    pub mp: BoundedNumber,

    pub team: Team,

    // Stats.
    // Attack, how much damage you do
    pub atk: u32,
    // Defense, how much you reduce damage
    pub def: u32,
    // Speed, determines who goes first
    pub spd: u32,
    // Luck, determines critical hit chance
    pub lck: u32,
}


impl Character {
    pub fn new(name: &str, team: Team) -> Character {
        Character {
            name: String::from(name),
            hp: BoundedNumber::new(10),
            mp: BoundedNumber::new(10),

            team: team,

            atk: 10,
            def: 10,
            spd: 10,
            lck: 10

        }
    }

    pub fn is_alive(&self) -> bool {
        self.hp.val > 0
    }

    pub fn take_damage(&mut self, damage: u32) {
        self.hp -= damage;
    }
}

impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {}, HP: {}, MP: {}", self.name, self.hp, self.mp)
    }
}

#[test]
fn random_char_methods() {
    let mut c = Character::new("Bob", Team::Monster);
    assert!(c.is_alive());
    c.take_damage(1_000_000);
    assert!(!c.is_alive());
}


