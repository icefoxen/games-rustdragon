use std::io;

extern crate rustdragon;

use rustdragon::character::*;
use rustdragon::battlefield::*;
use rustdragon::action::*;



enum BattleStatus {
    PlayerVictory,
    MonsterVictory,
    Continuing,
    // PlayersFled,
    // MonstersFled
}

fn parse_action(c: &Character, i: CharSpecifier, s: &str) -> Action {
    Action::Defend(0)
}

fn read_player_actions(field: &Battlefield, actions: &mut Vec<Action>) {
    let living_players = field.get_team_enumerate(Team::Player)
        .filter(|&(_, chr)| chr.is_alive());

    for (i,c) in living_players {
        let mut input = String::new();
        println!("Input action for {}", c.name);
        io::stdin().read_line(&mut input);
        println!("Read: '{}'", input);
        parse_action(c, i, input.as_str());
    }
}

fn decide_monster_actions(field: &Battlefield, actions: &mut Vec<Action>) {
    let living_monsters = field.get_team_enumerate(Team::Monster)
        .filter(|&(_, chr)| chr.is_alive());

    for (i,c) in living_monsters {
        let action = Action::Defend(i);
        actions.push(action);
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

    let mut actions = Vec::new();
    loop {
        println!("");
        println!("{}", field);
        actions.clear();
        read_player_actions(&field, &mut actions);
        decide_monster_actions(&field, &mut actions);
        match run_turn(&mut field, &mut actions) {
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
