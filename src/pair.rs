use crate::ratio::Ratio;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pair(pub u64, pub u64);

impl Pair {
    pub fn x(&self) -> u64 {
        self.0
    }

    pub fn y(&self) -> u64 {
        self.1
    }
}

impl From<Ratio> for Pair {
    fn from(r: Ratio) -> Self {
        r.as_pair()
    }
}

impl From<Pair> for f64 {
    fn from(p: Pair) -> Self {
        p.0 as f64 / p.1 as f64
    }
}

impl fmt::Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}x{}", self.0, self.1)
    }
}
