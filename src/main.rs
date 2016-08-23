use std::cmp;
use std::fmt;
use std::ops::{Add, Sub, AddAssign, SubAssign};

extern crate rand;
use rand::random;

/// Represents a u32 that is fixed to be between 0 and some max value.
#[derive(Copy, Clone, Debug, PartialEq)]
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



#[derive(Debug, Clone, PartialEq)]
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

    fn take_damage(&mut self, damage: u32) {
        self.hp -= damage;
    }
}

impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {}, HP: {}, MP: {}", self.name, self.hp, self.mp)
    }
}

/// A structure that specifies a specific character in a Battlefield.
type CharSpecifier = u32;


// The Character direct references here are bad and wrong,
// because what happens if a character dies before an attack goes off?
// They need to be some sort of indirect reference so we can check
// whether or not it's valid.
enum Action {
    Attack(CharSpecifier, CharSpecifier),
    Defend(CharSpecifier),
    None
}

/// The central structure containing a battle's state.
#[derive(Debug, Clone)]
struct Battlefield {
    chars: Vec<Character>,
    round: u32,
}

impl fmt::Display for Battlefield {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        try!(writeln!(f, "Round {}", self.round));
        try!(writeln!(f, "Characters:"));
        for chr in &self.chars {
            try!(writeln!(f, "  {}", chr));
        };
        //try!(writeln!(f, "Mobs:"));
        //for mob in &self.mobs {
        //    try!(writeln!(f, "  {}", mob));
        //}
        write!(f, "")
    }
}

impl Battlefield {
    fn increment_round(&mut self) {
        self.round += 1
    }

    fn get<'a>(&'a self, c: CharSpecifier) -> Option<&'a Character> {
        self.chars.get(c as usize)
    }

    fn get_mut<'a>(&'a mut self, c: CharSpecifier) -> Option<&'a mut Character> {
        self.chars.get_mut(c as usize)
    }

    // fn remove_char(self, c: &Character) -> Battlefield {
    //     // This is messy but any alternative is also messy it seems, so.
    //     if self.chars.conta) {
    //         let mut chars = self.chars.clone();
    //         chars.retain(|ch| ch != c);
    //         Battlefield { chars: chars, .. self }
    //     } else if self.mobs.contains(c) {
    //         let mut mobs = self.mobs.clone();
    //         mobs.retain(|ch| ch != c);
    //         Battlefield { mobs: mobs, .. self }
    //     } else {
    //         panic!("Aieee!  Tried to remove nonexistent character!")
    //     }
    // }

    // fn replace_char(self, c: &Character, with: &Character) -> Battlefield {
    //     if self.chars.contains(c) {
    //         let mut chars = self.chars.clone();
    //         let pos = chars.iter().position(|ch| ch == c);
    //         pos.and_then(|pos| Some(chars[pos] = with.clone()));
    //         //chars[pos] = with;
    //         Battlefield { chars: chars, .. self }
    //     } else if self.mobs.contains(c) {
    //         let mut mobs = self.mobs.clone();
    //         let pos = mobs.iter().position(|ch| ch == c);
    //         pos.and_then(|pos| Some(mobs[pos] = with.clone()));
    //         Battlefield { mobs: mobs, .. self }
    //     } else {
    //         panic!("Aieee!  Tried to replace nonexistent character!")
    //     }
    // }
}

fn do_attack(field: &mut Battlefield, from: CharSpecifier, to: CharSpecifier) {
    // For now, damage equation is just:
    // damage dealt = atk/2 + [0:atk) - soak
    // soak = [0:def)
    // TODO: Better error handling here than unwrap()
    let (damage, soak) = {
        let fromchar = field.get(from).unwrap();
        let tochar = field.get(to).unwrap();
        println!("{} attacked {}!", fromchar.name, tochar.name);
        let damage_ = (rand::random::<u32>() % fromchar.atk) + (fromchar.atk / 2);
        let soak_ = rand::random::<u32>() % tochar.def;
        //println!("Damage: {}, soak: {}", damage_, soak_);
        (damage_, soak_)
    };
    if soak >= damage {
        println!("Did no damage!");
    } else {
        let resulting_damage = damage - soak;
        println!("Hit!  Did {} damage!", resulting_damage);
        let tochar = field.get_mut(to).unwrap();
        tochar.take_damage(resulting_damage);
        if !tochar.is_alive() {
            println!("{} perished!", tochar.name);
            //field.remove_char(to)
        } else {
            //field.replace_char(to, &to2)
        }
    }
}

fn do_defend(field: &mut Battlefield, who: CharSpecifier) {
    // TODO: Better error handling here.
    let whochar = field.get(who).unwrap();
    println!("{} defended themselves!", whochar.name);
}

fn do_none(field: &mut Battlefield) {
    println!("Nothing happened!");
}


fn run_action(field: &mut Battlefield, action: &Action) {
    match *action {
        Action::Attack(from, to) => do_attack(field, from, to),
        Action::Defend(who) => do_defend(field, who),
        Action::None => do_none(field),
    };
}

/// Runs a single turn in the battle.
/// It takes a battlefield state, and a list of actions
/// and applies the actions in order.
/// It returns a new Battlefield state
fn run_turn(field: &mut Battlefield, actions: Vec<Action>) {
    // We're going to want a sort-actions step, where we order the actions
    // by priority and character speed and such (defend's always take effect first, etc)
    // and THEN execute them.
    for action in actions {
        run_action(field, &action);
    }
    println!("");
    field.increment_round();
}


fn main() {
    let c1 = Character::new("Ragnar");
    let c2 = Character::new("Alena");
    let s = Character::new("Slime");
    let mut b = Battlefield {
        chars: vec![c1, c2, s],
        round: 1
    };
    let a1 = Action::Attack(0, 2);
    let a2 = Action::Attack(2, 0);
    let a3 = Action::Attack(1, 2);
    println!("{}", b);
    run_turn(&mut b, vec![a1, a2, a3]);
    println!("{}", b);
    //println!("Hello, world! {}", c);
    //c.hp -= 12;
    //println!("Bye world! {}", c);
}
