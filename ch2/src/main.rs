use std::collections::HashMap;

fn main() {
    //let one = Expression::new_number(1);
    //let two = Expression::new_number(2);
    //let var_x = Expression::new_variable("x".to_string());
    //let var_y = Expression::new_variable("y".to_string());

    //let p = Expression::LessThan {
    //    left: Box::new(Expression::new_add(one, var_x)),
    //    right: Box::new(Expression::new_mul(two, var_y))
    //};

    //let mut env = HashMap::new();
    //env.insert("x".to_string(), Expression::new_number(10));
    //env.insert("y".to_string(), Expression::new_number(88));

    //let mut m = Machine {
    //    exp: p.clone()
    //};

    //let res = m.run(env);
    //println!("{}", res.to_string());

    //let if_s = Statement::If {
    //    condition: Expression::Variable{name: "x".to_string()},
    //    consequence: Box::new(Statement::Assign{
    //        name: "y".to_string(),
    //        exp: Expression::new_number(1)
    //    }),
    //    alternative: Box::new(Statement::Assign{
    //        name: "y".to_string(),
    //        exp: Expression::new_number(2)
    //    })
    //};
    let while_s = Statement::While {
        condition: Expression::LessThan{
            left: Box::new(Expression::new_variable("x".to_string())),
            right: Box::new(Expression::new_number(5))
        },
        body: Box::new(Statement::Assign{
            name: "x".to_string(),
            exp: Expression::new_mul(
                Expression::new_variable("x".to_string()),
                Expression::new_number(3)
            )
        })
    };
    let assigns = Statement::Sequence{
        first: Box::new(Statement::Assign{
            name: "c".to_string(),
            exp: Expression::new_number(0)
        }),
        second: Box::new(Statement::Assign{
            name: "r".to_string(),
            exp: Expression::new_number(1)
        })
    };
    let power_of_two = Statement::Sequence{
        first: Box::new(assigns),
        second: Box::new(Statement::While{
            condition: Expression::LessThan{
                left: Box::new(Expression::new_variable("c".to_string())),
                right: Box::new(Expression::new_number(5))
            },
            body: Box::new(Statement::Sequence{
                first: Box::new(Statement::Assign{
                    name: "r".to_string(),
                    exp: Expression::new_mul(
                        Expression::new_variable("r".to_string()),
                        Expression::new_number(2)
                    )
                }),
                second: Box::new(Statement::Assign{
                    name: "c".to_string(),
                    exp: Expression::new_add(
                        Expression::new_variable("c".to_string()),
                        Expression::new_number(1)
                    )
                }),
            })
        })
    };
    let env = HashMap::new();
    //env.insert("x".to_string(), Expression::new_number(1));
    let mut sm = StatementMachine{
        stm: power_of_two,
        env: env
    };
    let r = sm.run();
    for (k, v) in &r {
        println!("{}: {}", k, v.to_string());
    }
}

type Enviroment = HashMap<String, Expression>;

struct Machine {
    exp: Expression
}

impl Machine {
    fn step(&mut self, env: &Enviroment) {
        self.exp = self.exp.clone().reduce(env);
    }

    fn run(&mut self, env: Enviroment) -> Expression {
        while self.exp.reducible() {
            println!("{}", self.exp.clone().to_string());
            self.step(&env);
        }
        return self.exp.clone()
    }
}

struct StatementMachine {
    stm: Statement,
    env: Enviroment
}

impl StatementMachine {
    fn step(&mut self) {
        println!("{}", self.stm.to_string());
        let (stm, env) = self.stm.clone().reduce(&self.env);
        self.stm = stm;
        self.env = env;
    }
    fn run(&mut self) -> Enviroment {
        while self.stm.reducible() {
            self.step();
        }
        return self.env.clone()
    }
}

#[derive(Clone)]
enum Statement {
    DoNothing,
    Assign {
        name: String,
        exp: Expression
    },
    If {
        condition: Expression,
        consequence: Box<Statement>,
        alternative: Box<Statement>
    },
    Sequence {
        first: Box<Statement>,
        second: Box<Statement>
    },
    While {
        condition: Expression,
        body: Box<Statement>
    }
}

impl Statement {
    fn to_string(&self) -> String {
        match self {
            Statement::DoNothing => "do-nothing".to_string(),
            Statement::Assign {name, exp} => name.to_string() + " = " + &exp.to_string(),
            Statement::If {condition, consequence, alternative} => "If ".to_string() +
                &condition.to_string() + &" { ".to_string() + &consequence.to_string() +
                &" } else { ".to_string() + &alternative.to_string() + &" }".to_string(),
            Statement::Sequence {first, second} => first.to_string() + &"; ".to_string() + &second.to_string(),
            Statement::While {condition, body} => "While { ".to_string() +
                &condition.to_string() + " } { " + &body.to_string() + &" }".to_string()
        }
    }

    fn reducible(&self) -> bool {
        match self {
            Statement::DoNothing => false,
            _ => true
        }
    }

    fn reduce(self, env: &Enviroment) -> (Statement, Enviroment){
        match self {
            Statement::Assign { name, exp } => {
                if exp.reducible() {
                    (Statement::Assign{
                        name: name.to_string(),
                        exp: exp.reduce(&env)
                    }, env.clone())
                } else {
                    let mut new_env = env.clone();
                    new_env.insert(name.to_string(), exp.clone());
                    (Statement::DoNothing, new_env)
                }
            },
            Statement::If {condition, consequence, alternative} => {
                if condition.reducible() {
                    (Statement::If{
                        condition: condition.reduce(&env),
                        consequence: consequence,
                        alternative: alternative
                    }, env.clone())
                } else {
                    match condition {
                        Expression::Boolean { value } => match value {
                            true => (consequence.as_ref().clone(), env.clone()),
                            false => (alternative.as_ref().clone(), env.clone())
                        },
                        _ => unreachable!()
                    }
                }
            },
            Statement::Sequence {first, second} => {
                match first.as_ref() {
                    Statement::DoNothing => {
                        let (s_stm, s_env) = second.reduce(&env);
                        (s_stm, s_env)
                    },
                    _ => {
                        let (f_stm, f_env) = first.reduce(&env);
                        (Statement::Sequence{
                            first: Box::new(f_stm),
                            second: second
                        }, f_env)
                    }
                }
            },
            Statement::While {condition, body} => {
                (Statement::If{
                    condition: condition.clone(),
                    consequence: Box::new(Statement::Sequence{
                        first: body.clone(),
                        second: Box::new(Statement::While{
                            condition: condition.clone(),
                            body: body.clone()
                        })
                    }),
                    alternative: Box::new(Statement::DoNothing)
                }, env.clone())
            },
            Statement::DoNothing => (Statement::DoNothing, env.clone())
        }
    }
}

#[derive(Clone)]
enum Expression {
    Number { value: i32 },
    Add {
        left: Box<Expression>,
        right: Box<Expression>
    },
    Multiply {
        left: Box<Expression>,
        right: Box<Expression>
    },
    Boolean { value: bool },
    LessThan {
        left: Box<Expression>,
        right: Box<Expression>
    },
    Variable { name: String }
}

impl Expression {
    fn to_string(&self) -> String {
        match self {
            Expression::Number { value } => value.to_string(),
            Expression::Add { left, right } => left.to_string() + " + " + &right.to_string(),
            Expression::Multiply { left, right } => left.to_string() + " * " + &right.to_string(),
            Expression::Boolean { value } => value.to_string(),
            Expression::LessThan {left, right} => left.to_string() + " < " + &right.to_string(),
            Expression::Variable { name } => name.to_string()
        }
    }

    fn reducible(&self) -> bool {
        match self {
            Expression::Number { value: _ } => false,
            Expression::Boolean { value: _ } => false,
            _ => true
        }
    }

    fn new_number(value: i32) -> Expression {
        Expression::Number {value}
    }

    fn new_bool(value: bool) -> Expression {
        Expression::Boolean {value}
    }

    fn new_add(lhs: Expression, rhs: Expression) -> Expression {
        Expression::Add {
            left: Box::new(lhs),
            right: Box::new(rhs)
        }
    }

    fn new_mul(lhs: Expression, rhs: Expression) -> Expression {
        Expression::Multiply {
            left: Box::new(lhs),
            right: Box::new(rhs)
        }
    }

    fn new_variable(name: String) -> Expression {
        Expression::Variable { name }
    }

    fn reduce(&self, env: &Enviroment) -> Expression {
        match self {
            Expression::Number { value: _ } => self.clone(),
            Expression::Boolean { value: _ } => self.clone(),
            Expression::Add { left, right } => {
                if right.reducible() {
                    Expression::Add {
                        right: Box::new(right.reduce(env)),
                        left: left.clone()
                    }
                } else if left.reducible() {
                    Expression::Add {
                        right: right.clone(),
                        left: Box::new(left.reduce(env))
                    }
                } else {
                    match (left.as_ref(), right.as_ref()) {
                        (
                            Expression::Number { value: l_val},
                            Expression::Number { value: r_val}
                        ) => { Expression::new_number(l_val + r_val) },
                        _ => unreachable!(),
                    }
                }
            },
            Expression::Multiply { left, right } => {
                if right.reducible() {
                    Expression::Multiply {
                        right: Box::new(right.reduce(env)),
                        left: left.clone()
                    }
                } else if left.reducible() {
                    Expression::Multiply {
                        right: right.clone(),
                        left: Box::new(left.reduce(env))
                    }
                } else {
                    match (left.as_ref(), right.as_ref()) {
                        (
                            Expression::Number { value: l_val},
                            Expression::Number { value: r_val}
                        ) => { Expression::new_number(l_val * r_val) },
                        _ => unreachable!(),
                    }
                }
            },
            Expression::LessThan { left, right } => {
                if right.reducible() {
                    Expression::LessThan {
                        right: Box::new(right.reduce(env)),
                        left: left.clone()
                    }
                } else if left.reducible() {
                    Expression::LessThan {
                        right: right.clone(),
                        left: Box::new(left.reduce(env))
                    }
                } else {
                    match (left.as_ref(), right.as_ref()) {
                        (
                            Expression::Number { value: l_val},
                            Expression::Number { value: r_val}
                        ) => { Expression::new_bool(l_val < r_val) },
                        _ => unreachable!(),
                    }
                }
            },
            Expression::Variable { name } => {
                match env.get(name) {
                    Some(value) => value.clone(),
                    None => unreachable!(),
                }
            }
        }
    }
}
