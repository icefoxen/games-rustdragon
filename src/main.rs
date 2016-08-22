use std::cmp;
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



#[derive(Debug)]
struct Character {
    hp: BoundedNumber,
    mp: BoundedNumber,
}


impl Character {
    fn new() -> Character {
        Character {
            hp: BoundedNumber::new(10),
            mp: BoundedNumber::new(10),
        }
    }
}

fn main() {
    let mut c = Character::new();
    println!("Hello, world! {:?}", c);
    c.hp -= 12;
    println!("Bye world! {:?}", c);
}
