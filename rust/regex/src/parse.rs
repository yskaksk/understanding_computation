use crate::pattern::Pattern::{self, *};

struct Reader {
    regex: Vec<char>,
    index: i32,
}

impl Reader {
    fn new(regex: &str) -> Self {
        return Reader {
            regex: regex.chars().collect(),
            index: 0,
        };
    }

    fn current(&self) -> char {
        return self.regex[self.index as usize];
    }

    fn at_end(&self) -> bool {
        return self.index as usize == self.regex.len();
    }

    fn is_literal(&self) -> bool {
        if self.at_end() {
            return false;
        }
        if vec!['*', '|', '(', ')'].contains(&self.current()) {
            return false;
        }
        return true;
    }

    fn expect(&mut self, c: char) {
        if self.at_end() {
            eprintln!("expected {} but input ends", c);
            std::process::exit(1);
        }
        if self.current() == c {
            self.step()
        } else {
            eprintln!("expected {} but not", c);
            std::process::exit(1);
        }
    }

    fn is(&mut self, c: char) -> bool {
        if self.at_end() {
            return false
        }
        return self.current() == c;
    }

    fn step(&mut self) {
        self.index += 1;
    }
}

pub fn parse(reg: String) -> Pattern {
    let mut reader = Reader::new(&reg);
    return choose(&mut reader);
}

// choose = concatenate_or_empty ("|" choose)?
fn choose(r: &mut Reader) -> Pattern {
    let f = concatenate_or_empty(r);
    if r.is('|') {
        r.step();
        let s = choose(r);
        return Choose {
            first: Box::new(f),
            second: Box::new(s),
        };
    } else {
        return f;
    }
}

// connatenate_or_empty = concatenate | empty
fn concatenate_or_empty(r: &mut Reader) -> Pattern {
    if r.is('|') {
        return Empty
    } else {
        return concatenate(r)
    }
}

// concatenate = repeat (concatenate)?
fn concatenate(r: &mut Reader) -> Pattern {
    let first = Box::new(repeat(r));
    let second = Box::new(if r.is_literal() {
        concatenate(r)
    } else {
        Empty
    });
    return Concatenate { first, second };
}

// repeat = brackets("*")?
fn repeat(r: &mut Reader) -> Pattern {
    let b = brackets(r);
    if r.is('*') {
        r.step();
        return Repeat(Box::new(b));
    } else {
        return b;
    }
}

// brackets = "(" choose ")" | literal
fn brackets(r: &mut Reader) -> Pattern {
    return if r.is('(') {
        r.step();
        let pat = choose(r);
        r.expect(')');
        pat
    } else {
        literal(r)
    };
}

// literal
fn literal(r: &mut Reader) -> Pattern {
    let c = r.current();
    r.step();
    return Literal { character: c };
}
