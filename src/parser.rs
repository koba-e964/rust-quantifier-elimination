use crate::syntax::{Comp, Expr, Fml, Op, Quant};
use lazy_static::lazy_static;
use pest::error::Error as PestError;
use pest::iterators::{Pair, Pairs};
use pest::prec_climber::{Assoc, Operator, PrecClimber};
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "formula.pest"]
enum FormulaParser {}

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

fn op(op: Rule) -> Op {
    match op {
        Rule::add => Op::Add,
        Rule::subtract => Op::Sub,
        Rule::multiply => Op::Mul,
        Rule::power => Op::Pow,
        _ => unreachable!(),
    }
}

fn expr(expression: Pairs<Rule>) -> Expr {
    PREC_CLIMBER.climb(
        expression,
        |pair: Pair<Rule>| match pair.as_rule() {
            Rule::num => {
                let val = pair.as_str().parse::<i64>().unwrap();
                Expr::Num(val)
            }
            Rule::expr => expr(pair.into_inner()),
            Rule::var => Expr::Var(pair.as_str().to_owned()),
            _ => unreachable!(),
        },
        |lhs: Expr, op_: Pair<Rule>, rhs: Expr| {
            let op = op(op_.as_rule());
            Expr::Op {
                op,
                l: Box::new(lhs),
                r: Box::new(rhs),
            }
        },
    )
}

fn comp(comp: Rule) -> Comp {
    match comp {
        Rule::eq => Comp::Eq,
        Rule::ne => Comp::Ne,
        Rule::lt => Comp::Lt,
        Rule::gt => Comp::Gt,
        Rule::le => Comp::Le,
        Rule::ge => Comp::Ge,
        _ => unreachable!(),
    }
}

fn quant(quant: Pair<Rule>) -> (Quant, String) {
    let inner: Vec<Pair<Rule>> = quant.into_inner().collect();
    let quant = match inner[0].as_rule() {
        Rule::exists => Quant::Exists,
        Rule::forall => Quant::Forall,
        _ => unreachable!(),
    };
    let var = inner[1].as_str().to_owned();
    (quant, var)
}

fn fml(formula: Pair<Rule>) -> Fml {
    let vec: Vec<Pair<Rule>> = formula.into_inner().collect();
    let inner: Vec<Pair<Rule>> = vec[0].clone().into_inner().collect();
    // prim_fml
    if vec.len() == 1 {
        assert_eq!(inner.len(), 3);
        let comp = comp(inner[1].as_rule());
        let l = inner[0].clone().into_inner();
        let r = inner[2].clone().into_inner();
        return Fml::Prim {
            comp,
            l: expr(l),
            r: expr(r),
        };
    }
    // quant_var fml
    assert_eq!(vec.len(), 2);
    let (quant, var) = quant(vec[0].clone());
    Fml::Quant {
        quant,
        var,
        inner: Box::new(fml(vec[1].clone())),
    }
}

pub fn parse(s: &str) -> Result<Fml, PestError<Rule>> {
    let parsed = FormulaParser::parse(Rule::main, &s)?.next().unwrap();
    Ok(fml(parsed))
}

#[cfg(test)]
mod tests {
    use super::*;

    // 123 + 456 + x > y^2
    fn fml1() -> Fml {
        Fml::Prim {
            comp: Comp::Gt,
            l: Expr::Op {
                op: Op::Add,
                l: Box::new(Expr::Op {
                    op: Op::Add,
                    l: Box::new(Expr::Num(123)),
                    r: Box::new(Expr::Num(456)),
                }),
                r: Box::new(Expr::Var("x".to_owned())),
            },
            r: Expr::Op {
                op: Op::Pow,
                l: Box::new(Expr::Var("y".to_owned())),
                r: Box::new(Expr::Num(2)),
            },
        }
    }

    #[test]
    fn test_parse_fml() {
        let actual_fml = parse("123 + 456 + x > y^2").unwrap();
        let expected_fml = fml1();
        assert_eq!(actual_fml, expected_fml);
    }
    #[test]
    fn test_parse_fml_with_quants() {
        let actual_fml = parse("(A y) 123 + 456 + x > y^2").unwrap();
        let expected_fml = Fml::Quant {
            quant: Quant::Forall,
            var: "y".to_owned(),
            inner: Box::new(fml1()),
        };
        assert_eq!(actual_fml, expected_fml);
    }
}
