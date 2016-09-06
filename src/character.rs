use std::fmt;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::cmp;

use super::bounded_number::BoundedNumber;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Team {
    Player,
    Monster
}

/// A structure that contains every possible buff
/// because there's no damn reason to manaeg them
/// individually...?
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuffType {
    Defend
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub struct BuffEffect {
//     pub turns_left: u32
// }

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

    // Buff type, duration
    pub buffs: HashMap<BuffType, u32>,
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
            lck: 10,

            buffs: HashMap::new()

        }
    }

    pub fn is_alive(&self) -> bool {
        self.hp.val > 0
    }

    pub fn take_damage(&mut self, damage: u32) {
        self.hp -= damage;
    }

    /// Buffs have a timer,
    /// so this increments all the timers
    /// and removes the buffs that have timed out.
    pub fn tick_buffs(&mut self) {
        // Might be a better way of doing this,
        // but it works.
        let mut buffs_to_remove : Vec<BuffType> = Vec::new();
        for (buff, turns_left) in self.buffs.iter_mut() {
            //println!("Buff {:?} has {} turns left", buff, turns_left);
            if *turns_left == 0 {
                // remove buff
                buffs_to_remove.push(*buff);
            } else {
                *turns_left -= 1;
            }
        }
        for buff in buffs_to_remove {
            self.buffs.remove(&buff);
        }
    }

    /// Add a buff on a character.
    /// If it already exists, take the one with the longer
    /// duration.
    /// So, two instances of the same buff don't stack, it just
    /// renews the buff to the max duration given.
    pub fn add_buff(&mut self, buff: BuffType, duration: u32) {
        //self.buffs.insert(buff, duration);
        let entry = self.buffs.entry(buff);
        match entry {
            Entry::Vacant(_) => {
                entry.or_insert(duration);
            },
            Entry::Occupied(e) => {
                let val = e.into_mut();
                let max = cmp::max(*val, duration);
                *val = max;

            }
        }
    }

    pub fn has_buff(&self, buff: BuffType) -> bool {
        self.buffs.contains_key(&buff)
    }
}

impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {}, HP: {}, MP: {}", self.name, self.hp, self.mp);
        if self.buffs.len() > 0 {
            write!(f, " {:?}", self.buffs)
        } else {
            write!(f, "")
        }
    }
}

#[test]
fn random_char_methods() {
    let mut c = Character::new("Bob", Team::Monster);
    assert!(c.is_alive());
    c.take_damage(1_000_000);
    assert!(!c.is_alive());
}


