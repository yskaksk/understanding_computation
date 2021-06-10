use simple::expression::Expression;
use simple::expression::Expression::{Number, Boolean, LessThan, Variable};
use simple::functions::fibonacci_n;

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
    println!("Multiply(1, 2).reduce(&env) = {}", m.reduce(&env).to_string());

    let lt = LessThan {
        left: Box::new(Number(1)),
        right: Box::new(Number(11))
    };
    println!("LessThan(1, 11) = {}", lt.to_string());
    println!("LessThan(1, 11).reduce(&env) = {}", lt.reduce(&env).to_string());

    let v = Variable("x".to_string());
    println!("Variable(x) = {}", v.to_string());
    println!("Variable(x).reduce(&env) = {}", v.reduce(&env).to_string());

    let f_10 = fibonacci_n(10);
    println!("10th fibonacci number is {}", f_10);
}
