use std::cmp;
use std::fmt;
use std::ops::{Add, Sub, AddAssign, SubAssign};

/// Represents a u32 that is fixed to be between 0 and some max value.
#[derive(Copy, Clone, Debug)]
struct BoundedNumber {
    val: u32,
    max: u32
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
}


impl Character {
    fn new(name: &str) -> Character {
        Character {
            name: String::from(name),
            hp: BoundedNumber::new(10),
            mp: BoundedNumber::new(10),
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
        writeln!(f, "")
    }
}

impl Battlefield {
    fn increment_round(self) -> Battlefield {
        Battlefield { round: self.round + 1, .. self }
    }
}

fn do_attack(field: Battlefield, from: &Character, to: &Character) -> Battlefield {
    println!("{} attacked {}!", from.name, to.name);
    field
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
/// Do we want this mutable or not?
fn run_turn(field: Battlefield, actions: Vec<Action>) -> Battlefield {
    let f = actions.iter()
        .fold(field, run_action);
    f.increment_round()
}


fn main() {
    let c = Character::new("Ragnar");
    let s = Character::new("Slime");
    let b = Battlefield {
        chars: vec![c],
        mobs: vec![s],
        round: 1
    };
    let a1 = Action::Attack(&b.chars[0], &b.mobs[0]);
    let a2 = Action::Defend(&b.mobs[0]);
    println!("Battlefield: {}", b);
    let b_ = run_turn(b.clone(), vec![a1, a2]);
    println!("Battlefield: {}", b_);
    //println!("Hello, world! {}", c);
    //c.hp -= 12;
    //println!("Bye world! {}", c);
}
