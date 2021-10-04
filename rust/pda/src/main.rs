#![allow(dead_code)]

type State = i32;

struct Stack {
    contents: Vec<char>
}

impl Stack {
    fn new(mut contents: Vec<char>) -> Self {
        contents.reverse();
        Stack {
            contents
        }
    }

    fn push(&self, character: char) -> Self {
        let mut contents = self.contents.clone();
        contents.push(character);
        Stack {
            contents
        }
    }

    fn pop(&self) -> Self {
        let mut contents = self.contents.clone();
        contents.pop();
        Stack {
            contents
        }
    }

    fn top(&self) -> char {
        self.contents[self.contents.len()-1]
    }

    fn inspect(&self) -> String {
        let mut contents = self.contents.clone();
        contents.reverse();
        let first = contents[0];
        let rest: String = contents[1..contents.len()].into_iter().collect();
        format!("#<Stack ({}){}>", first, rest)
    }
}

struct PDAConfiguration {
    state: State,
    stack: Stack
}

struct PDARule {
    state: State,
    character: char,
    next_state: State,
    pop_character: char,
    push_characters: Vec<char>
}

impl PDARule {
    fn applies_to(&self, configuration: PDAConfiguration, character: char) -> bool {
        self.state == configuration.state && self.pop_character == configuration.stack.top() && self.character == character
    }

    fn follow(&self, configuration: PDAConfiguration) -> PDAConfiguration {
        PDAConfiguration {
            state: self.next_state,
            stack: self.next_stack(configuration)
        }
    }

    fn next_stack(&self, configuration: PDAConfiguration) -> Stack {
        let mut popped_stack = configuration.stack.pop();
        let mut push_characters = self.push_characters.clone();
        push_characters.reverse();
        for c in push_characters {
            popped_stack = popped_stack.push(c);
        }
        popped_stack
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::{Stack, PDARule, PDAConfiguration};

    #[test]
    fn test_stack() {
        let mut contents = vec!['a', 'b', 'c', 'd', 'e'];
        contents.reverse();
        let stack = Stack {
            contents
        };
        assert_eq!(stack.top(), 'a');
        assert_eq!(stack.pop().pop().top(), 'c');
        assert_eq!(stack.push('x').push('y').top(), 'y');
        assert_eq!(stack.push('x').push('y').pop().top(), 'x');
    }

    #[test]
    fn test_pdarule() {
        let rule = PDARule {
            state: 1,
            character: '(',
            next_state: 2,
            pop_character: '$',
            push_characters: vec!['b', '$']
        };
        let configuration = PDAConfiguration {
            state: 1,
            stack: Stack::new(vec!['$'])
        };
        assert!(rule.applies_to(configuration, '('));
    }
}
