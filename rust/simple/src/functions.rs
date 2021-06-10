use crate::expression::Expression;
use crate::expression::Expression::{Number, Variable, Add, LessThan};
use crate::statement::Statement::{Assign, Sequence, While};
use crate::statement::Machine;


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
    let a = Assign {
        name: "a".to_string(),
        expression: Number(0)
    };
    let b = Assign {
        name: "b".to_string(),
        expression: Number(1)
    };
    let count = Assign {
        name: "count".to_string(),
        expression: Number(0)
    };
    let assigns = Sequence {
        first: Box::new(Sequence {
            first: Box::new(a),
            second: Box::new(b)
        }),
        second: Box::new(count)
    };
    let body = Sequence {
        first: Box::new(Sequence {
            first: Box::new(Assign {
                name: "tmp".to_string(),
                expression: Variable("b".to_string())
            }),
            second: Box::new(Assign {
                name: "b".to_string(),
                expression: Add {
                    left: Box::new(Variable("a".to_string())),
                    right: Box::new(Variable("b".to_string()))
                }
            })
        }),
        second: Box::new(Sequence {
            first: Box::new(Assign {
                name: "a".to_string(),
                expression: Variable("tmp".to_string())
            }),
            second: Box::new(Assign {
                name: "count".to_string(),
                expression: Add {
                    left: Box::new(Variable("count".to_string())),
                    right: Box::new(Number(1))
                }
            })
        })
    };
    let while_s = While {
        condition: LessThan {
            left: Box::new(Variable("count".to_string())),
            right: Box::new(Number(n))
        },
        body: Box::new(body)
    };
    let s = Sequence {
        first: Box::new(assigns),
        second: Box::new(while_s)
    };
    let mut m = Machine {
        statement: s,
        environment: Expression::new_env()
    };
    m.run();
    m.environment.get(&"b".to_string()).expect("something wrong :(").get_number().unwrap()
}
