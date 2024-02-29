use crate::{Error, EvNum};
use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EvVec(EvNum, EvNum);

impl EvVec {
    #[must_use]
    pub fn new(num1: EvNum, num2: EvNum) -> Self {
        Self(num1, num2)
    }
}

impl Add for EvVec {
    type Output = EvVec;

    fn add(self, rhs: Self) -> Self::Output {
        EvVec(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for EvVec {
    type Output = EvVec;

    fn sub(self, rhs: Self) -> Self::Output {
        EvVec(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Mul for EvVec {
    type Output = EvVec;

    fn mul(self, rhs: Self) -> Self::Output {
        EvVec(
            self.0 * rhs.0 - self.1 * rhs.1,
            self.0 * rhs.1 + self.1 * rhs.0,
        )
    }
}

impl Div for EvVec {
    type Output = Result<EvVec, Error>;

    #[allow(clippy::many_single_char_names)]
    fn div(self, rhs: Self) -> Self::Output {
        let EvVec(a, b) = self;
        let zero = EvNum::new(0);
        match rhs {
            EvVec(c, d) if c == zero && d == zero => Err(Error::VecDivNoSolution(self, rhs)),
            EvVec(c, d) if c == zero => {
                let x = b / d;
                let y = -a / d;
                Ok(EvVec(x?, y?))
            }
            EvVec(c, d) if d == zero => {
                let x = a / c;
                let y = b / c;
                Ok(EvVec(x?, y?))
            }
            EvVec(c, d) => {
                let x = (a * c + b * d) / (c * c + d * d);
                let y = (b - d * x.clone()?) / c;
                Ok(EvVec(x?, y?))
            }
        }
    }
}

impl Neg for EvVec {
    type Output = EvVec;

    fn neg(self) -> Self::Output {
        EvVec(EvNum::new(0), EvNum::new(0)) - self
    }
}

impl Display for EvVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}|{}", self.0, self.1)
    }
}

impl FromStr for EvVec {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s
            .split_once('|')
            .ok_or(Error::VecStringNoPipe(s.to_string()))?;
        Ok(Self(EvNum::from_str(first)?, EvNum::from_str(second)?))
    }
}
