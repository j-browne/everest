use crate::{
    ev_num::EvNum,
    expr::{EValue, Expr},
    EvVec,
};
use nom::{
    branch::alt,
    character::complete::{char, one_of, space0},
    combinator::map,
    multi::many0,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

pub fn parse(input: &str) -> IResult<&str, Expr> {
    parse_basic_expr(input)
}

fn parse_basic_expr(input: &str) -> IResult<&str, Expr> {
    parse_math_expr(input)
}

fn parse_parens(input: &str) -> IResult<&str, Expr> {
    delimited(
        space0,
        delimited(char('('), parse_math_expr, char(')')),
        space0,
    )(input)
}

fn parse_operation(input: &str) -> IResult<&str, Expr> {
    //alt((parse_parens, parse_number))(input)
    alt((parse_parens, parse_value))(input)
}

fn parse_factor(input: &str) -> IResult<&str, Expr> {
    let (input, num1) = parse_operation(input)?;
    let (input, exprs) = many0(tuple((char('^'), parse_factor)))(input)?;
    Ok((input, parse_expr(num1, exprs)))
}

fn parse_term(input: &str) -> IResult<&str, Expr> {
    let (input, num1) = parse_factor(input)?;
    let (input, exprs) = many0(tuple((alt((char('/'), char('*'))), parse_factor)))(input)?;
    Ok((input, parse_expr(num1, exprs)))
}

fn parse_math_expr(input: &str) -> IResult<&str, Expr> {
    let (input, num1) = parse_term(input)?;
    let (input, exprs) = many0(tuple((alt((char('+'), char('-'))), parse_term)))(input)?;
    Ok((input, parse_expr(num1, exprs)))
}

fn parse_expr(expr: Expr, rem: Vec<(char, Expr)>) -> Expr {
    rem.into_iter().fold(expr, |acc, val| parse_op(val, acc))
}

fn parse_op(tup: (char, Expr), expr1: Expr) -> Expr {
    let (op, expr2) = tup;
    match op {
        '+' => Expr::EAdd(Box::new(expr1), Box::new(expr2)),
        '-' => Expr::ESub(Box::new(expr1), Box::new(expr2)),
        '*' => Expr::EMul(Box::new(expr1), Box::new(expr2)),
        '/' => Expr::EDiv(Box::new(expr1), Box::new(expr2)),
        _ => panic!("Unknown Operation"),
    }
}

fn parse_num(input: &str) -> IResult<&str, EvNum> {
    map(one_of("0123456789X"), |c| {
        EvNum::try_from(c).expect("only valid input was supplied")
    })(input)
}

fn parse_vec(input: &str) -> IResult<&str, EvVec> {
    map(
        separated_pair(parse_num, char('|'), parse_num),
        |(n1, n2)| EvVec::new(n1, n2),
    )(input)
}

fn parse_value(input: &str) -> IResult<&str, Expr> {
    delimited(
        space0,
        alt((
            map(parse_vec, |v| Expr::EValue(EValue::EVec(v))),
            map(parse_num, |n| Expr::EValue(EValue::ENum(n))),
        )),
        space0,
    )(input)
}
