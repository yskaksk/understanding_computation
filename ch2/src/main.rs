use std::collections::HashMap;

fn main() {
    let one = Expression::new_number(1);
    let two = Expression::new_number(2);
    let three = Expression::new_number(3);
    let four = Expression::new_number(4);
    let var_x = Expression::new_variable("x".to_string());
    let var_y = Expression::new_variable("y".to_string());

    let p = Expression::LessThan {
        left: Box::new(Expression::new_add(Expression::new_mul(three, four), var_x)),
        right: Box::new(Expression::new_mul(Expression::new_add(one, two), var_y))
    };

    let mut env = HashMap::new();
    env.insert("x".to_string(), 10);
    env.insert("y".to_string(), 88);

    let mut m = Machine {
        exp: p.clone()
    };

    let res = m.run(env);
    println!("{}", res.to_string());
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

#[derive(Clone)]
enum Statement {
    DoNothing,
    Assign {
        name: String,
        exp: Expression
    }
}

impl Statement {
    fn to_string(&self) -> String {
        match self {
            Statement::DoNothing => "do-nothing".to_string(),
            Statement::Assign {name, exp} => name.to_string() + " = " + &exp.to_string()
        }
    }

    fn reducible(&self) -> bool {
        match self {
            Statement::DoNothing => false,
            _ => true
        }
    }

    fn reduce(&self, env: &Enviroment) -> (Statement, &Enviroment){
        match self {
            Statement::Assign { name, exp } => {
                if exp.reducible() {
                    (Statement::Assign{
                        name: name.to_string(),
                        exp: exp.reduce(&env.clone())
                    }, env)
                } else {
                    env.insert(name.to_string(), *exp);
                    (Statement::DoNothing, env)
                }
            },
            Statement::DoNothing => (Statement::DoNothing, env)
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
            Expression::Add { left, right } => left.to_string().to_string() + " + " + &right.to_string(),
            Expression::Multiply { left, right } => left.to_string().to_string() + " * " + &right.to_string(),
            Expression::Boolean { value } => value.to_string(),
            Expression::LessThan {left, right} => left.to_string().to_string() + " < " + &right.to_string(),
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
                    Some(value) => Expression::new_number(*value),
                    None => unreachable!(),
                }
            }
        }
    }
}
