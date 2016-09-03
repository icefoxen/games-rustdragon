//use std;
use std::cmp;
use super::character::*;
use super::battlefield::*;

extern crate rand;
//use rand::random;


// The Character direct references here are bad and wrong,
// because what happens if a character dies before an attack goes off?
// They need to be some sort of indirect reference so we can check
// whether or not it's valid.
#[derive(Debug, Clone, Copy)]
pub enum Action {
    Attack(CharSpecifier, CharSpecifier),
    Defend(CharSpecifier),
}

impl Action {
    /// Defines the priority of actions, the order
    /// in which they will be taken.
    /// Higher priority goes first.
    pub fn priority(&self) -> i32 {
        match *self {
            Action::Attack(..) => 0,
            Action::Defend(_) => 10,
        }
    }

    pub fn source(&self) -> CharSpecifier {
        match *self {
            Action::Attack(from, _) => from,
            Action::Defend(who) => who,
        }
    }

}


/// Does exactly what it says on the tin.
/// If the 'to' character specified is not alive,
/// choose another target at random (that isn't on the same team)
/// and returns a CharSpecifier referring to it.
pub fn choose_new_target_if_target_is_dead(field: &mut Battlefield, from: CharSpecifier, to: CharSpecifier) -> CharSpecifier {
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

pub fn do_attack(field: &mut Battlefield, from: CharSpecifier, to: CharSpecifier) {
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

pub fn do_defend(field: &mut Battlefield, who: CharSpecifier) {
    // TODO: Better error handling here.
    let mut whochar = field.get_mut(who).unwrap();
    println!("{} defended themselves!", whochar.name);
    let defbuff = Buff {
        turns_left: 3,
        effect: BuffEffect::StatUp(10)
    };
    whochar.buffs.push(defbuff)
}




pub fn run_action(field: &mut Battlefield, action: &Action) {
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
pub fn order_actions(field: &Battlefield, actions: &mut Vec<Action>) {
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
