use crate::expression::{Environment, Expression};

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    DoNothing,
    Assign {
        name: String,
        expression: Expression,
    },
    If {
        condition: Expression,
        consequence: Box<Statement>,
        alternative: Box<Statement>,
    },
    While {
        condition: Expression,
        body: Box<Statement>,
    },
    Sequence {
        first: Box<Statement>,
        second: Box<Statement>,
    },
}

use Statement::{Assign, DoNothing, If, Sequence, While};

impl Statement {
    pub fn to_string(&self) -> String {
        match self {
            DoNothing => "do-nothing".to_string(),
            Assign { name, expression } => format!("{} = {}", name, expression.to_string()),
            If {
                condition,
                consequence,
                alternative,
            } => format!(
                "If {} {{ {} }} else {{ {} }}",
                condition.to_string(),
                consequence.to_string(),
                alternative.to_string()
            ),
            While { condition, body } => format!(
                "While {{ {} }} {{ {} }}",
                condition.to_string(),
                body.to_string()
            ),
            Sequence { first, second } => format!("{}; {}", first.to_string(), second.to_string()),
        }
    }
    fn reducible(&self) -> bool {
        match self {
            DoNothing => false,
            _ => true,
        }
    }
    fn reduce(self, env: &mut Environment) -> (Statement, Environment) {
        match self {
            DoNothing => unreachable!(),
            Assign { name, expression } => reduce_assign(name, expression, env),
            If {
                condition,
                consequence,
                alternative,
            } => reduce_if(condition, *consequence, *alternative, env),
            While { condition, body } => reduce_while(condition, *body, env),
            Sequence { first, second } => reduce_sequence(*first, *second, env),
        }
    }
    pub fn evaluate(self, env: &mut Environment) -> Environment {
        match self {
            DoNothing => env.clone(),
            Assign { name, expression } => {
                env.insert(name, expression.evaluate(env));
                env.clone()
            }
            If {
                condition,
                consequence,
                alternative,
            } => match condition.evaluate(env) {
                Expression::Boolean(true) => consequence.evaluate(env),
                Expression::Boolean(false) => alternative.evaluate(env),
                _ => unreachable!(),
            },
            While { condition, body } => evaluate_while(condition, *body, env),
            Sequence { first, second } => second.evaluate(&mut first.evaluate(env)),
        }
    }
}

fn reduce_assign(name: String, exp: Expression, env: &mut Environment) -> (Statement, Environment) {
    if exp.reducible() {
        let a = Assign {
            name: name.to_string(),
            expression: exp.reduce(env),
        };
        (a, env.clone())
    } else {
        env.insert(name.to_string(), exp.clone());
        (DoNothing, env.clone())
    }
}

fn reduce_if(
    cond: Expression,
    cons: Statement,
    alt: Statement,
    env: &mut Environment,
) -> (Statement, Environment) {
    if cond.reducible() {
        let if_s = If {
            condition: cond.reduce(env),
            consequence: Box::new(cons),
            alternative: Box::new(alt),
        };
        (if_s, env.clone())
    } else {
        match cond {
            Expression::Boolean(true) => (cons, env.clone()),
            Expression::Boolean(false) => (alt, env.clone()),
            _ => unreachable!(),
        }
    }
}

fn reduce_while(
    cond: Expression,
    body: Statement,
    env: &mut Environment,
) -> (Statement, Environment) {
    let cons = Sequence {
        first: Box::new(body.clone()),
        second: Box::new(While {
            condition: cond.clone(),
            body: Box::new(body),
        }),
    };
    let if_s = If {
        condition: cond,
        consequence: Box::new(cons),
        alternative: Box::new(DoNothing),
    };
    (if_s, env.clone())
}

fn reduce_sequence(f: Statement, s: Statement, env: &mut Environment) -> (Statement, Environment) {
    match f {
        DoNothing => s.reduce(env),
        _ => {
            let (new_f, new_env) = f.reduce(env);
            (
                Sequence {
                    first: Box::new(new_f),
                    second: Box::new(s),
                },
                new_env,
            )
        }
    }
}

fn evaluate_while(cond: Expression, body: Statement, env: &mut Environment) -> Environment {
    match cond.clone().evaluate(env) {
        Expression::Boolean(true) => {
            let while_s = While {
                condition: cond.clone(),
                body: Box::new(body.clone()),
            };
            while_s.evaluate(&mut body.evaluate(env))
        }
        Expression::Boolean(false) => env.clone(),
        _ => unreachable!(),
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Machine {
    pub statement: Statement,
    pub environment: Environment,
}

impl Machine {
    fn step(&mut self) {
        let mut env = self.environment.clone();
        let st = self.statement.clone();
        let (statement, environment) = st.reduce(&mut env);
        self.statement = statement;
        self.environment = environment;
    }

    pub fn run(&mut self) {
        while self.statement.reducible() {
            self.step();
        }
    }

    pub fn evaluate(self) -> Environment {
        let mut env = self.environment.clone();
        self.statement.evaluate(&mut env)
    }
}

#[cfg(test)]
mod tests {
    use super::Expression;
    use super::Machine;
    use super::{Assign, DoNothing, If, Sequence, While};

    #[test]
    fn test_to_string() {
        let dn = DoNothing;
        assert_eq!(dn.to_string(), "do-nothing".to_string());

        let assign = Assign {
            name: "x".to_string(),
            expression: Expression::new_add(1, 2),
        };
        assert_eq!(assign.to_string(), "x = 1 + 2".to_string());

        let if_s = If {
            condition: Expression::Boolean(true),
            consequence: Box::new(Assign {
                name: "x".to_string(),
                expression: Expression::Number(1),
            }),
            alternative: Box::new(DoNothing),
        };
        assert_eq!(if_s.to_string(), "If true { x = 1 } else { do-nothing }");

        let while_s = While {
            condition: Expression::Boolean(true),
            body: Box::new(Assign {
                name: "x".to_string(),
                expression: Expression::Number(22),
            }),
        };
        assert_eq!(while_s.to_string(), "While { true } { x = 22 }");

        let seq = Sequence {
            first: Box::new(Assign {
                name: "x".to_string(),
                expression: Expression::Number(11),
            }),
            second: Box::new(Assign {
                name: "y".to_string(),
                expression: Expression::Number(22),
            }),
        };
        assert_eq!(seq.to_string(), "x = 11; y = 22");
    }

    #[test]
    fn test_reduce() {
        let assign1 = Assign {
            name: "x".to_string(),
            expression: Expression::Number(1),
        };
        let mut env = Expression::new_env();
        let (reduced, new_env) = assign1.reduce(&mut env);
        let mut expected_env = Expression::new_env();
        expected_env.insert("x".to_string(), Expression::Number(1));
        assert_eq!(reduced, DoNothing);
        assert_eq!(new_env, expected_env);

        let assign2 = Assign {
            name: "x".to_string(),
            expression: Expression::new_add(1, 2),
        };
        let mut env = Expression::new_env();
        let (reduced, new_env) = assign2.reduce(&mut env);
        let expected = Assign {
            name: "x".to_string(),
            expression: Expression::Number(3),
        };
        assert_eq!(reduced, expected);
        assert_eq!(new_env, env);

        let if_1 = If {
            condition: Expression::LessThan {
                left: Box::new(Expression::Number(1)),
                right: Box::new(Expression::Number(22)),
            },
            consequence: Box::new(DoNothing),
            alternative: Box::new(DoNothing),
        };
        let mut env = Expression::new_env();
        let (reduced, new_env) = if_1.reduce(&mut env);
        let expected = If {
            condition: Expression::Boolean(true),
            consequence: Box::new(DoNothing),
            alternative: Box::new(DoNothing),
        };
        assert_eq!(reduced, expected);
        assert_eq!(new_env, env);

        let if_2 = If {
            condition: Expression::Boolean(true),
            consequence: Box::new(Assign {
                name: "x".to_string(),
                expression: Expression::Number(1),
            }),
            alternative: Box::new(DoNothing),
        };
        let mut env = Expression::new_env();
        let (reduced, new_env) = if_2.reduce(&mut env);
        let expected = Assign {
            name: "x".to_string(),
            expression: Expression::Number(1),
        };
        assert_eq!(reduced, expected);
        assert_eq!(new_env, env);

        let body = Box::new(Assign {
            name: "x".to_string(),
            expression: Expression::Number(1),
        });
        let while_s = While {
            condition: Expression::Boolean(true),
            body: body.clone(),
        };
        let mut env = Expression::new_env();
        let (reduced, new_env) = while_s.reduce(&mut env);
        let expected = If {
            condition: Expression::Boolean(true),
            consequence: Box::new(Sequence {
                first: body.clone(),
                second: Box::new(While {
                    condition: Expression::Boolean(true),
                    body: body.clone(),
                }),
            }),
            alternative: Box::new(DoNothing),
        };
        assert_eq!(reduced, expected);
        assert_eq!(new_env, env);

        let seq_1 = Sequence {
            first: Box::new(DoNothing),
            second: Box::new(Assign {
                name: "x".to_string(),
                expression: Expression::Number(1),
            }),
        };
        let mut env = Expression::new_env();
        let (reduced, new_env) = seq_1.reduce(&mut env);
        let expected = DoNothing;
        env.insert("x".to_string(), Expression::Number(1));
        assert_eq!(reduced, expected);
        assert_eq!(new_env, env);
    }

    #[test]
    fn test_evaluate() {
        let mut env = Expression::new_env();
        let assign = Assign {
            name: "x".to_string(),
            expression: Expression::Number(11),
        };
        let mut expected = Expression::new_env();
        expected.insert("x".to_string(), Expression::Number(11));
        assert_eq!(DoNothing.evaluate(&mut env), env);
        assert_eq!(assign.evaluate(&mut env), expected);

        let if_s = If {
            condition: Expression::Boolean(true),
            consequence: Box::new(Assign {
                name: "x".to_string(),
                expression: Expression::Number(999),
            }),
            alternative: Box::new(DoNothing),
        };
        let mut expected = Expression::new_env();
        expected.insert("x".to_string(), Expression::Number(999));
        assert_eq!(if_s.evaluate(&mut env), expected);

        let while_s = While {
            condition: Expression::Boolean(false),
            body: Box::new(Assign {
                name: "x".to_string(),
                expression: Expression::Number(1),
            }),
        };
        assert_eq!(while_s.evaluate(&mut env), env);

        let seq = Sequence {
            first: Box::new(Assign {
                name: "x".to_string(),
                expression: Expression::Number(1),
            }),
            second: Box::new(Assign {
                name: "x".to_string(),
                expression: Expression::Number(999),
            }),
        };
        let mut expected = Expression::new_env();
        expected.insert("x".to_string(), Expression::Number(999));
        assert_eq!(seq.evaluate(&mut env), expected);
    }

    #[test]
    fn test_machine() {
        let mut m = Machine {
            statement: Sequence {
                first: Box::new(Assign {
                    name: "x".to_string(),
                    expression: Expression::Number(1),
                }),
                second: Box::new(Assign {
                    name: "y".to_string(),
                    expression: Expression::Number(2),
                }),
            },
            environment: Expression::new_env(),
        };
        m.run();
        let mut new_env = Expression::new_env();
        new_env.insert("x".to_string(), Expression::Number(1));
        new_env.insert("y".to_string(), Expression::Number(2));
        let expected = Machine {
            statement: DoNothing,
            environment: new_env,
        };
        assert_eq!(m, expected);
    }
}
