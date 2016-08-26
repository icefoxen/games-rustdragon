use std;
use std::fmt;


use super::character::*;

/// The central structure containing a battle's state.
#[derive(Debug, Clone)]
pub struct Battlefield {
    pub chars: Vec<Character>,
    pub round: u32,
}

/// A structure that specifies a specific character in a Battlefield.
pub type CharSpecifier = u32;

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
    pub fn new() -> Battlefield {
        Battlefield {
            chars: vec![],
            round: 1,
        }
    }
    pub fn increment_round(&mut self) {
        self.round += 1
    }

    pub fn get<'a>(&'a self, c: CharSpecifier) -> Option<&'a Character> {
        self.chars.get(c as usize)
    }

    pub fn get_mut<'a>(&'a mut self, c: CharSpecifier) -> Option<&'a mut Character> {
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
    pub fn players<'a>(&'a self) -> std::iter::Filter<std::slice::Iter<'a, Character>, fn(&&Character) -> bool> {
        self.get_team(Team::Player)
    }

    pub fn monsters<'a>(&'a self) -> std::iter::Filter<std::slice::Iter<'a, Character>, fn(&&Character) -> bool> {
        self.get_team(Team::Monster)
    }

    pub fn get_team<'a>(&'a self, team: Team) -> std::iter::Filter<std::slice::Iter<'a, Character>, fn(&&Character) -> bool> {
        // Booooo returning the results of filter() is dumb
        // 'cause you can't specify types of closures.
        // Though apparently there's a feature in nightly
        // as of August 2016 that allows you to specify a
        // trait return value rather than a specific type
        // And you can't use instance methods as if they
        // were class methods, either.

                
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

    pub fn get_opponents<'a>(&'a self, team: Team) -> std::iter::Filter<std::slice::Iter<'a, Character>, fn(&&Character) -> bool> {
        match team {
            Team::Player => self.get_team(Team::Monster),
            Team::Monster => self.get_team(Team::Player)
        }
    }

    pub fn team_victorious(&self, team: Team) -> bool {
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
