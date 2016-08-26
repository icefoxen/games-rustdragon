use std::cmp;
use std::fmt;

extern crate rand;
//use rand::random;

extern crate rustdragon;

use rustdragon::character::*;

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

    fn get_opponents<'a>(&'a self, team: Team) -> std::iter::Filter<std::slice::Iter<'a, Character>, fn(&&Character) -> bool> {
        match team {
            Team::Player => self.get_team(Team::Monster),
            Team::Monster => self.get_team(Team::Player)
        }
    }

    fn team_victorious(&self, team: Team) -> bool {
        self.get_opponents(team).filter(|x| x.is_alive()).count() == 0
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
/// and returns a CharSpecifier referring to it.
fn choose_new_target_if_target_is_dead(field: &mut Battlefield, from: CharSpecifier, to: CharSpecifier) -> CharSpecifier {
    let fromteam = field.get(from).unwrap().team;
    let tochar_is_alive = field.get(to).unwrap().is_alive();
    if !tochar_is_alive {
        // Now we need to get opponents and select one at random.
        // We check if the battle is over before every action, so
        // there should always be at least *one* opponent to choose from.

        // After writing all these nice iterators we can't use them 'cause
        // we need to get the character *index*, so we need to stick an
        // enumerate() in there.
        // Sigh.
        let living_enemies = field.chars.iter()
            .enumerate()
            .filter(|&(_, chr)| chr.team != fromteam)
            .filter(|&(_, chr)| chr.is_alive());

        let mut rng = rand::thread_rng();
        let sample = rand::sample(&mut rng, living_enemies, 1);
        // We always check for victory before each action, so,
        // there should always be at least opponent available to
        // choose from.
        let (i, _) = sample[0];
        i as CharSpecifier
    } else {
        to
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
    
    let defender_idx = choose_new_target_if_target_is_dead(field, from, to);
    let defender = field.get_mut(defender_idx).unwrap();
    let soak = rand::random::<u32>() % defender.def;

    print!("{} attacked {}!  ", attacker_name, defender.name);
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

enum BattleStatus {
    PlayerVictory,
    MonsterVictory,
    Continuing,
    // PlayersFled,
    // MonstersFled
}

/// Runs a single turn in the battle.
/// It takes a battlefield state, and a list of actions
/// and applies the actions in the proper order.
/// It returns a battle status.
fn run_turn(field: &mut Battlefield, actions: &mut Vec<Action>) -> BattleStatus {
    // We're going to want a sort-actions step, where we order the actions
    // by priority and character speed and such (defend's always take effect first, etc)
    // and THEN execute them.
    order_actions(field, actions);
    for action in actions {
        // If the battle is over, we stop where we are!
        // Partially 'cause any remaining actions will be invalid.
        if field.team_victorious(Team::Player) {
            return BattleStatus::PlayerVictory;
        }
        else if field.team_victorious(Team::Monster) {
            return BattleStatus::MonsterVictory;
        }

        run_action(field, &action);
    }

    // Check again, juuuuust in case that last action finished
    // something off.
    if field.team_victorious(Team::Player) {
        return BattleStatus::PlayerVictory;
    }
    else if field.team_victorious(Team::Monster) {
        return BattleStatus::MonsterVictory;
    }

    
    field.increment_round();
    BattleStatus::Continuing
}

fn mainloop(mut field: Battlefield) {
    let a1 = Action::Attack(0, 2);
    let a2 = Action::Defend(2);
    let a3 = Action::Attack(1, 2);
    let mut actions1 = vec![a1, a2, a3];

    loop {
        println!("");
        println!("{}", field);
        match run_turn(&mut field, &mut actions1) {
            BattleStatus::PlayerVictory => {
                println!("Victory!\n");
                break;
            },
            BattleStatus::MonsterVictory => {
                println!("Horrible, crushing defeat!\n");
                break;
            }
            _ => ()
        }
    };
}


fn main() {
    let c1 = Character::new("Ragnar", Team::Player);
    let mut c2 = Character::new("Alena", Team::Player);
    c2.spd = 100;
    let m1 = Character::new("Slime", Team::Monster);
    let m2 = Character::new("Bat", Team::Monster);
    let mut b = Battlefield::new();
    b.chars = vec![c1, c2, m1, m2];
    mainloop(b);
}
