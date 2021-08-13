use std::collections::HashSet;
use rand;

use crate::nfa::{FARule, NFARuleBook, NFADesign};

pub enum Pattern {
    Empty,
    Literal {
        character: char,
    },
    Concatenate {
        first: Box<Pattern>,
        second: Box<Pattern>,
    },
    Choose {
        first: Box<Pattern>,
        second: Box<Pattern>,
    },
    Repeat(Box<Pattern>),
}

use Pattern::*;

impl Pattern {
    pub fn matches(&self, s: String) -> bool {
        self.to_nfa_design().accepts(s)
    }

    pub fn precedence(&self) -> i32 {
        match self {
            Empty => 3,
            Literal { character: _ } => 3,
            Concatenate {
                first: _,
                second: _,
            } => 3,
            Choose {
                first: _,
                second: _,
            } => 0,
            Repeat(_) => 2,
        }
    }

    pub fn to_s(&self) -> String {
        match self {
            Empty => String::from(""),
            Literal { character } => character.to_string(),
            Concatenate { first, second } => format!(
                "{}{}",
                &*first.bracket(self.precedence()),
                &*second.bracket(self.precedence())
            ),
            Choose { first, second } => format!(
                "{}|{}",
                &*first.bracket(self.precedence()),
                &*second.bracket(self.precedence())
            ),
            Repeat(pat) => format!("{}*", &*pat.bracket(self.precedence())),
        }
    }

    fn bracket(&self, outer_precedence: i32) -> String {
        if self.precedence() < outer_precedence {
            format!("({})", self.to_s())
        } else {
            self.to_s()
        }
    }

    pub fn to_nfa_design(&self) -> NFADesign {
        match self {
            Empty => {
                let start_state: i32 = rand::random();
                let mut accept_states = HashSet::new();
                accept_states.insert(start_state);
                let rules: Vec<FARule> = vec![];
                let rulebook = NFARuleBook { rules };
                return NFADesign {
                    start_state,
                    accept_states,
                    rulebook,
                };
            }
            Literal { character } => {
                let start_state: i32 = rand::random();
                let accept_state: i32 = rand::random();
                let rule = FARule::new(start_state, *character, accept_state);
                let rulebook = NFARuleBook { rules: vec![rule] };
                let mut accept_states = HashSet::new();
                accept_states.insert(accept_state);
                return NFADesign {
                    start_state,
                    accept_states,
                    rulebook,
                };
            }
            Concatenate { first, second } => {
                let first_nfa_design = first.to_nfa_design();
                let second_nfa_design = second.to_nfa_design();
                let start_state = first_nfa_design.start_state;
                let accept_states = second_nfa_design.accept_states;
                let mut rules = first_nfa_design.rulebook.rules.clone();
                rules.append(&mut second_nfa_design.rulebook.rules.clone());
                let second_start_state: i32 = second_nfa_design.start_state.clone();
                let mut extra_rules: Vec<FARule> = first_nfa_design
                    .accept_states
                    .iter()
                    .map(|state| FARule::new(*state, '\u{029e}',second_start_state,))
                    .collect();
                rules.append(&mut extra_rules);
                let rulebook = NFARuleBook { rules: rules };
                return NFADesign {
                    start_state,
                    accept_states,
                    rulebook,
                };
            }
            Choose { first, second } => {
                let first_nfa_design = first.to_nfa_design();
                let second_nfa_design = second.to_nfa_design();

                let start_state: i32 = rand::random();
                let mut accept_states: HashSet<i32> = HashSet::new();
                accept_states.extend(first_nfa_design.accept_states.clone());
                accept_states.extend(second_nfa_design.accept_states.clone());
                let mut rules = first_nfa_design.rulebook.rules.clone();
                rules.append(&mut second_nfa_design.rulebook.rules.clone());
                let mut extra_rules: Vec<FARule> = vec![
                    FARule::new(start_state, '\u{029e}', first_nfa_design.start_state),
                    FARule::new(start_state, '\u{029e}', second_nfa_design.start_state),
                ];
                rules.append(&mut extra_rules);
                let rulebook = NFARuleBook { rules: rules };
                return NFADesign {
                    start_state,
                    accept_states,
                    rulebook,
                };
            }
            Repeat(pat) => {
                let pat_nfa_design = pat.to_nfa_design();

                let start_state: i32 = rand::random();
                let mut accept_states: HashSet<i32> = HashSet::new();
                accept_states.extend(pat_nfa_design.accept_states.clone());
                accept_states.insert(start_state);
                let mut rules = pat_nfa_design.rulebook.rules.clone();
                let mut extra_rules: Vec<FARule> = pat_nfa_design
                    .accept_states
                    .iter()
                    .map(|state| FARule::new(*state,'\u{029e}',pat_nfa_design.start_state))
                    .collect();
                rules.append(&mut extra_rules);
                rules.append(&mut vec![FARule::new(start_state,'\u{029e}',pat_nfa_design.start_state)]);
                let rulebook = NFARuleBook { rules: rules };
                return NFADesign {
                    start_state,
                    accept_states,
                    rulebook,
                };
            }
        }
    }
}
