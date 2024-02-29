use crate::{Error, EvNum, EvVec};
use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    EValue(EValue),
    EAdd(Box<Expr>, Box<Expr>),
    ESub(Box<Expr>, Box<Expr>),
    EMul(Box<Expr>, Box<Expr>),
    EDiv(Box<Expr>, Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum EValue {
    ENum(EvNum),
    EVec(EvVec),
}

impl Add for EValue {
    type Output = Result<EValue, Error>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (EValue::ENum(n1), EValue::ENum(n2)) => Ok(EValue::ENum(n1 + n2)),
            (EValue::EVec(v1), EValue::EVec(v2)) => Ok(EValue::EVec(v1 + v2)),
            (e1, e2) => Err(Error::IncompatibleAdd(e1, e2)),
        }
    }
}

impl Sub for EValue {
    type Output = Result<EValue, Error>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (EValue::ENum(n1), EValue::ENum(n2)) => Ok(EValue::ENum(n1 - n2)),
            (EValue::EVec(v1), EValue::EVec(v2)) => Ok(EValue::EVec(v1 - v2)),
            (e1, e2) => Err(Error::IncompatibleSub(e1, e2)),
        }
    }
}

impl Mul for EValue {
    type Output = Result<EValue, Error>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (EValue::ENum(n1), EValue::ENum(n2)) => Ok(EValue::ENum(n1 * n2)),
            (EValue::EVec(v1), EValue::EVec(v2)) => Ok(EValue::EVec(v1 * v2)),
            (e1, e2) => Err(Error::IncompatibleMul(e1, e2)),
        }
    }
}

impl Div for EValue {
    type Output = Result<EValue, Error>;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (EValue::ENum(n1), EValue::ENum(n2)) => Ok(EValue::ENum((n1 / n2)?)),
            (EValue::EVec(v1), EValue::EVec(v2)) => Ok(EValue::EVec((v1 / v2)?)),
            (e1, e2) => Err(Error::IncompatibleDiv(e1, e2)),
        }
    }
}

impl Display for EValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EValue::ENum(n) => write!(f, "{n}"),
            EValue::EVec(v) => write!(f, "{v}"),
        }
    }
}
