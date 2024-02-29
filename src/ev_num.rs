use crate::Error;
use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EvNum(i32);

impl EvNum {
    const MAX: i32 = 11;

    #[must_use]
    pub fn new(num: i32) -> Self {
        Self(num.rem_euclid(Self::MAX))
    }

    #[must_use]
    pub fn inner(self) -> i32 {
        self.0
    }
}

impl Add for EvNum {
    type Output = EvNum;

    fn add(self, rhs: Self) -> Self::Output {
        EvNum::new(self.0 + rhs.0)
    }
}

impl Sub for EvNum {
    type Output = EvNum;

    fn sub(self, rhs: Self) -> Self::Output {
        EvNum::new(self.0 - rhs.0)
    }
}

impl Mul for EvNum {
    type Output = EvNum;

    fn mul(self, rhs: Self) -> Self::Output {
        EvNum::new(self.0 * rhs.0)
    }
}

impl Div for EvNum {
    type Output = Result<EvNum, Error>;

    fn div(self, rhs: Self) -> Self::Output {
        if rhs.0 == 0 {
            return Err(Error::NumDivNoSolution(self, rhs));
        }

        for i in 0..Self::MAX {
            let i = EvNum(i);
            if i * rhs == self {
                return Ok(i);
            }
        }

        Err(Error::NumDivNoSolution(self, rhs))
    }
}

impl Neg for EvNum {
    type Output = EvNum;

    fn neg(self) -> Self::Output {
        EvNum(0) - self
    }
}

impl Display for EvNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 == 10 {
            write!(f, "X")
        } else {
            write!(f, "{}", self.0)
        }
    }
}

impl TryFrom<char> for EvNum {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '0'..='9' => Ok(Self(value as i32 - i32::from(b'0'))),
            'X' => Ok(Self(10)),
            _ => Err(Error::UnknownDigit(value)),
        }
    }
}

impl FromStr for EvNum {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 1 {
            return Err(Error::NumStringTooLong(s.to_string()));
        }
        let Some(c) = s.chars().next() else {
            return Err(Error::NumStringEmpty);
        };
        EvNum::try_from(c)
    }
}
