#[derive(Debug, PartialEq)]
enum Op {
    Add,
    #[allow(unused)]
    Sub,
    Mul,
    #[allow(unused)]
    Div,
}

#[derive(Debug, PartialEq)]
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

fn main() {
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
    ];
    // assert_eq!(
    //     parse("1 + 1"),
    //     expr,
    // );
    for (expr, result) in test_cases {
        assert_eq!(eval(expr), result);
    }
}
