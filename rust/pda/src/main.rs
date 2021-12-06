#![allow(dead_code)]

type State = i32;
const NIL: char = '\u{029e}';

#[derive(Clone, Eq, PartialEq, Debug)]
struct Stack {
    contents: Vec<char>,
}

impl Stack {
    fn new(mut contents: Vec<char>) -> Self {
        contents.reverse();
        Stack { contents }
    }

    fn push(&self, character: char) -> Self {
        let mut contents = self.contents.clone();
        contents.push(character);
        Stack { contents }
    }

    fn pop(&self) -> Self {
        let mut contents = self.contents.clone();
        contents.pop();
        Stack { contents }
    }

    fn top(&self) -> char {
        self.contents[self.contents.len() - 1]
    }

    fn inspect(&self) -> String {
        let mut contents = self.contents.clone();
        contents.reverse();
        let first = contents[0];
        let rest: String = contents[1..contents.len()].into_iter().collect();
        format!("#<Stack ({}){}>", first, rest)
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct PDAConfiguration {
    state: State,
    stack: Stack,
}

impl PDAConfiguration {
    const STUCK_STATE: State = -1;

    fn stuck(&self) -> Self {
        PDAConfiguration {
            state: Self::STUCK_STATE,
            stack: self.stack.clone(),
        }
    }

    fn is_stuck(&self) -> bool {
        self.state == Self::STUCK_STATE
    }
}

#[derive(Clone, Eq, PartialEq)]
struct PDARule {
    state: State,
    character: char,
    next_state: State,
    pop_character: char,
    push_characters: Vec<char>,
}

impl PDARule {
    fn applies_to(&self, configuration: PDAConfiguration, character: char) -> bool {
        self.state == configuration.state
            && self.pop_character == configuration.stack.top()
            && self.character == character
    }

    fn follow(&self, configuration: &PDAConfiguration) -> PDAConfiguration {
        PDAConfiguration {
            state: self.next_state,
            stack: self.next_stack(configuration),
        }
    }

    fn next_stack(&self, configuration: &PDAConfiguration) -> Stack {
        let mut popped_stack = configuration.stack.pop();
        let mut push_characters = self.push_characters.clone();
        push_characters.reverse();
        for c in push_characters {
            popped_stack = popped_stack.push(c);
        }
        popped_stack
    }
}

#[derive(Clone)]
struct DPDARuleBook {
    rules: Vec<PDARule>,
}

impl DPDARuleBook {
    fn next_configuration(
        &self,
        configuration: &PDAConfiguration,
        character: char,
    ) -> PDAConfiguration {
        self.rule_for(configuration, character)
            .expect("no rule for this configuration")
            .follow(configuration)
    }

    fn applies_to(&self, configuration: &PDAConfiguration, character: char) -> bool {
        self.rule_for(configuration, character).is_some()
    }

    fn rule_for(&self, configuration: &PDAConfiguration, character: char) -> Option<&PDARule> {
        self.rules
            .iter()
            .find(|rule| rule.clone().applies_to(configuration.clone(), character))
    }

    fn follow_free_moves(&self, configuration: &PDAConfiguration) -> PDAConfiguration {
        if self.applies_to(configuration, NIL) {
            self.follow_free_moves(&self.next_configuration(configuration, NIL))
        } else {
            configuration.clone()
        }
    }
}

#[derive(Clone)]
    struct DPDA {
    current_configuration: PDAConfiguration,
    accept_states: Vec<State>,
    rulebook: DPDARuleBook,
}

impl DPDA {
    fn next_configuration(&self, character: char) -> PDAConfiguration {
        let current_configuration = self.get_current_configuration();
        if self.rulebook.applies_to(&current_configuration, character) {
            self.rulebook
                .next_configuration(&current_configuration, character)
        } else {
            current_configuration.stuck()
        }
    }

    fn accepting(&self) -> bool {
        self.accept_states
            .contains(&self.get_current_configuration().state)
    }

    fn is_stuck(&self) -> bool {
        self.get_current_configuration().is_stuck()
    }

    fn read_character(&mut self, character: char) {
        self.current_configuration = self.next_configuration(character);
    }

    fn read_string(&mut self, string: String) {
        for c in string.chars().into_iter() {
            self.read_character(c);
            if self.get_current_configuration().is_stuck() {
                break;
            }
        }
    }

    fn get_current_configuration(&self) -> PDAConfiguration {
        self.rulebook.follow_free_moves(&self.current_configuration)
    }
}

struct DPDADesign {
    start_state: State,
    bottom_character: char,
    accept_states: Vec<State>,
    rulebook: DPDARuleBook,
}

impl DPDADesign {
    fn accepts(&self, string: String) -> bool {
        let mut dpda = self.to_dpda();
        dpda.read_string(string);
        dpda.accepting()
    }

    fn to_dpda(&self) -> DPDA {
        let start_configuration = PDAConfiguration {
            state: self.start_state,
            stack: Stack::new(vec![self.bottom_character]),
        };
        DPDA {
            current_configuration: start_configuration,
            accept_states: self.accept_states.clone(),
            rulebook: self.rulebook.clone(),
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::{DPDADesign, DPDARuleBook, PDAConfiguration, PDARule, Stack, DPDA, NIL};

    #[test]
    fn test_stack() {
        let mut contents = vec!['a', 'b', 'c', 'd', 'e'];
        contents.reverse();
        let stack = Stack { contents };
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
            push_characters: vec!['b', '$'],
        };
        let configuration = PDAConfiguration {
            state: 1,
            stack: Stack::new(vec!['$']),
        };
        assert!(rule.applies_to(configuration, '('));
    }

    #[test]
    fn test_dpdarulebook() {
        let rulebook = DPDARuleBook {
            rules: vec![
                PDARule {
                    state: 1,
                    character: '(',
                    next_state: 2,
                    pop_character: '$',
                    push_characters: vec!['b', '$'],
                },
                PDARule {
                    state: 2,
                    character: '(',
                    next_state: 2,
                    pop_character: 'b',
                    push_characters: vec!['b', 'b'],
                },
                PDARule {
                    state: 2,
                    character: ')',
                    next_state: 2,
                    pop_character: 'b',
                    push_characters: vec![],
                },
                PDARule {
                    state: 2,
                    character: NIL,
                    next_state: 1,
                    pop_character: '$',
                    push_characters: vec!['$'],
                },
            ],
        };
        let mut configuration = PDAConfiguration {
            state: 1,
            stack: Stack::new(vec!['$']),
        };
        configuration = rulebook.next_configuration(&configuration, '(');
        assert_eq!(
            configuration,
            PDAConfiguration {
                state: 2,
                stack: Stack::new(vec!['b', '$'])
            }
        );
        configuration = rulebook.next_configuration(&configuration, '(');
        assert_eq!(
            configuration,
            PDAConfiguration {
                state: 2,
                stack: Stack::new(vec!['b', 'b', '$'])
            }
        );
        configuration = rulebook.next_configuration(&configuration, ')');
        assert_eq!(
            configuration,
            PDAConfiguration {
                state: 2,
                stack: Stack::new(vec!['b', '$'])
            }
        );

        let configuration = PDAConfiguration {
            state: 2,
            stack: Stack::new(vec!['$']),
        };
        assert_eq!(
            rulebook.follow_free_moves(&configuration),
            PDAConfiguration {
                state: 1,
                stack: Stack::new(vec!['$'])
            }
        );

        let mut dpda = DPDA {
            current_configuration: PDAConfiguration {
                state: 1,
                stack: Stack::new(vec!['$']),
            },
            accept_states: vec![1],
            rulebook: rulebook.clone(),
        };
        assert!(dpda.clone().accepting());
        dpda.read_string(String::from("(()("));
        assert!(!dpda.clone().accepting());
        assert_eq!(
            dpda.get_current_configuration(),
            PDAConfiguration {
                state: 2,
                stack: Stack::new(vec!['b', 'b', '$'])
            }
        );

        dpda.read_string(String::from("))()"));
        assert!(dpda.clone().accepting());
        assert_eq!(
            dpda.get_current_configuration(),
            PDAConfiguration {
                state: 1,
                stack: Stack::new(vec!['$'])
            }
        );
    }

    #[test]
    fn test_dpdadesign() {
        let rulebook = DPDARuleBook {
            rules: vec![
                PDARule {
                    state: 1,
                    character: '(',
                    next_state: 2,
                    pop_character: '$',
                    push_characters: vec!['b', '$'],
                },
                PDARule {
                    state: 2,
                    character: '(',
                    next_state: 2,
                    pop_character: 'b',
                    push_characters: vec!['b', 'b'],
                },
                PDARule {
                    state: 2,
                    character: ')',
                    next_state: 2,
                    pop_character: 'b',
                    push_characters: vec![],
                },
                PDARule {
                    state: 2,
                    character: NIL,
                    next_state: 1,
                    pop_character: '$',
                    push_characters: vec!['$'],
                },
            ],
        };
        let dpda_design = DPDADesign {
            start_state: 1,
            bottom_character: '$',
            accept_states: vec![1],
            rulebook,
        };
        assert!(dpda_design.accepts(String::from("(((((((((())))))))))")));
        assert!(dpda_design.accepts(String::from("()(())((()))(()(()))")));
        assert!(!dpda_design.accepts(String::from("(()(()(()()(()()))()")));
    }

    #[test]
    fn test_dpda_stuck() {
        let rulebook = DPDARuleBook {
            rules: vec![
                PDARule {
                    state: 1,
                    character: '(',
                    next_state: 2,
                    pop_character: '$',
                    push_characters: vec!['b', '$'],
                },
                PDARule {
                    state: 2,
                    character: '(',
                    next_state: 2,
                    pop_character: 'b',
                    push_characters: vec!['b', 'b'],
                },
                PDARule {
                    state: 2,
                    character: ')',
                    next_state: 2,
                    pop_character: 'b',
                    push_characters: vec![],
                },
                PDARule {
                    state: 2,
                    character: NIL,
                    next_state: 1,
                    pop_character: '$',
                    push_characters: vec!['$'],
                },
            ],
        };
        let mut dpda = DPDA {
            current_configuration: PDAConfiguration {
                state: 1,
                stack: Stack::new(vec!['$']),
            },
            accept_states: vec![1],
            rulebook: rulebook.clone(),
        };
        dpda.read_string(String::from("())"));
        assert_eq!(
            dpda.get_current_configuration(),
            PDAConfiguration {
                state: PDAConfiguration::STUCK_STATE,
                stack: Stack::new(vec!['$'])
            }
        );
        assert!(!dpda.accepting());
        assert!(dpda.is_stuck());

        let dpda_design = DPDADesign {
            start_state: 1,
            bottom_character: '$',
            accept_states: vec![1],
            rulebook: rulebook.clone(),
        };
        assert!(!dpda_design.accepts(String::from("())")))
    }

    #[test]
    fn test_same_chars() {
        let rulebook = DPDARuleBook {
            rules: vec![
                PDARule {
                    state: 1,
                    character: 'a',
                    next_state: 2,
                    pop_character: '$',
                    push_characters: vec!['a', '$'],
                },
                PDARule {
                    state: 1,
                    character: 'b',
                    next_state: 2,
                    pop_character: '$',
                    push_characters: vec!['b', '$'],
                },
                PDARule {
                    state: 2,
                    character: 'a',
                    next_state: 2,
                    pop_character: 'a',
                    push_characters: vec!['a', 'a'],
                },
                PDARule {
                    state: 2,
                    character: 'b',
                    next_state: 2,
                    pop_character: 'b',
                    push_characters: vec!['b', 'b'],
                },
                PDARule {
                    state: 2,
                    character: 'a',
                    next_state: 2,
                    pop_character: 'b',
                    push_characters: vec![],
                },
                PDARule {
                    state: 2,
                    character: 'b',
                    next_state: 2,
                    pop_character: 'a',
                    push_characters: vec![],
                },
                PDARule {
                    state: 2,
                    character: NIL,
                    next_state: 1,
                    pop_character: '$',
                    push_characters: vec!['$'],
                },
            ]
        };
        let dpda_design = DPDADesign {
            start_state: 1,
            bottom_character: '$',
            accept_states: vec![1],
            rulebook
        };
        assert!(dpda_design.accepts(String::from("ababab")));
        assert!(dpda_design.accepts(String::from("bbbaaaab")));
        assert!(!dpda_design.accepts(String::from("baa")));
    }

    #[test]
    fn test_palindrome() {
        let rulebook = DPDARuleBook {
            rules: vec![
                PDARule {
                    state: 1,
                    character: 'a',
                    next_state: 1,
                    pop_character: '$',
                    push_characters: vec!['a', '$'],
                },
                PDARule {
                    state: 1,
                    character: 'a',
                    next_state: 1,
                    pop_character: 'a',
                    push_characters: vec!['a', 'a'],
                },
                PDARule {
                    state: 1,
                    character: 'a',
                    next_state: 1,
                    pop_character: 'b',
                    push_characters: vec!['a', 'b'],
                },
                PDARule {
                    state: 1,
                    character: 'b',
                    next_state: 1,
                    pop_character: '$',
                    push_characters: vec!['b', '$'],
                },
                PDARule {
                    state: 1,
                    character: 'b',
                    next_state: 1,
                    pop_character: 'a',
                    push_characters: vec!['b', 'a'],
                },
                PDARule {
                    state: 1,
                    character: 'b',
                    next_state: 1,
                    pop_character: 'b',
                    push_characters: vec!['b', 'b'],
                },
                PDARule {
                    state: 1,
                    character: 'm',
                    next_state: 2,
                    pop_character: '$',
                    push_characters: vec!['$'],
                },
                PDARule {
                    state: 1,
                    character: 'm',
                    next_state: 2,
                    pop_character: 'a',
                    push_characters: vec!['a'],
                },
                PDARule {
                    state: 1,
                    character: 'm',
                    next_state: 2,
                    pop_character: 'b',
                    push_characters: vec!['b'],
                },
                PDARule {
                    state: 2,
                    character: 'a',
                    next_state: 2,
                    pop_character: 'a',
                    push_characters: vec![],
                },
                PDARule {
                    state: 2,
                    character: 'b',
                    next_state: 2,
                    pop_character: 'b',
                    push_characters: vec![],
                },
                PDARule {
                    state: 2,
                    character: NIL,
                    next_state: 3,
                    pop_character: '$',
                    push_characters: vec!['$'],
                },
            ]
        };
        let dpda_design = DPDADesign {
            start_state: 1,
            bottom_character: '$',
            accept_states: vec![3],
            rulebook
        };
        assert!(dpda_design.accepts(String::from("abmba")));
        assert!(dpda_design.accepts(String::from("babbamabbab")));
        assert!(!dpda_design.accepts(String::from("abmb")));
        assert!(!dpda_design.accepts(String::from("baambaa")));
    }
}
