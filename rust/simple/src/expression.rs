use std::collections::HashMap;
pub type Environment = HashMap<String, Expression>;

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Number(i32),
    Boolean(bool),
    Add {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Multiply {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    LessThan {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Variable(String),
}

use Expression::{Add, Boolean, LessThan, Multiply, Number, Variable};

impl Expression {
    pub fn to_string(&self) -> String {
        match self {
            Number(value) => value.to_string(),
            Boolean(value) => value.to_string(),
            Add { left, right } => format!("{} + {}", left.to_string(), right.to_string()),
            Multiply { left, right } => format!("{} * {}", left.to_string(), right.to_string()),
            LessThan { left, right } => format!("{} < {}", left.to_string(), right.to_string()),
            Variable(name) => name.to_string(),
        }
    }

    pub fn reducible(&self) -> bool {
        match self {
            Number(_) | Boolean(_) => false,
            _ => true,
        }
    }

    pub fn reduce(self, env: &Environment) -> Expression {
        match self {
            Number(_) => unreachable!(),
            Boolean(_) => unreachable!(),
            // Box<T>のdereferenceは*でよい
            Add { left, right } => reduce_add(*left, *right, env),
            Multiply { left, right } => reduce_multiply(*left, *right, env),
            LessThan { left, right } => reduce_lessthan(*left, *right, env),
            Variable(name) => reduce_variable(name, env),
        }
    }

    pub fn evaluate(self, env: &Environment) -> Expression {
        match self {
            Number(_) => self,
            Boolean(_) => self,
            Add { left, right } => {
                let eval_l = left.evaluate(env);
                let eval_r = right.evaluate(env);
                Number(eval_l.get_number().unwrap() + eval_r.get_number().unwrap())
            }
            Multiply { left, right } => {
                let eval_l = left.evaluate(env);
                let eval_r = right.evaluate(env);
                Number(eval_l.get_number().unwrap() * eval_r.get_number().unwrap())
            }
            LessThan { left, right } => {
                let eval_l = left.evaluate(env);
                let eval_r = right.evaluate(env);
                Boolean(eval_l.get_number().unwrap() < eval_r.get_number().unwrap())
            }
            Variable(name) => env
                .get(&name)
                .expect(&format!("variable {} does not exist", name))
                .clone(),
        }
    }

    pub fn to_ruby(&self) -> String {
        match self {
            Number(_) | Boolean(_) => format!("-> e {{ {} }}", self.to_string()),
            Variable(name) => format!("-> e {{ e[:{}] }}", name.to_string()),
            Add { left, right } => format!(
                "-> e {{ ({}).call(e) + ({}).call(e) }}",
                left.to_ruby(),
                right.to_ruby()
            ),
            Multiply { left, right } => format!(
                "-> e {{ ({}).call(e) * ({}).call(e) }}",
                left.to_ruby(),
                right.to_ruby()
            ),
            LessThan { left, right } => format!(
                "-> e {{ ({}).call(e) < ({}).call(e) }}",
                left.to_ruby(),
                right.to_ruby()
            ),
        }
    }

    pub fn get_number(&self) -> Result<i32, String> {
        match self {
            Number(value) => Ok(value.clone()),
            _ => Err("not a number".to_string()),
        }
    }

    pub fn get_bool(&self) -> Result<bool, String> {
        match self {
            Boolean(value) => Ok(value.clone()),
            _ => Err("not a boolean".to_string()),
        }
    }

    pub fn new_add(left: i32, right: i32) -> Expression {
        Add {
            left: Box::new(Number(left)),
            right: Box::new(Number(right)),
        }
    }

    pub fn new_multiply(left: i32, right: i32) -> Expression {
        Multiply {
            left: Box::new(Number(left)),
            right: Box::new(Number(right)),
        }
    }

    pub fn new_env() -> Environment {
        let env: Environment = HashMap::new();
        return env;
    }

    pub fn new_var<T: ToString>(name: T) -> Expression {
        Variable(name.to_string())
    }
}

fn reduce_add(left: Expression, right: Expression, env: &Environment) -> Expression {
    if left.reducible() {
        Add {
            left: Box::new(left.reduce(env)),
            right: Box::new(right),
        }
    } else if right.reducible() {
        Add {
            left: Box::new(left),
            right: Box::new(right.reduce(env)),
        }
    } else {
        Number(left.get_number().unwrap() + right.get_number().unwrap())
    }
}

fn reduce_multiply(left: Expression, right: Expression, env: &Environment) -> Expression {
    if left.reducible() {
        Multiply {
            left: Box::new(left.reduce(env)),
            right: Box::new(right),
        }
    } else if right.reducible() {
        Multiply {
            left: Box::new(left),
            right: Box::new(right.reduce(env)),
        }
    } else {
        Number(left.get_number().unwrap() * right.get_number().unwrap())
    }
}

fn reduce_lessthan(left: Expression, right: Expression, env: &Environment) -> Expression {
    if left.reducible() {
        LessThan {
            left: Box::new(left.reduce(env)),
            right: Box::new(right),
        }
    } else if right.reducible() {
        LessThan {
            left: Box::new(left),
            right: Box::new(right.reduce(env)),
        }
    } else {
        Boolean(left.get_number().unwrap() < right.get_number().unwrap())
    }
}

fn reduce_variable(name: String, env: &Environment) -> Expression {
    env.get(&name)
        .expect(&format!("variable {} does not exist", name))
        .clone()
}

#[cfg(test)]
mod tests {
    use super::Expression::{Add, Boolean, LessThan, Multiply, Number, Variable};
    use super::{
        reduce_add, reduce_lessthan, reduce_multiply, reduce_variable, Environment, Expression,
    };
    use std::collections::HashMap;

    #[test]
    fn test_reduce_add() {
        let env: Environment = HashMap::new();
        let a = Expression::new_add(1, 2);
        let b = Number(11);
        assert_eq!(
            reduce_add(a.clone(), b.clone(), &env),
            Expression::new_add(3, 11)
        );
        assert_eq!(
            reduce_add(b.clone(), a.clone(), &env),
            Expression::new_add(11, 3)
        );
    }

    #[test]
    fn test_reduce_multiply() {
        let env: Environment = HashMap::new();
        let a = Expression::new_multiply(2, 3);
        let b = Number(11);
        assert_eq!(
            reduce_multiply(a.clone(), b.clone(), &env),
            Expression::new_multiply(6, 11)
        );
        assert_eq!(
            reduce_multiply(b.clone(), a.clone(), &env),
            Expression::new_multiply(11, 6)
        );
    }

    #[test]
    fn test_reduce_lessthan() {
        let env: Environment = HashMap::new();
        let a = Number(3);
        let b = Number(5);
        assert_eq!(reduce_lessthan(a.clone(), b.clone(), &env), Boolean(true));
        assert_eq!(reduce_lessthan(b.clone(), a.clone(), &env), Boolean(false));
    }

    #[test]
    fn test_reduce_variable() {
        let mut env: Environment = HashMap::new();
        env.insert("x".to_string(), Number(10));
        let expected = Number(10);
        assert_eq!(reduce_variable("x".to_string(), &env), expected);
    }

    #[test]
    #[should_panic]
    fn test_reduce_number() {
        let n = Number(10);
        let env: Environment = HashMap::new();
        n.reduce(&env);
    }

    #[test]
    #[should_panic]
    fn test_reduce_boolean() {
        let n = Boolean(false);
        let env: Environment = HashMap::new();
        n.reduce(&env);
    }

    #[test]
    fn test_reduce() {
        let mut env: Environment = HashMap::new();
        let a = Add {
            left: Box::new(Number(10)),
            right: Box::new(Number(20)),
        };
        assert_eq!(a.reduce(&env), Number(30));

        let m = Multiply {
            left: Box::new(Number(3)),
            right: Box::new(Number(4)),
        };
        assert_eq!(m.reduce(&env), Number(12));

        let lt = LessThan {
            left: Box::new(Number(5)),
            right: Box::new(Number(3)),
        };
        assert_eq!(lt.reduce(&env), Boolean(false));

        let v = Variable("x".to_string());
        env.insert("x".to_string(), Number(10));
        assert_eq!(v.reduce(&env), Number(10));
    }

    #[test]
    fn test_to_string() {
        let n = Number(43);
        let b = Boolean(true);
        let a = Expression::new_add(3, 4);
        let m = Expression::new_multiply(3, 4);
        let lt = LessThan {
            left: Box::new(Number(3)),
            right: Box::new(Number(4)),
        };
        assert_eq!(n.to_string(), 43.to_string());
        assert_eq!(b.to_string(), true.to_string());
        assert_eq!(a.to_string(), "3 + 4".to_string());
        assert_eq!(m.to_string(), "3 * 4".to_string());
        assert_eq!(lt.to_string(), "3 < 4".to_string());
    }

    #[test]
    fn test_reducible() {
        let n = Number(43);
        let b = Boolean(true);
        let a = Expression::new_add(3, 4);
        let m = Expression::new_multiply(3, 4);
        let lt = LessThan {
            left: Box::new(Number(3)),
            right: Box::new(Number(4)),
        };
        assert!(!n.reducible());
        assert!(!b.reducible());
        assert!(a.reducible());
        assert!(m.reducible());
        assert!(lt.reducible());
    }

    #[test]
    fn test_evaluate() {
        let env = Expression::new_env();
        let n = Number(43);
        let b = Boolean(true);
        // 3 + (4 * 5)
        let a = Add {
            left: Box::new(Number(3)),
            right: Box::new(Multiply {
                left: Box::new(Number(4)),
                right: Box::new(Number(5)),
            }),
        };
        // 3 * (4 + 5)
        let m = Multiply {
            left: Box::new(Number(3)),
            right: Box::new(Add {
                left: Box::new(Number(4)),
                right: Box::new(Number(5)),
            }),
        };
        let lt = LessThan {
            left: Box::new(Number(3)),
            right: Box::new(Number(4)),
        };
        assert_eq!(n.evaluate(&env), Number(43));
        assert_eq!(b.evaluate(&env), Boolean(true));
        assert_eq!(a.evaluate(&env), Number(23));
        assert_eq!(m.evaluate(&env), Number(27));
        assert_eq!(lt.evaluate(&env), Boolean(true));
    }
}
