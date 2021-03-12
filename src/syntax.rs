use std::fmt::Debug;
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Fml {
    Prim {
        comp: Comp,
        l: Expr,
        r: Expr,
    },
    Quant {
        quant: Quant,
        var: String,
        inner: Box<Fml>,
    },
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Quant {
    Exists,
    Forall,
    ExistsN(u32),
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Comp {
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
}

impl Debug for Comp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            Comp::Eq => "=",
            Comp::Ne => "/=",
            Comp::Lt => "<",
            Comp::Gt => ">",
            Comp::Le => "<=",
            Comp::Ge => ">=",
        };
        write!(f, "{}", s)
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Expr {
    Num(i64),
    Var(String),
    Op { op: Op, l: Box<Expr>, r: Box<Expr> },
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Pow,
}
