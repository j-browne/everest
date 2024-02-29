use crate::{
    expr::{EValue, Expr},
    Error,
};

pub fn evaluate(expr: Expr) -> Result<EValue, Error> {
    match expr {
        Expr::EValue(val) => Ok(val),
        Expr::EAdd(expr1, expr2) => evaluate(*expr1)? + evaluate(*expr2)?,
        Expr::ESub(expr1, expr2) => evaluate(*expr1)? - evaluate(*expr2)?,
        Expr::EMul(expr1, expr2) => evaluate(*expr1)? * evaluate(*expr2)?,
        Expr::EDiv(expr1, expr2) => evaluate(*expr1)? / evaluate(*expr2)?,
    }
}
