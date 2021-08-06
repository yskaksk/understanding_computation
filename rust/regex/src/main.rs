use std::collections::HashSet;
use rand::Rng;

#[derive(Clone, Debug, PartialEq)]
struct FARule {
    state: i32,
    character: char,
    next_state: i32,
}

impl FARule {
    fn new(state: i32, character: char, next_state: i32) -> Self {
        FARule {state, character, next_state}
    }

    fn inspect(&self) -> String {
        format!(
            "#<FARule #{} --#{}--> #{}>",
            self.state.to_string(),
            self.character.to_string(),
            self.next_state.to_string()
        )
    }

    fn follow(&self) -> i32 {
        self.next_state
    }

    fn applies_to(&self, state: i32, character: char) -> bool {
        ((self.state == state) && (self.character == character)) || ((self.state == state) && self.character == '\u{029e}')
    }
}

#[derive(Clone, Debug, PartialEq)]
struct NFARuleBook {
    rules: Vec<FARule>
}

impl NFARuleBook {
    fn next_states(&self, states: &HashSet<i32>, character: char) -> HashSet<i32> {
        let mut r = HashSet::new();
        for s in states.iter() {
            for ss in self.follow_rules_for(*s, character).iter() {
                r.insert(*ss);
            }
        }
        return r
    }

    fn follow_rules_for(&self, state: i32, character: char) -> Vec<i32> {
        self.rules_for(state, character).iter().map(|fr| fr.follow()).collect()
    }

    fn follow_free_moves(&self, states: &HashSet<i32>) -> HashSet<i32> {
        let more_states = self.next_states(&states, '\u{029e}');
        if more_states.is_subset(&states) {
            return states.clone()
        } else {
            let new_states: HashSet<i32> = states.union(&more_states).into_iter().map(|e| *e).collect();
            return self.follow_free_moves(&new_states)
        }
    }

    fn rules_for(&self, state: i32, character: char) -> Vec<&FARule> {
        let filtered: Vec<&FARule> = self
            .rules
            .iter()
            .filter(|fr| fr.applies_to(state, character))
            .collect();
        return filtered
    }
}

#[derive(Clone, Debug, PartialEq)]
struct NFA {
    current_states: HashSet<i32>,
    accept_states: HashSet<i32>,
    rulebook: NFARuleBook
}

impl NFA {
    fn accepting(&self) -> bool {
        let cs = self.get_current_states();
        !cs.is_disjoint(&self.accept_states)
    }

    fn read_character(&mut self, character: char) {
        let cs = self.get_current_states();
        self.current_states = self.rulebook.next_states(&cs, character)
    }

    fn read_string(&mut self, string: String) -> Self {
        let chars: Vec<char> = string.chars().collect();
        for char in chars {
            self.read_character(char)
        }
        self.clone()
    }

    fn get_current_states(&self) -> HashSet<i32> {
        self.rulebook.follow_free_moves(&self.current_states)
    }
}

struct NFADesign {
    start_state: i32,
    accept_states: HashSet<i32>,
    rulebook: NFARuleBook,
}

impl NFADesign {
    fn to_nfa(&self) -> NFA {
        let mut cs = HashSet::new();
        cs.insert(self.start_state);
        NFA {
            current_states: cs,
            accept_states: self.accept_states.clone(),
            rulebook: self.rulebook.clone()
        }
    }

    fn accepts(&self, string: String) -> bool {
        let mut nfa = self.to_nfa();
        nfa.read_string(string).accepting()
    }
}

enum Pattern {
    Empty,
    Literal {
        character: char
    },
    Concatenate {
        first: Box<Pattern>,
        second: Box<Pattern>,
    },
    Choose {
        first: Box<Pattern>,
        second: Box<Pattern>,
    },
    Repeat(Box<Pattern>)
}

use Pattern::*;

impl Pattern {
    fn precedence(&self) -> i32 {
        match self {
            Empty => 3,
            Literal{character:_} => 3,
            Concatenate{first:_, second:_} => 3,
            Choose{first: _, second:_} => 0,
            Repeat(_) => 2
        }
    }

    fn to_s(&self) -> String {
        match self {
            Empty => String::from(""),
            Literal{character} => character.to_string(),
            Concatenate{first, second} => {
                format!("{}{}", &*first.bracket(self.precedence()), &*second.bracket(self.precedence()))
            },
            Choose{first, second} => {
                format!("{}|{}", &*first.bracket(self.precedence()), &*second.bracket(self.precedence()))
            },
            Repeat(pat) => {
                format!("{}*", &*pat.bracket(self.precedence()))
            }
        }
    }

    fn bracket(&self, outer_precedence: i32) -> String {
        if self.precedence() < outer_precedence {
            format!("({})", self.to_s())
        } else {
            self.to_s()
        }
    }

    fn to_nfa_design(&self) -> NFADesign {
        match self {
            Empty => {
                let start_state: i32 = rand::random();
                let mut accept_states = HashSet::new();
                accept_states.insert(start_state);
                let rules: Vec<FARule> = vec![];
                let rulebook = NFARuleBook {
                    rules
                };
                return NFADesign {
                    start_state,
                    accept_states,
                    rulebook
                }
            },
            Literal{character} => {
                let start_state: i32 = rand::random();
                let accept_state: i32 = rand::random();
                let rule = FARule::new(start_state, *character, accept_state);
                let rulebook = NFARuleBook {
                    rules: vec![rule]
                };
                let mut accept_states = HashSet::new();
                accept_states.insert(accept_state);
                return NFADesign {
                    start_state,
                    accept_states,
                    rulebook
                }
            },
            Concatenate {first, second} => {
                let first_nfa_design = first.to_nfa_design();
                let second_nfa_design = second.to_nfa_design();
                let start_state = first_nfa_design.start_state;
                let accept_states = second_nfa_design.accept_states;
                let rules = 
            }
            _ => unimplemented!()
        }
    }
}

fn main() {
    let pattern = Repeat(Box::new(
            Choose {
                first: Box::new(Concatenate {
                    first: Box::new(Literal {
                        character: 'a'
                    }),
                    second: Box::new(Literal {
                        character: 'b'
                    })
                }),
                second: Box::new(Literal {
                    character: 'a'
                })
            }
    ));
    println!("{}", pattern.to_s());

    let nfa_design = Empty.to_nfa_design();
    println!("nfa_design.accept(''): {}", nfa_design.accepts(String::from("")));
    println!("nfa_design.accept('a'): {}", nfa_design.accepts(String::from("a")));

    let lit = Literal {
        character: 'a'
    };
    let nfa_design = lit.to_nfa_design();
    println!("nfa_design.accept(''): {}", nfa_design.accepts(String::from("")));
    println!("nfa_design.accept('a'): {}", nfa_design.accepts(String::from("a")));
    println!("nfa_design.accept('b'): {}", nfa_design.accepts(String::from("b")));
}
