use crate::pair::Pair;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Ratio {
    this: Pair,
    orig: Pair,
}

impl Ratio {
    pub fn new(x: u64, y: u64) -> Self {
        let gcd = gcd(x, y);
        let rx = x / gcd;
        let ry = y / gcd;

        Self {
            this: Pair(rx, ry),
            orig: Pair(x, y),
        }
    }

    pub fn as_pair(&self) -> Pair {
        self.this
    }

    pub fn original(&self) -> Pair {
        self.orig
    }
}

fn gcd(x: u64, y: u64) -> u64 {
    let mut gcd = *(vec![x, y].iter().min().unwrap_or(&1));

    while gcd > 1 {
        if x % gcd == 0 && y % gcd == 0 {
            break;
        }
        gcd -= 1;
    }

    gcd
}

impl From<Pair> for Ratio {
    fn from(pair: Pair) -> Self {
        Self::new(pair.x(), pair.y())
    }
}

impl From<Ratio> for f64 {
    fn from(r: Ratio) -> Self {
        r.this.into()
    }
}

impl fmt::Display for Ratio {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.this.0, self.this.1)
    }
}
