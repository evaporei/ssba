#[derive(Clone, Debug, PartialEq)]
pub enum Op {
    Add,
    #[allow(unused)]
    Sub,
    Mul,
    #[allow(unused)]
    Div,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    BinOp {
        left: Box<Expr>,
        op: Op,
        right: Box<Expr>,
    },
    Const(isize),
    Var(String),
}

#[allow(unused)]
fn parse(code: &str) -> Expr {
    todo!()
}

pub fn eval(expr: Expr, ctx: &Context) -> isize {
    match expr {
        Expr::BinOp { left, op, right } => match op {
            Op::Add => eval(*left, ctx) + eval(*right, ctx),
            Op::Sub => eval(*left, ctx) - eval(*right, ctx),
            Op::Mul => eval(*left, ctx) * eval(*right, ctx),
            Op::Div => eval(*left, ctx) / eval(*right, ctx),
        },
        Expr::Const(v) => v,
        Expr::Var(name) => ctx[&name],
    }
}

pub fn optimize(expr: Expr) -> Expr {
    match expr {
        Expr::BinOp { left, op, right } => match (*left.clone(), *right.clone()) {
            (Expr::Const(l), Expr::Const(r)) => Expr::Const(eval(
                Expr::BinOp {
                    left: Box::new(Expr::Const(l)),
                    op,
                    right: Box::new(Expr::Const(r)),
                },
                &Context::new(),
            )),
            _ => Expr::BinOp { left, op, right },
        },
        Expr::Const(v) => Expr::Const(v),
        Expr::Var(_name) => Expr::Const(123),
    }
}

use std::collections::HashMap;

type Context = HashMap<String, isize>;

#[test]
fn test_eval() {
    let test_cases = [
        (
            Expr::BinOp {
                left: Box::new(Expr::Const(1)),
                op: Op::Add,
                right: Box::new(Expr::Const(1)),
            },
            2,
        ),
        (
            Expr::BinOp {
                left: Box::new(Expr::Const(1)),
                op: Op::Add,
                right: Box::new(Expr::BinOp {
                    left: Box::new(Expr::Const(2)),
                    op: Op::Mul,
                    right: Box::new(Expr::Const(3)),
                }),
            },
            7,
        ),
        (
            Expr::BinOp {
                left: Box::new(Expr::Const(1)),
                op: Op::Add,
                right: Box::new(Expr::BinOp {
                    left: Box::new(Expr::Const(2)),
                    op: Op::Mul,
                    right: Box::new(Expr::BinOp {
                        left: Box::new(Expr::Const(3)),
                        op: Op::Sub,
                        right: Box::new(Expr::Const(4)),
                    }),
                }),
            },
            -1,
        ),
        (
            Expr::BinOp {
                left: Box::new(Expr::BinOp {
                    left: Box::new(Expr::Const(1)),
                    op: Op::Add,
                    right: Box::new(Expr::BinOp {
                        left: Box::new(Expr::BinOp {
                            left: Box::new(Expr::Const(2)),
                            op: Op::Sub,
                            right: Box::new(Expr::Const(3)),
                        }),
                        op: Op::Mul,
                        right: Box::new(Expr::Const(4)),
                    }),
                }),
                op: Op::Add,
                right: Box::new(Expr::Const(5)),
            },
            2,
        ),
    ];
    let ctx = Context::new();

    for (expr, result) in test_cases {
        assert_eq!(eval(expr, &ctx), result);
    }
}

#[ignore]
#[test]
fn test_optimize() {
    let test_cases = [(
        Expr::BinOp {
            left: Box::new(Expr::Const(1)),
            op: Op::Add,
            right: Box::new(Expr::BinOp {
                left: Box::new(Expr::Const(2)),
                op: Op::Mul,
                right: Box::new(Expr::Const(3)),
            }),
        },
        Expr::Const(7),
    )];

    for (expr, result) in test_cases {
        assert_eq!(optimize(expr), result);
    }
}
