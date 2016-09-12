

use super::character::*;
use super::battlefield::*;

use rand;


// lazy_static is awesome.
// It basically defines a "static variable" that can contain function calls,
// which is initialized at runtime upon the first time it is dereferenced.
// Will probably be made obsolete by const fn, but currently that's still in
// nightly.
lazy_static! {
    static ref CHARS: Vec<Character> = {
        let mut c = Vec::new();
        c.push(Character::new("Ragnar", Team::Player));
        c.push(Character::new("Alena", Team::Player));

        c.push(Character::new("Slime", Team::Monster));
        c.push(Character::new("Bat", Team::Monster));
        c
    };

    static ref PLAYERS: Vec<Character> = {
        let mut c = Vec::new();
        c.push(Character::new("Ragnar", Team::Player));
        c.push(Character::new("Alena", Team::Player));
        c.push(Character::new("Cristo", Team::Player));
        c.push(Character::new("Brey", Team::Player));
        c.push(Character::new("Taloon", Team::Player));
        c.push(Character::new("Mara", Team::Player));
        c.push(Character::new("Nara", Team::Player));
        c.push(Character::new("Orin", Team::Player));
        c.push(Character::new("Katta", Team::Player));

        c.push(Character::new("Papas", Team::Player));
        c.push(Character::new("Bianca", Team::Player));
        c.push(Character::new("Flora", Team::Player));
        c.push(Character::new("Tabitha", Team::Player));
        c.push(Character::new("Rex", Team::Player));
        c.push(Character::new("Sancho", Team::Player));
        c.push(Character::new("Rusty", Team::Player));
        
        c.push(Character::new("Hassan", Team::Player));
        c.push(Character::new("Muriel", Team::Player));
        c.push(Character::new("Barbara", Team::Player));
        c.push(Character::new("Chamoro", Team::Player));
        c.push(Character::new("Amos", Team::Player));
        c.push(Character::new("Terry", Team::Player));
        c
    };

    static ref MOBS: Vec<Character> = {
        let mut c = Vec::new();
        c.push(Character::new("Slime", Team::Monster));
        c.push(Character::new("Slime Knight", Team::Monster));
        c.push(Character::new("King Slime", Team::Monster));
        c.push(Character::new("Magician", Team::Monster));
        c.push(Character::new("Healer", Team::Monster));
        c.push(Character::new("Babble", Team::Monster));
        c.push(Character::new("Army Crab", Team::Monster));
        c.push(Character::new("Gas Cloud", Team::Monster));
        c.push(Character::new("Demon Toadstool", Team::Monster));
        c.push(Character::new("Rogue Knight", Team::Monster));
        c.push(Character::new("Mimic", Team::Monster));
        c.push(Character::new("Bomb Crag", Team::Monster));
        c.push(Character::new("Wyvern", Team::Monster));
        c.push(Character::new("Armor Scorpion", Team::Monster));
        c.push(Character::new("Blazeghost", Team::Monster));
        c.push(Character::new("Metal Slime", Team::Monster));
        c.push(Character::new("Baby Panther", Team::Monster));
        c.push(Character::new("Clay Doll", Team::Monster));
        c.push(Character::new("Cactus Ball", Team::Monster));
        c.push(Character::new("Drakee", Team::Monster));
        c.push(Character::new("Minidemon", Team::Monster));
        c.push(Character::new("Hork", Team::Monster));
        c.push(Character::new("Demon Pot", Team::Monster));
        c.push(Character::new("Dancing Jewel", Team::Monster));
        c.push(Character::new("Lipps", Team::Monster));
        c.push(Character::new("Onion", Team::Monster));
        

        // They should drop small medals after dying :-3
        c.push(Character::new("Rust Dragon", Team::Monster));
        c.push(Character::new("Bandersnatch", Team::Monster));
        c
    };
}
    
//const characters: [Character;1] = [
//    Character::new("Ragnar", Team::Player)
//];

fn select_players() -> Vec<&'static Character> {
    let mut rng = rand::thread_rng();
    let sample = rand::sample(&mut rng, (*PLAYERS).iter(), 4);
    sample.clone()
}

fn select_monsters() -> Vec<&'static Character> {
    let mut rng = rand::thread_rng();
    let sample = rand::sample(&mut rng, (*MOBS).iter(), 3);
    sample.clone()

}

pub fn generate() -> Battlefield {
    let mut b = Battlefield::new();
    let players_refs = select_players();
    let players: Vec<_> = players_refs.iter().map(|p|
        (*p).clone()).collect();
    let mut monsters = select_monsters();
    //players.extend(monsters);
    b.chars = players;
    b
}
