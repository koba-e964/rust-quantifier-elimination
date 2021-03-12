#[derive(Debug)]
pub enum Expr {
    Num(i64),
    Var,
    Op { op: Op, l: Box<Expr>, r: Box<Expr> },
}

#[derive(Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Pow,
}
