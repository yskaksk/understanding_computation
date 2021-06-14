use simple::expression::Expression;
use simple::expression::Expression::{Add, Boolean, LessThan, Number, Variable};
use simple::functions::fibonacci_n;
use simple::statement::Statement::{Assign, While};

fn main() {
    let mut env = Expression::new_env();
    env.insert("x".to_string(), Number(99));

    let n = Number(1);
    println!("Number(1) = {}", n.to_string());

    let b = Boolean(true);
    println!("Boolean(true) = {}", b.to_string());

    let a = Expression::new_add(1, 2);
    println!("Add(1, 2) = {}", a.to_string());
    println!("Add(1, 2).reduce(&env) = {}", a.reduce(&env).to_string());

    let m = Expression::new_multiply(1, 2);
    println!("Multiply(1, 2) = {}", m.to_string());
    println!(
        "Multiply(1, 2).reduce(&env) = {}",
        m.reduce(&env).to_string()
    );

    let lt = LessThan {
        left: Box::new(Number(1)),
        right: Box::new(Number(11)),
    };
    println!("LessThan(1, 11) = {}", lt.to_string());
    println!(
        "LessThan(1, 11).reduce(&env) = {}",
        lt.reduce(&env).to_string()
    );

    let v = Variable("x".to_string());
    println!("Variable(x) = {}", v.to_string());
    println!("Variable(x).reduce(&env) = {}", v.reduce(&env).to_string());

    let f_10 = fibonacci_n(10);
    println!("10th fibonacci number is {}", f_10);

    println!("");
    println!("to_ruby: Expression");
    println!("");
    let lt = LessThan {
        left: Box::new(Add {
            left: Box::new(Expression::new_var("x")),
            right: Box::new(Number(1)),
        }),
        right: Box::new(Number(3)),
    };
    println!("let lt = LessThan {{");
    println!("    left: Box::new(Add {{");
    println!("        left: Box::new(Expression::new_var(\"x\")),");
    println!("        right: Box::new(Number(1)),");
    println!("}}),");
    println!("right: Box::new(Number(3)),");
    println!("}};");
    println!("lt.to_ruby()");
    println!("{}", lt.to_ruby());

    println!("to_ruby: Statement");
    let st = While {
        condition: LessThan {
            left: Box::new(Expression::new_var("x")),
            right: Box::new(Number(3)),
        },
        body: Box::new(Assign {
            name: "x".to_string(),
            expression: Expression::Multiply {
                left: Box::new(Expression::new_var("x")),
                right: Box::new(Number(3)),
            },
        }),
    };
    println!("{}", st.to_ruby());
}
