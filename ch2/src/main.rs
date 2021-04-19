fn main() {
    let one = Elm::new_number(1);
    let two = Elm::new_number(2);
    let three = Elm::new_number(3);
    let four = Elm::new_number(4);

    let p = Elm::new_add(four, Elm::new_mul(Elm::new_add(one, two), three));

    println!("{}", p.clone().to_string());
    println!("{}", p.reduce().to_string());
    println!("{}", p.reduce().reduce().to_string());
    println!("{}", p.reduce().reduce().reduce().to_string());
}

#[derive(Clone)]
enum Elm {
    Number { value: i32 },
    Add {
        right: Box<Elm>,
        left: Box<Elm>
    },
    Multiply {
        right: Box<Elm>,
        left: Box<Elm>
    }
}

impl Elm {
    fn to_string(&self) -> String {
        match self {
            Elm::Number { value } => { value.to_string() }
            Elm::Add { right, left } => {
                let s = right.to_string().to_string() + " + " + &left.to_string();
                s
            }
            Elm::Multiply { right, left } => {
                let s = right.to_string().to_string() + " * " + &left.to_string();
                s
            }
        }
    }

    fn reducible(&self) -> bool {
        match self {
            Elm::Number { value: _ } => { false }
            _ => { true }
        }
    }

    fn new_number(value: i32) -> Elm {
        Elm::Number {value}
    }

    fn new_add(lhs: Elm, rhs: Elm) -> Elm {
        Elm::Add {
            left: Box::new(lhs),
            right: Box::new(rhs)
        }
    }

    fn new_mul(lhs: Elm, rhs: Elm) -> Elm {
        Elm::Multiply {
            left: Box::new(lhs),
            right: Box::new(rhs)
        }
    }

    fn get_val(num: Box<Elm>) -> i32 {
        match *num {
            Elm::Number { value: val } => val,
            _ => panic!("haha")
        }
    }

    fn reduce(&self) -> Elm {
        match self {
            Elm::Number { value: _ } => { self.clone() },
            Elm::Add { right, left } => {
                if right.reducible() {
                    Elm::Add {
                        right: Box::new(right.reduce()),
                        left: left.clone()
                    }
                } else if left.reducible() {
                    Elm::Add {
                        right: right.clone(),
                        left: Box::new(left.reduce())
                    }
                } else {
                    Elm::Number {
                        value: Elm::get_val(right.clone()) + Elm::get_val(left.clone())
                    }
                }
            },
            Elm::Multiply { right, left} => {
                if right.reducible() {
                    Elm::Multiply {
                        right: Box::new(right.reduce()),
                        left: left.clone()
                    }
                } else if left.reducible() {
                    Elm::Multiply {
                        right: right.clone(),
                        left: Box::new(left.reduce())
                    }
                } else {
                    Elm::Number {
                        value: Elm::get_val(right.clone()) * Elm::get_val(left.clone())
                    }
                }
            }
        }
    }
}

