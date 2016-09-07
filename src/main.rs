use std::io;

extern crate rand;
extern crate rustdragon;

use rustdragon::character::*;
use rustdragon::battlefield::*;
use rustdragon::action::*;


enum BattleStatus {
    PlayerVictory,
    MonsterVictory,
    Continuing, /* PlayersFled,
                 * MonstersFled */
}

fn print_possible_actions() {
    println!(" 1) Attack");
    println!(" 2) Defend");
}

fn read_attack(field: &Battlefield, i: CharSpecifier) -> Action {
    // Print out the targets to attack
    println!("Attack what?");
    let living_monsters = field.get_team_enumerate(Team::Monster)
        .filter(|&(_, chr)| chr.is_alive())
        .collect::<Vec<(CharSpecifier, &Character)>>();
    let mut j = 0;

    for &(_, m) in living_monsters.iter() {
        j += 1;
        println!(" {}) {}", j, m.name);
    }

    // Read a numerical string and match it to a target
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    match input.trim().parse::<usize>() {
        Ok(target) if target > 0 && target <= living_monsters.len() => {
            let (idx, _) = living_monsters[target - 1];
            Action::Attack(i, idx)
        }
        _ => {
            println!("Please enter a valid option!");
            read_attack(field, i)
        }
    }
}

fn read_player_action(field: &Battlefield, i: CharSpecifier) -> Action {
    print_possible_actions();
    // This UI really needs a state machine.
    // We'll just implement it the simple and dumb way.
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);

    match input.trim().parse::<usize>() {
        Ok(1) => read_attack(field, i),
        Ok(2) => Action::Defend(i),
        _res => {
            println!("Please enter a valid option.");
            read_player_action(field, i)
        }
    }
}

fn read_player_actions(field: &Battlefield, actions: &mut Vec<Action>) {
    let living_players = field.get_team_enumerate(Team::Player)
        .filter(|&(_, chr)| chr.is_alive());

    for (i, c) in living_players {
        println!("Input action for {}", c.name);
        let action = read_player_action(field, i);
        actions.push(action);
    }
}

fn decide_monster_actions(field: &Battlefield, actions: &mut Vec<Action>) {
    let living_monsters = field.get_team_enumerate(Team::Monster)
        .filter(|&(_, chr)| chr.is_alive());

    for (i, _) in living_monsters {
        let living_players = field.get_team_enumerate(Team::Player)
            .filter(|&(_, chr)| chr.is_alive());
        let mut rng = rand::thread_rng();
        let sample = rand::sample(&mut rng, living_players, 1);
        // We always check for victory before each action, so,
        // there should always be at least opponent available to
        // choose from.
        let (targetidx, _) = sample[0];
        let action = Action::Attack(i, targetidx);
        actions.push(action);
    }
}


fn tick_buffs(field: &mut Battlefield) {
    for c in field.chars.iter_mut() {
        c.tick_buffs()
    }
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
        } else if field.team_victorious(Team::Monster) {
            return BattleStatus::MonsterVictory;
        }

        run_action(field, &action);
    }

    // Check again, juuuuust in case that last action finished
    // something off.
    if field.team_victorious(Team::Player) {
        return BattleStatus::PlayerVictory;
    } else if field.team_victorious(Team::Monster) {
        return BattleStatus::MonsterVictory;
    }

    field.increment_round();
    BattleStatus::Continuing
}

fn mainloop(mut field: Battlefield) {
    let mut actions = Vec::new();
    loop {
        // This has to happen before printing out the field,
        // since it happens at the beginning of the turn and we
        // don't want to print out-of-date info.
        tick_buffs(&mut field);

        println!("");
        println!("{}", field);

        actions.clear();
        read_player_actions(&field, &mut actions);
        decide_monster_actions(&field, &mut actions);
        match run_turn(&mut field, &mut actions) {
            BattleStatus::PlayerVictory => {
                println!("Victory!\n");
                break;
            }
            BattleStatus::MonsterVictory => {
                println!("Horrible, crushing defeat!\n");
                break;
            }
            _ => (),
        }
    }
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
