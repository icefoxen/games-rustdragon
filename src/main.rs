use std::cmp;
use std::fmt;
use std::ops::{Add, Sub, AddAssign, SubAssign};

extern crate rand;
use rand::random;

/// Represents a u32 that is fixed to be between 0 and some max value.
#[derive(Copy, Clone, Debug)]
struct BoundedNumber {
    val: u32,
    max: u32,
}

impl BoundedNumber {
    fn new(max:u32) -> BoundedNumber {
        BoundedNumber {
            val: max,
            max: max,
        }
    }
}

impl Add<u32> for BoundedNumber {
    type Output = BoundedNumber;
    fn add(self, _rhs: u32) -> BoundedNumber {
        BoundedNumber {
            max: self.max,
            val: cmp::min(self.max, self.val.saturating_add(_rhs)),
        }
    }
}

impl AddAssign<u32> for BoundedNumber {
    fn add_assign(&mut self, _rhs: u32) {
        self.val = (*self + _rhs).val;
    }
}


impl Sub<u32> for BoundedNumber {
    type Output = BoundedNumber;
    fn sub(self, _rhs: u32) -> BoundedNumber {
        BoundedNumber {
            max: self.max,
            val: cmp::max(0, self.val.saturating_sub(_rhs)),
        }
    }
}

impl SubAssign<u32> for BoundedNumber {
    fn sub_assign(&mut self, _rhs: u32) {
        self.val = (*self - _rhs).val;
    }
}

impl fmt::Display for BoundedNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.val, self.max)
    }
}

#[test]
fn bounded_number_is_bounded() {
    let max = 100;
    let x = BoundedNumber::new(max);
    assert!(x.val == max);
    let y = x + 100;
    assert!(y.val == max);
    let z = x - 9999;
    assert!(z.val == 0);
}



#[derive(Debug, Clone)]
struct Character {
    name: String,
    hp: BoundedNumber,
    mp: BoundedNumber,

    // Stats.
    // Attack, how much damage you do
    atk: u32,
    // Defense, how much you reduce damage
    def: u32,
    // Speed, determines who goes first
    spd: u32,
    // Luck, determines critical hit chance
    lck: u32,

}


impl Character {
    fn new(name: &str) -> Character {
        Character {
            name: String::from(name),
            hp: BoundedNumber::new(10),
            mp: BoundedNumber::new(10),

            atk: 10,
            def: 10,
            spd: 10,
            lck: 10

        }
    }

    fn is_alive(&self) -> bool {
        self.hp.val > 0
    }

    fn take_damage(self, damage: u32) -> Character {
        Character {
            hp: self.hp - damage,
            .. self
        }
    }
}

impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {}, HP: {}, MP: {}", self.name, self.hp, self.mp)
    }
}

enum Action<'a> {
    Attack(&'a Character, &'a Character),
    Defend(&'a Character),
    None
}

/// The central structure containing a battle's state.
#[derive(Debug, Clone)]
struct Battlefield {
    chars: Vec<Character>,
    mobs: Vec<Character>,
    round: u32,
}

impl fmt::Display for Battlefield {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        try!(writeln!(f, "Round {}", self.round));
        try!(writeln!(f, "Characters:"));
        for chr in &self.chars {
            try!(writeln!(f, "  {}", chr));
        };
        try!(writeln!(f, "Mobs:"));
        for mob in &self.mobs {
            try!(writeln!(f, "  {}", mob));
        }
        write!(f, "")
    }
}

impl Battlefield {
    fn increment_round(self) -> Battlefield {
        Battlefield { round: self.round + 1, .. self }
    }

    fn remove_char(self, c: &Character) -> Battlefield {
        self
    }

    fn replace_char(self, c: &Character, with: &Character) -> Battlefield {
        self
    }
}

fn do_attack(field: Battlefield, from: &Character, to: &Character) -> Battlefield {
    println!("{} attacked {}!", from.name, to.name);
    // For now, damage equation is just:
    // damage dealt = atk/2 + [0:atk) - soak
    // soak = [0:def)
    let raw_damage = (rand::random::<u32>() % from.atk) + (from.atk / 2);
    let soak = rand::random::<u32>() % to.def;
    if soak >= raw_damage {
        println!("Did no damage!");
        field
    } else {
        let damage = raw_damage - soak;
        println!("Hit!  Did {} damage!", damage);
        let to2 = to.clone().take_damage(damage);
        if !to2.is_alive() {
            println!("{} perished!", to.name);
            field.remove_char(to)
        } else {
            field.replace_char(to, &to2)
        }
    }
}

fn do_defend(field: Battlefield, who: &Character) -> Battlefield {
    println!("{} defended themselves!", who.name);
    field
}

fn do_none(field: Battlefield) -> Battlefield {
    println!("Nothing happened!");
    field
}


fn run_action(field: Battlefield, action: &Action) -> Battlefield {
    let f = match *action {
        Action::Attack(from, to) => do_attack(field, from, to),
        Action::Defend(who) => do_defend(field, who),
        Action::None => do_none(field),
    };
    f.clone()
}

/// Runs a single turn in the battle.
/// It takes a battlefield state, and a list of actions
/// and applies the actions in order.
/// It returns a new Battlefield state
fn run_turn(field: Battlefield, actions: Vec<Action>) -> Battlefield {
    // We're going to want a sort-actions step, where we order the actions
    // by priority and character speed and such (defend's always take effect first, etc)
    // and THEN execute them.
    let f = actions.iter()
        .fold(field, run_action);
    println!("");
    f.increment_round()
}


fn main() {
    let c1 = Character::new("Ragnar");
    let c2 = Character::new("Alena");
    let s = Character::new("Slime");
    let b = Battlefield {
        chars: vec![c1, c2],
        mobs: vec![s],
        round: 1
    };
    let a1 = Action::Attack(&b.chars[0], &b.mobs[0]);
    let a2 = Action::Defend(&b.mobs[0]);
    println!("{}", b);
    let b_ = run_turn(b.clone(), vec![a1, a2]);
    println!("{}", b_);
    //println!("Hello, world! {}", c);
    //c.hp -= 12;
    //println!("Bye world! {}", c);
}
