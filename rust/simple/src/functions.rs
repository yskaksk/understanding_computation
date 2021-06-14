use crate::expression::Expression;
use crate::expression::Expression::{Add, LessThan, Number, Variable};
use crate::statement::Machine;
use crate::statement::Statement;
use crate::statement::Statement::{Sequence, While};

pub fn fibonacci_n(n: i32) -> i32 {
    // a = 0
    // b = 1
    // count = 0
    // while count < n {
    //   tmp = b
    //   b = a + b
    //   a = tmp
    //   count = count + 1
    // }
    // return b
    let a = Statement::new_assign("a", Number(0));
    let b = Statement::new_assign("b", Number(1));
    let count = Statement::new_assign("count", Number(0));
    let assigns = Sequence {
        first: Box::new(Sequence {
            first: Box::new(a),
            second: Box::new(b),
        }),
        second: Box::new(count),
    };
    let body = Sequence {
        first: Box::new(Sequence {
            first: Box::new(Statement::new_assign("tmp", Expression::new_var("b"))),
            second: Box::new(Statement::new_assign(
                "b",
                Add {
                    left: Box::new(Expression::new_var("a")),
                    right: Box::new(Expression::new_var("b")),
                },
            )),
        }),
        second: Box::new(Sequence {
            first: Box::new(Statement::new_assign("a", Expression::new_var("tmp"))),
            second: Box::new(Statement::new_assign(
                "count",
                Add {
                    left: Box::new(Expression::new_var("count")),
                    right: Box::new(Number(1)),
                },
            )),
        }),
    };
    let while_s = While {
        condition: LessThan {
            left: Box::new(Variable("count".to_string())),
            right: Box::new(Number(n)),
        },
        body: Box::new(body),
    };
    let s = Sequence {
        first: Box::new(assigns),
        second: Box::new(while_s),
    };
    let mut m = Machine {
        statement: s,
        environment: Expression::new_env(),
    };
    m.run();
    m.environment
        .get(&"b".to_string())
        .expect("something wrong :(")
        .get_number()
        .unwrap()
}
