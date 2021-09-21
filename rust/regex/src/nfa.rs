use std::collections::{HashSet, BTreeSet};
use std::fmt::Display;


#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct State {
    values: BTreeSet<i32>
}

impl State {
    fn to_string(&self) -> String {
        let mut s = String::from("");
        s += "(";
        for v in self.values {
            s += &format!("{}, ", v);
        }
        s += ")";
        return s
    }

    fn merge(&self, other: &State) -> Self {
        let values: BTreeSet<i32> = self.values.union(other.values).clone();
        State {
            values: self.values.union(&other.values).clone()
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FARule {
    state: State,
    character: char,
    next_state: State,
}

#[allow(dead_code)]
impl FARule {
    pub fn new(state: State, character: char, next_state: State) -> Self {
        FARule {
            state,
            character,
            next_state,
        }
    }

    fn inspect(&self) -> String {
        format!(
            "#<FARule #{} --#{}--> #{}>",
            self.state.to_string(),
            self.character.to_string(),
            self.next_state.to_string()
        )
    }

    fn follow(&self) -> State {
        self.next_state
    }

    fn applies_to(&self, state: State, character: char) -> bool {
        (self.state == state) && (self.character == character)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NFARuleBook {
    pub rules: Vec<FARule>,
}

impl NFARuleBook {
    fn next_states(&self, states: &HashSet<State>, character: char) -> HashSet<State> {
        let mut r = HashSet::new();
        for s in states.iter() {
            for ss in self.follow_rules_for(*s, character).iter() {
                r.insert(*ss);
            }
        }
        return r;
    }

    fn follow_rules_for(&self, state: State, character: char) -> Vec<State> {
        self.rules_for(state, character)
            .iter()
            .map(|fr| fr.follow())
            .collect()
    }

    fn follow_free_moves(&self, states: &HashSet<State>) -> HashSet<State> {
        let more_states = self.next_states(&states, '\u{029e}');
        if more_states.is_subset(&states) {
            return states.clone();
        } else {
            let new_states: HashSet<State> =
                states.union(&more_states).into_iter().map(|e| *e).collect();
            return self.follow_free_moves(&new_states);
        }
    }

    fn rules_for(&self, state: State, character: char) -> Vec<&FARule> {
        let filtered: Vec<&FARule> = self
            .rules
            .iter()
            .filter(|fr| fr.applies_to(state, character))
            .collect();
        return filtered;
    }

    fn alphabet(&self) -> Vec<char> {
        let mut uniq: Vec<char> = self.rules.iter().map(|fr| fr.character).collect();
        uniq.sort();
        uniq.dedup();
        return uniq
    }
}

#[derive(Clone, Debug, PartialEq)]
struct NFA {
    current_states: HashSet<State>,
    accept_states: HashSet<State>,
    rulebook: NFARuleBook,
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

    fn get_current_states(&self) -> HashSet<State> {
        self.rulebook.follow_free_moves(&self.current_states)
    }
}

#[derive(Debug)]
pub struct NFADesign {
    pub start_state: State,
    pub accept_states: HashSet<State>,
    pub rulebook: NFARuleBook,
}

impl NFADesign {
    fn to_nfa(&self, current_states: HashSet<State>) -> NFA {
        NFA {
            current_states,
            accept_states: self.accept_states.clone(),
            rulebook: self.rulebook.clone(),
        }
    }

    fn to_nfa_init(&self) -> NFA {
        let mut cs = HashSet::new();
        cs.insert(self.start_state);
        NFA {
            current_states: cs,
            accept_states: self.accept_states.clone(),
            rulebook: self.rulebook.clone(),
        }
    }

    pub fn accepts(&self, string: String) -> bool {
        let mut nfa = self.to_nfa_init();
        nfa.read_string(string).accepting()
    }
}

struct NFASimulation {
    nfa_design: NFADesign
}

impl NFASimulation {
    fn next_state(&self, state: HashSet<State>, character: char) -> HashSet<State> {
        let mut nfa = self.nfa_design.to_nfa(state);
        nfa.read_character(character);
        return nfa.get_current_states()
    }

    fn rules_for(&self, state: HashSet<State>) -> Vec<&FARule> {
        self.nfa_design.rulebook.alphabet().map(|character| FARule::new(state, character, self.next_state(state, character)))
    }

    fn discover_states_and_rules(&self, states: Vec<HashSet<i32>>) {
    }
}
