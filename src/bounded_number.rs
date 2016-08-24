use std::cmp;
use std::fmt;
use std::ops::{Add, Sub, AddAssign, SubAssign};


/// Represents a u32 that is fixed to be between 0 and some max value.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BoundedNumber {
    pub val: u32,
    pub max: u32,
}

impl BoundedNumber {
    pub fn new(max:u32) -> BoundedNumber {
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


#[cfg(test)]
mod tests {
    use super::BoundedNumber;
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

}
