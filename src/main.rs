use std::cmp;
use std::fmt;

extern crate rand;
//use rand::random;

extern crate rustdragon;
use rustdragon::bounded_number::BoundedNumber;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Team {
    Player,
    Monster
}

#[derive(Debug, Clone, PartialEq)]
struct Character {
    name: String,
    hp: BoundedNumber,
    mp: BoundedNumber,

    team: Team,

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
    fn new(name: &str, team: Team) -> Character {
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

    fn is_alive(&self) -> bool {
        self.hp.val > 0
    }

    fn take_damage(&mut self, damage: u32) {
        self.hp -= damage;
    }
}

#[test]
fn random_char_methods() {
    let mut c = Character::new("Bob", Team::Monster);
    assert!(c.is_alive());
    c.take_damage(1_000_000);
    assert!(!c.is_alive());
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
#[derive(Debug, Clone, Copy)]
enum Action {
    Attack(CharSpecifier, CharSpecifier),
    Defend(CharSpecifier),
}

impl Action {
    /// Defines the priority of actions, the order
    /// in which they will be taken.
    /// Higher priority goes first.
    fn priority(&self) -> i32 {
        match *self {
            Action::Attack(..) => 0,
            Action::Defend(_) => 10,
        }
    }

    fn source(&self) -> CharSpecifier {
        match *self {
            Action::Attack(from, _) => from,
            Action::Defend(who) => who,
        }
    }

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
        for chr in self.players() {
            try!(writeln!(f, "  {}", chr));
        };
        try!(writeln!(f, "Monsters:"));
        for mob in self.monsters() {
            try!(writeln!(f, "  {}", mob));
        };
        write!(f, "")
    }
}


impl Battlefield {
    fn new() -> Battlefield {
        Battlefield {
            chars: vec![],
            round: 1,
        }
    }
    fn increment_round(&mut self) {
        self.round += 1
    }

    fn get<'a>(&'a self, c: CharSpecifier) -> Option<&'a Character> {
        self.chars.get(c as usize)
    }

    fn get_mut<'a>(&'a mut self, c: CharSpecifier) -> Option<&'a mut Character> {
        self.chars.get_mut(c as usize)
    }

    // This is insane.
    // Never touch it.  The type system will eat you alive.
    // You can't define types from closures.  That's the first hiccup.
    // The type of Filter is crazy.  That's the second hiccup.
    // I still have no idea why this needs a &&Character instead of just a &Character.
    // Third, the Filter borrows the thing it's filtering, which makes the lifetimes squirrelly.
    // impl Trait is currently in nightly, and once that's useable we'll be able to make this:
    // fn players<'a>(&'a self) ->
    //    impl Iterator<Item=&'a Character> {
    //        self.chars.iter().filter(|chr| chr.team == Team::Player)
    //  } 
    fn players<'a>(&'a self) -> std::iter::Filter<std::slice::Iter<'a, Character>, fn(&&Character) -> bool> {
        // Booooo returning the results of filter() is dumb
        // 'cause you can't specify types of closures.
        // Though apparently there's a feature in nightly
        // as of August 2016 that allows you to specify a
        // trait return value rather than a specific type
        // And you can't use instance methods as if they
        // were class methods, either.

        self.get_team(Team::Player)
    }

    fn monsters<'a>(&'a self) -> std::iter::Filter<std::slice::Iter<'a, Character>, fn(&&Character) -> bool> {
        self.get_team(Team::Monster)
    }

    fn get_team<'a>(&'a self, team: Team) -> std::iter::Filter<std::slice::Iter<'a, Character>, fn(&&Character) -> bool> {
                
        fn is_player(p: &&Character) -> bool {
            p.team == Team::Player
        }
                
        fn is_monster(p: &&Character) -> bool {
            p.team == Team::Monster
        }
        match team {
            Team::Player => self.chars.iter().filter(is_player),
            Team::Monster => self.chars.iter().filter(is_monster)
        }
    }

    /// Return an iterator containing the opponents of whichever
    /// team you pass to it.
    fn opponents<'a>(&'a self, team: Team) -> std::iter::Filter<std::slice::Iter<'a, Character>, fn(&&Character) -> bool> {
        match team {
            Team::Player => self.monsters(),
            Team::Monster => self.players(),
        }
    }

    fn battle_is_over(&self) -> bool {
        self.players().filter(|x| x.is_alive()).count() == 0 ||
            self.monsters().filter(|x| x.is_alive()).count() == 0
    }
}

#[test]
fn random_battlefield_methods() {
    let mut b = Battlefield::new();
    assert!(b.round == 1);
    b.increment_round();
    assert!(b.round == 2);

    {
        let c1 = b.get(1);
        assert!(c1 == None);
    }
    
    b.chars.push(Character::new("Joe", Team::Player));
    {
        let c2 = b.get(0);
        assert!(c2 != None);
    }
}

/// Does exactly what it says on the tin.
/// If the 'to' character specified is not alive,
/// choose another target at random (that isn't on the same team)
/// and returns it.
fn choose_new_target_if_target_is_dead(field: &mut Battlefield, from: CharSpecifier, to: CharSpecifier) -> &mut Character {
    let fromteam = field.get(from).unwrap().team;
    let tochar_is_alive = field.get(to).unwrap().is_alive();
    if !tochar_is_alive {
        // Now we need to get opponents and select one at random.
        // We check if the battle is over before every action, so
        // there should always be at least *one* opponent to choose from.
        //let mut sample = {
        //    let target_team = field.opponents(fromteam);
        //    let mut rng = rand::thread_rng();
        //    rand::sample(&mut rng, target_team, 1)
        //};
        //sample.get_mut(0).unwrap()

        field.get_mut(to).unwrap()
    } else {
        field.get_mut(to).unwrap()
    }
}

fn do_attack(field: &mut Battlefield, from: CharSpecifier, to: CharSpecifier) {
    // For now, damage equation is just:
    // damage dealt = atk/2 + [0:atk) - soak
    // soak = [0:def)
    // TODO: Better error handling here than unwrap()
    let atk;
    let attacker_name;
    {
        let attacker = field.get(from).unwrap();
        atk = attacker.atk;
        // This clone is a little dumb, and we could get around
        // it by breaking the following println!() up into parts.
        // But that would be dumb and is not worth it.
        // Since we can't have a reference to attacker and defender
        // at the same time.
        attacker_name = attacker.name.clone()
    }
    let damage = (rand::random::<u32>() % atk) + (atk / 2);
    
    let defender = choose_new_target_if_target_is_dead(field, from, to);
    let soak = rand::random::<u32>() % defender.def;
    if soak >= damage {
        println!("Did no damage!");
    } else {
        let resulting_damage = damage - soak;
        println!("Hit!  Did {} damage!", resulting_damage);
        defender.take_damage(resulting_damage);
        if !defender.is_alive() {
            println!("{} perished!", defender.name);
        }
    }
}

fn do_defend(field: &mut Battlefield, who: CharSpecifier) {
    // TODO: Better error handling here.
    let whochar = field.get(who).unwrap();
    println!("{} defended themselves!", whochar.name);
}




fn run_action(field: &mut Battlefield, action: &Action) {
    // If the source of an action is dead, we skip the action.
    {
        let source = action.source();
        let sourcechar = field.get(source).unwrap();
        if !sourcechar.is_alive() {
            return;
        }
    };
        
    match *action {
        Action::Attack(from, to) => do_attack(field, from, to),
        Action::Defend(who) => do_defend(field, who),
    };

}

///Takes a Vec<Action> and reorders it into the order
/// in which they should be executed in the fight:
/// Actions have priority, highest priority ones go first
/// Then, characters with higher speed go befoer those with
/// lower speed.
fn order_actions(field: &Battlefield, actions: &mut Vec<Action>) {
    let compare_actions = |action1: &Action, action2: &Action| {
        if action1.priority() > action2.priority() {
            cmp::Ordering::Less
        } else if action1.priority() < action2.priority() {
            cmp::Ordering::Greater
        } else {
            // Actions have equal priority,
            // find out who is doing each action and go off
            // the faster one.
            let charspec1 = action1.source();
            let charspec2 = action2.source();
            let char1 = field.get(charspec1).unwrap();
            let char2 = field.get(charspec2).unwrap();
            // BUGGO:
            // This is actually slightly wrong, because the sort is stable,
            // so if the speeds are equal, the first one will always go first.
            // Ideally it should be a coin-flip, or such...
            // If we want to really do it Dragon Warrior style there should probably
            // be a bit of unpredictability in the ordering here, too.
            // Ah well, fine for now.
            char2.spd.cmp(&char1.spd)
        }
    };
    actions.sort_by(compare_actions);

}

/// Runs a single turn in the battle.
/// It takes a battlefield state, and a list of actions
/// and applies the actions in the proper order.
/// It returns true when the battle is over.
fn run_turn(field: &mut Battlefield, actions: &mut Vec<Action>) -> bool {
    // We're going to want a sort-actions step, where we order the actions
    // by priority and character speed and such (defend's always take effect first, etc)
    // and THEN execute them.
    order_actions(field, actions);
    for action in actions {
        // If the battle is over, we stop where we are!
        // Partially 'cause any remaining actions will be invalid.
        if field.battle_is_over() {
            return true;
        }

        run_action(field, &action);
    }
    println!("");
    field.increment_round();
    false
}


fn main() {
    let c1 = Character::new("Ragnar", Team::Player);
    let mut c2 = Character::new("Alena", Team::Player);
    c2.spd = 100;
    let s = Character::new("Slime", Team::Monster);
    let mut b = Battlefield {
        chars: vec![c1, c2, s],
        round: 1
    };
    let a1 = Action::Attack(0, 2);
    let a2 = Action::Defend(2);
    let a3 = Action::Attack(1, 2);
    let mut actions1 = vec![a1, a2, a3];
    loop {
        println!("{}", b);
        if run_turn(&mut b, &mut actions1) {
            println!("Victory!\n");
            println!("{}", b);
            break;
        }
    };
    //let mut actions2 = actions1.clone();

    //run_turn(&mut b, &mut actions1);
    //println!("{}", b);
    //run_turn(&mut b, &mut actions2);
    //println!("{}", b);
    //println!("Hello, world! {}", c);
    //c.hp -= 12;
    //println!("Bye world! {}", c);
}
