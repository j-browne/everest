use crate::{expr::EValue, EvNum, EvVec};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Error {
    UnknownDigit(char),
    NumStringEmpty,
    NumStringTooLong(String),
    VecStringNoPipe(String),
    NumDivNoSolution(EvNum, EvNum),
    VecDivNoSolution(EvVec, EvVec),
    IncompatibleAdd(EValue, EValue),
    IncompatibleSub(EValue, EValue),
    IncompatibleMul(EValue, EValue),
    IncompatibleDiv(EValue, EValue),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownDigit(c) => write!(f, "an unknown digit was encountered: {c:?}"),
            Self::NumStringEmpty => write!(f, "tried to parse an empty string as a number"),
            Self::NumStringTooLong(s) => write!(f, "number longer than one digit: {s:?}"),
            Self::VecStringNoPipe(s) => write!(f, "vec without a separator: {s:?}"),
            Self::NumDivNoSolution(n1, n2) => write!(f, "cannot divide {n1} by {n2}"),
            Self::VecDivNoSolution(v1, v2) => write!(f, "cannot divide {v1} by {v2}"),
            Self::IncompatibleAdd(v1, v2) => write!(f, "cannot add {v1} and {v2}"),
            Self::IncompatibleSub(v1, v2) => write!(f, "cannot subtract {v2} from {v1}"),
            Self::IncompatibleMul(v1, v2) => write!(f, "cannot multiply {v1} and {v2}"),
            Self::IncompatibleDiv(v1, v2) => write!(f, "cannot divide {v1} by {v2}"),
        }
    }
}

impl std::error::Error for Error {}
