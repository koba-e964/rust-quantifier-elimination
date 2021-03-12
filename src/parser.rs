use crate::syntax::{Expr, Op};
use lazy_static::lazy_static;
use pest::error::Error as PestError;
use pest::iterators::{Pair, Pairs};
use pest::prec_climber::{Assoc, Operator, PrecClimber};
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "formula.pest"]
pub enum FormulaParser {}

lazy_static! {
    static ref PREC_CLIMBER: PrecClimber<Rule> = {
        use Assoc::*;
        use Rule::*;

        PrecClimber::new(vec![
            Operator::new(add, Left) | Operator::new(subtract, Left),
            Operator::new(multiply, Left),
            Operator::new(power, Right),
        ])
    };
}

pub fn eval(expression: Pairs<Rule>) -> Expr {
    PREC_CLIMBER.climb(
        expression,
        |pair: Pair<Rule>| match pair.as_rule() {
            Rule::num => {
                let val = pair.as_str().parse::<i64>().unwrap();
                Expr::Num(val)
            }
            Rule::expr => eval(pair.into_inner()),
            Rule::var => Expr::Var,
            _ => unreachable!(),
        },
        |lhs: Expr, op: Pair<Rule>, rhs: Expr| {
            let op = match op.as_rule() {
                Rule::add => Op::Add,
                Rule::subtract => Op::Sub,
                Rule::multiply => Op::Mul,
                Rule::power => Op::Pow,
                _ => unreachable!(),
            };
            Expr::Op {
                op,
                l: Box::new(lhs),
                r: Box::new(rhs),
            }
        },
    )
}

pub fn parse(s: &str) -> Result<Expr, PestError<Rule>> {
    let parsed = FormulaParser::parse(Rule::expr, &s)?;
    Ok(eval(parsed))
}
