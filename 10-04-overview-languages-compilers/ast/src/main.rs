#[derive(Clone, Debug, PartialEq)]
enum Op {
    Add,
    #[allow(unused)]
    Sub,
    Mul,
    #[allow(unused)]
    Div,
}

#[derive(Clone, Debug, PartialEq)]
enum Expr {
    BinOp {
        left: Box<Expr>,
        op: Op,
        right: Box<Expr>,
    },
    Const(isize),
}

#[allow(unused)]
fn parse(code: &str) -> Expr {
    todo!()
}

fn eval(expr: Expr) -> isize {
    match expr {
        Expr::BinOp { left, op, right } => match op {
            Op::Add => eval(*left) + eval(*right),
            Op::Sub => eval(*left) - eval(*right),
            Op::Mul => eval(*left) * eval(*right),
            Op::Div => eval(*left) / eval(*right),
        },
        Expr::Const(v) => v,
    }
}

fn optimize(expr: Expr) -> Expr {
    match expr {
        Expr::BinOp {
            left,
            op,
            right,
        } => match (*left.clone(), *right.clone()) {
            (Expr::Const(l), Expr::Const(r)) => Expr::Const(eval(Expr::BinOp {
                left: Box::new(Expr::Const(l)),
                op,
                right: Box::new(Expr::Const(r)),
            })),
            _ => Expr::BinOp {
                left,
                op,
                right,
            },
        },
        Expr::Const(v) => Expr::Const(v),
    }
}

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
    for (expr, result) in test_cases {
        assert_eq!(eval(expr), result);
    }
}

fn test_optimize() {
    let test_cases = [
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
            Expr::Const(7),
        ),
    ];

    for (expr, result) in test_cases {
        assert_eq!(optimize(expr), result);
    }
}

fn main() {
    test_eval();
    test_optimize();
}
