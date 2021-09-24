use std::collections::{BTreeSet, HashSet};
use std::hash::Hash;

pub trait State {
    fn to_string(&self) -> String;
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StateInt {
    value: i32,
}

impl StateInt {
    pub fn new(value: i32) -> Self {
        StateInt { value }
    }
}

impl State for StateInt {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct StateSet {
    values: BTreeSet<i32>,
}

impl StateSet {
    fn new(values: Vec<i32>) -> Self {
        let mut bt = BTreeSet::new();
        for v in values {
            bt.insert(v);
        }
        return StateSet { values: bt };
    }
    fn from_hashset(states: HashSet<StateInt>) -> Self {
        let values = states.into_iter().map(|s| s.value).collect();
        return StateSet::new(values);
    }
    fn to_hashset(&self) -> HashSet<StateInt> {
        let mut r = HashSet::new();
        for v in self.values.clone().into_iter() {
            r.insert(StateInt::new(v));
        }
        return r;
    }
}

impl State for StateSet {
    fn to_string(&self) -> String {
        let mut s = String::from("");
        s += "(";
        for v in self.values.iter() {
            s += &format!("{}, ", v.to_string());
        }
        s += ")";
        return s;
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FARule<S: State> {
    state: S,
    character: char,
    next_state: S,
}

#[allow(dead_code)]
impl<S: State + Eq + Clone> FARule<S> {
    pub fn new(state: S, character: char, next_state: S) -> Self {
        FARule {
            state,
            character,
            next_state,
        }
    }

    fn from_hashset(
        state: HashSet<StateInt>,
        character: char,
        next_state: HashSet<StateInt>,
    ) -> FARule<StateSet> {
        FARule {
            state: StateSet::from_hashset(state),
            character,
            next_state: StateSet::from_hashset(next_state),
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

    fn follow(&self) -> S {
        self.next_state.clone()
    }

    fn applies_to(&self, state: &S, character: char) -> bool {
        (&self.state == state) && (self.character == character)
    }
}

#[derive(Clone)]
struct DFARuleBook<S: State> {
    rules: Vec<FARule<S>>,
}

impl<S: State + Eq + Clone> DFARuleBook<S> {
    fn next_state(&self, state: S, character: char) -> S {
        self.rule_for(state, character).follow()
    }
    fn rule_for(&self, state: S, character: char) -> FARule<S> {
        let rules = self.rules.clone();
        let rules: Vec<FARule<S>> = rules
            .into_iter()
            .filter(|fr| fr.applies_to(&state, character))
            .collect();
        if rules.len() > 1 {
            panic!("more than two rules are detected :(");
        }
        return rules[0].clone();
    }
}

#[derive(Clone)]
struct DFA<S: State> {
    current_state: S,
    accept_states: HashSet<S>,
    rulebook: DFARuleBook<S>,
}

impl<S: State + Eq + Clone> DFA<S> {
    fn accepting(&self) -> bool {
        for s in self.accept_states.iter() {
            if *s == self.current_state {
                return true;
            }
        }
        return false;
    }

    fn read_character(&mut self, character: char) {
        self.current_state = self
            .rulebook
            .next_state(self.current_state.clone(), character)
    }

    fn read_string(&mut self, string: String) -> Self {
        let chars: Vec<char> = string.chars().collect();
        for char in chars {
            self.read_character(char);
        }
        return self.clone();
    }
}

struct DFADesign<S: State> {
    start_state: S,
    accept_states: HashSet<S>,
    rulebook: DFARuleBook<S>,
}

impl<S: State + Clone + Eq> DFADesign<S> {
    fn to_dfa(&self) -> DFA<S> {
        return DFA {
            current_state: self.start_state.clone(),
            accept_states: self.accept_states.clone(),
            rulebook: self.rulebook.clone(),
        };
    }

    fn accepts(&self, string: String) -> bool {
        let mut dfa = self.to_dfa();
        dfa.read_string(string).accepting()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NFARuleBook<S: State> {
    pub rules: Vec<FARule<S>>,
}

impl<S: State + Eq + Clone + Hash> NFARuleBook<S> {
    fn next_states(&self, states: &HashSet<S>, character: char) -> HashSet<S> {
        let mut r = HashSet::new();
        for state in states.into_iter() {
            for next_state in self.follow_rules_for(state, character).into_iter() {
                r.insert(next_state);
            }
        }
        return r;
    }

    fn follow_rules_for(&self, state: &S, character: char) -> Vec<S> {
        self.rules_for(state, character)
            .iter()
            .map(|fr| fr.follow())
            .collect()
    }

    fn follow_free_moves(&self, states: &HashSet<S>) -> HashSet<S> {
        let more_states = self.next_states(states, '\u{029e}');
        if more_states.is_subset(states) {
            return states.clone();
        } else {
            let new_states: HashSet<S> = states
                .union(&more_states)
                .into_iter()
                .map(|e| e.clone())
                .collect();
            return self.follow_free_moves(&new_states);
        }
    }

    fn rules_for(&self, state: &S, character: char) -> Vec<&FARule<S>> {
        let filtered: Vec<&FARule<S>> = self
            .rules
            .iter()
            .filter(|fr| fr.applies_to(state, character))
            .collect();
        return filtered;
    }

    fn alphabet(&self) -> Vec<char> {
        let mut uniq: Vec<char> = self
            .rules
            .iter()
            .map(|fr| fr.character)
            .filter(|&c| c != '\u{029e}')
            .collect();
        uniq.sort();
        uniq.dedup();
        return uniq;
    }
}

#[derive(Clone, Debug, PartialEq)]
struct NFA<S: State + Eq + Clone + Hash> {
    current_states: HashSet<S>,
    accept_states: HashSet<S>,
    rulebook: NFARuleBook<S>,
}

impl<S: State + Eq + Clone + Hash> NFA<S> {
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

    fn get_current_states(&self) -> HashSet<S> {
        self.rulebook.follow_free_moves(&self.current_states)
    }
}

#[derive(Debug)]
pub struct NFADesign<S: State + Eq + Clone + Hash> {
    pub start_state: S,
    pub accept_states: HashSet<S>,
    pub rulebook: NFARuleBook<S>,
}

impl<S: State + Eq + Clone + Hash> NFADesign<S> {
    fn to_nfa(&self, current_states: HashSet<S>) -> NFA<S> {
        NFA {
            current_states,
            accept_states: self.accept_states.clone(),
            rulebook: self.rulebook.clone(),
        }
    }

    fn to_nfa_init(&self) -> NFA<S> {
        let mut cs = HashSet::new();
        cs.insert(self.start_state.clone());
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
    nfa_design: NFADesign<StateInt>,
}

impl NFASimulation {
    fn next_state(&self, state: StateSet, character: char) -> StateSet {
        let mut nfa = self.nfa_design.to_nfa(state.to_hashset());
        nfa.read_character(character);
        return StateSet::from_hashset(nfa.get_current_states());
    }

    fn rules_for(&self, state: StateSet) -> HashSet<FARule<StateSet>> {
        let mut r = HashSet::new();
        for c in self.nfa_design.rulebook.alphabet().into_iter() {
            r.insert(FARule {
                state: state.clone(),
                character: c,
                next_state: self.next_state(state.clone(), c),
            });
        }
        return r;
    }

    fn discover_states_and_rules(
        &self,
        states: HashSet<StateSet>,
    ) -> (HashSet<StateSet>, HashSet<FARule<StateSet>>) {
        let mut rules = HashSet::new();
        for state in states.clone().into_iter() {
            rules.extend(self.rules_for(state));
        }
        let mut more_states = HashSet::new();
        for rule in rules.iter() {
            more_states.insert(rule.follow());
        }
        if more_states.is_subset(&states) {
            return (states, rules);
        } else {
            let mut new_states = HashSet::new();
            new_states.extend(states);
            new_states.extend(more_states);
            return self.discover_states_and_rules(new_states);
        }
    }

    fn to_dfa_design(&self) -> DFADesign<StateSet> {
        let start_state =
            StateSet::from_hashset(self.nfa_design.to_nfa_init().get_current_states());
        let mut initial_state = HashSet::new();
        initial_state.insert(start_state.clone());
        let (states, rules) = self.discover_states_and_rules(initial_state);
        let mut r = vec![];
        for s in rules.into_iter() {
            r.push(s);
        }
        let accept_states = states
            .clone()
            .into_iter()
            .filter(|s| self.nfa_design.to_nfa(s.to_hashset()).accepting())
            .collect();

        return DFADesign {
            start_state,
            accept_states,
            rulebook: DFARuleBook { rules: r },
        };
    }
}

#[cfg(test)]
mod tests {
    use super::{DFARuleBook, FARule, NFARuleBook};
    use super::{NFADesign, NFASimulation, NFA};
    use super::{State, StateInt, StateSet};
    use std::collections::HashSet;

    #[test]
    fn test_stateint() {
        let si = StateInt { value: 10 };
        assert_eq!(si.to_string(), String::from("10"));
    }

    #[test]
    fn test_dfarulebook_int() {
        let rulebook = DFARuleBook {
            rules: vec![
                FARule::new(StateInt::new(1), 'a', StateInt::new(2)),
                FARule::new(StateInt::new(1), 'b', StateInt::new(1)),
                FARule::new(StateInt::new(2), 'a', StateInt::new(2)),
                FARule::new(StateInt::new(2), 'b', StateInt::new(3)),
                FARule::new(StateInt::new(3), 'a', StateInt::new(3)),
                FARule::new(StateInt::new(3), 'b', StateInt::new(3)),
            ],
        };
        assert_eq!(rulebook.next_state(StateInt::new(1), 'a'), StateInt::new(2));
        assert_eq!(rulebook.next_state(StateInt::new(1), 'b'), StateInt::new(1));
        assert_eq!(rulebook.next_state(StateInt::new(2), 'b'), StateInt::new(3));
    }

    #[test]
    fn test_dfarulebook_set() {
        let rulebook = DFARuleBook {
            rules: vec![
                FARule::new(StateSet::new(vec![1, 2]), 'a', StateSet::new(vec![2, 3])),
                FARule::new(StateSet::new(vec![1, 2]), 'b', StateSet::new(vec![1, 2])),
                FARule::new(StateSet::new(vec![2, 3]), 'a', StateSet::new(vec![2, 3])),
                FARule::new(StateSet::new(vec![2, 3]), 'b', StateSet::new(vec![3, 4])),
                FARule::new(StateSet::new(vec![3, 4]), 'a', StateSet::new(vec![3, 4])),
                FARule::new(StateSet::new(vec![3, 4]), 'b', StateSet::new(vec![3, 4])),
            ],
        };
        assert_eq!(
            rulebook.next_state(StateSet::new(vec![1, 2]), 'a'),
            StateSet::new(vec![2, 3])
        );
        assert_eq!(
            rulebook.next_state(StateSet::new(vec![1, 2]), 'b'),
            StateSet::new(vec![1, 2])
        );
        assert_eq!(
            rulebook.next_state(StateSet::new(vec![2, 3]), 'b'),
            StateSet::new(vec![3, 4])
        );
    }

    #[test]
    fn test_nfarulebook_int() {
        let rulebook = NFARuleBook {
            rules: vec![
                FARule::new(StateInt::new(1), 'a', StateInt::new(1)),
                FARule::new(StateInt::new(1), 'b', StateInt::new(1)),
                FARule::new(StateInt::new(1), 'b', StateInt::new(2)),
                FARule::new(StateInt::new(2), 'a', StateInt::new(3)),
                FARule::new(StateInt::new(2), 'b', StateInt::new(3)),
                FARule::new(StateInt::new(3), 'a', StateInt::new(4)),
                FARule::new(StateInt::new(3), 'b', StateInt::new(4)),
            ],
        };
        let mut s = HashSet::new();
        s.insert(StateInt::new(1));
        let mut r = HashSet::new();
        r.insert(StateInt::new(1));
        r.insert(StateInt::new(2));
        assert_eq!(rulebook.next_states(&s, 'b'), r);

        let mut s = HashSet::new();
        s.insert(StateInt::new(1));
        s.insert(StateInt::new(2));
        let mut r = HashSet::new();
        r.insert(StateInt::new(1));
        r.insert(StateInt::new(3));
        assert_eq!(rulebook.next_states(&s, 'a'), r);

        let mut s = HashSet::new();
        s.insert(StateInt::new(1));
        s.insert(StateInt::new(3));
        let mut r = HashSet::new();
        r.insert(StateInt::new(1));
        r.insert(StateInt::new(2));
        r.insert(StateInt::new(4));
        assert_eq!(rulebook.next_states(&s, 'b'), r);
    }

    #[test]
    fn test_nfarulebook_set() {
        let rulebook = NFARuleBook {
            rules: vec![
                FARule::new(StateSet::new(vec![1, 2]), 'a', StateSet::new(vec![1, 2])),
                FARule::new(StateSet::new(vec![1, 2]), 'b', StateSet::new(vec![1, 2])),
                FARule::new(StateSet::new(vec![1, 2]), 'b', StateSet::new(vec![2, 3])),
                FARule::new(StateSet::new(vec![2, 3]), 'a', StateSet::new(vec![3, 4])),
                FARule::new(StateSet::new(vec![2, 3]), 'b', StateSet::new(vec![3, 4])),
                FARule::new(StateSet::new(vec![3, 4]), 'a', StateSet::new(vec![4, 5])),
                FARule::new(StateSet::new(vec![3, 4]), 'b', StateSet::new(vec![4, 5])),
            ],
        };
        let mut s = HashSet::new();
        s.insert(StateSet::new(vec![1, 2]));
        let mut r = HashSet::new();
        r.insert(StateSet::new(vec![1, 2]));
        r.insert(StateSet::new(vec![2, 3]));
        assert_eq!(rulebook.next_states(&s, 'b'), r);

        let mut s = HashSet::new();
        s.insert(StateSet::new(vec![1, 2]));
        s.insert(StateSet::new(vec![2, 3]));
        let mut r = HashSet::new();
        r.insert(StateSet::new(vec![1, 2]));
        r.insert(StateSet::new(vec![3, 4]));
        assert_eq!(rulebook.next_states(&s, 'a'), r);

        let mut s = HashSet::new();
        s.insert(StateSet::new(vec![1, 2]));
        s.insert(StateSet::new(vec![3, 4]));
        let mut r = HashSet::new();
        r.insert(StateSet::new(vec![1, 2]));
        r.insert(StateSet::new(vec![2, 3]));
        r.insert(StateSet::new(vec![4, 5]));
        assert_eq!(rulebook.next_states(&s, 'b'), r);
    }

    #[test]
    fn test_nfa_int() {
        let mut cs = HashSet::new();
        cs.insert(StateInt::new(1));
        let mut acs = HashSet::new();
        acs.insert(StateInt::new(4));
        let rulebook = NFARuleBook {
            rules: vec![
                FARule::new(StateInt::new(1), 'a', StateInt::new(1)),
                FARule::new(StateInt::new(1), 'b', StateInt::new(1)),
                FARule::new(StateInt::new(1), 'b', StateInt::new(2)),
                FARule::new(StateInt::new(2), 'a', StateInt::new(3)),
                FARule::new(StateInt::new(2), 'b', StateInt::new(3)),
                FARule::new(StateInt::new(3), 'a', StateInt::new(4)),
                FARule::new(StateInt::new(3), 'b', StateInt::new(4)),
            ],
        };
        let mut nfa = NFA {
            current_states: cs,
            accept_states: acs,
            rulebook,
        };
        assert!(!nfa.accepting());
        nfa.read_character('b');
        assert!(!nfa.accepting());
        nfa.read_character('a');
        assert!(!nfa.accepting());
        nfa.read_character('b');
        assert!(nfa.accepting());

        let mut cs = HashSet::new();
        cs.insert(StateInt::new(1));
        let mut acs = HashSet::new();
        acs.insert(StateInt::new(4));
        let rulebook = NFARuleBook {
            rules: vec![
                FARule::new(StateInt::new(1), 'a', StateInt::new(1)),
                FARule::new(StateInt::new(1), 'b', StateInt::new(1)),
                FARule::new(StateInt::new(1), 'b', StateInt::new(2)),
                FARule::new(StateInt::new(2), 'a', StateInt::new(3)),
                FARule::new(StateInt::new(2), 'b', StateInt::new(3)),
                FARule::new(StateInt::new(3), 'a', StateInt::new(4)),
                FARule::new(StateInt::new(3), 'b', StateInt::new(4)),
            ],
        };
        let mut nfa = NFA {
            current_states: cs,
            accept_states: acs,
            rulebook,
        };
        nfa.read_string(String::from("bbbbb"));
        assert!(nfa.accepting());
    }

    #[test]
    fn test_follow_free_moves_int() {
        let rulebook = NFARuleBook {
            rules: vec![
                FARule::new(StateInt::new(1), '\u{029e}', StateInt::new(2)),
                FARule::new(StateInt::new(1), '\u{029e}', StateInt::new(4)),
                FARule::new(StateInt::new(2), 'a', StateInt::new(3)),
                FARule::new(StateInt::new(3), 'a', StateInt::new(2)),
                FARule::new(StateInt::new(4), 'a', StateInt::new(5)),
                FARule::new(StateInt::new(5), 'a', StateInt::new(6)),
                FARule::new(StateInt::new(6), 'a', StateInt::new(4)),
            ],
        };
        let mut s = HashSet::new();
        s.insert(StateInt::new(1));
        let mut r = HashSet::new();
        r.insert(StateInt::new(1));
        r.insert(StateInt::new(2));
        r.insert(StateInt::new(4));
        assert_eq!(rulebook.follow_free_moves(&s), r);

        let mut s = HashSet::new();
        s.insert(StateInt::new(1));
        s.insert(StateInt::new(5));
        let mut r = HashSet::new();
        r.insert(StateInt::new(1));
        r.insert(StateInt::new(2));
        r.insert(StateInt::new(4));
        r.insert(StateInt::new(5));
        assert_eq!(rulebook.follow_free_moves(&s), r);
    }

    #[test]
    fn test_nfa_set() {
        let mut cs = HashSet::new();
        cs.insert(StateSet::new(vec![1, 2]));
        let mut acs = HashSet::new();
        acs.insert(StateSet::new(vec![4, 5]));
        let rulebook = NFARuleBook {
            rules: vec![
                FARule::new(StateSet::new(vec![1, 2]), 'a', StateSet::new(vec![1, 2])),
                FARule::new(StateSet::new(vec![1, 2]), 'b', StateSet::new(vec![1, 2])),
                FARule::new(StateSet::new(vec![1, 2]), 'b', StateSet::new(vec![2, 3])),
                FARule::new(StateSet::new(vec![2, 3]), 'a', StateSet::new(vec![3, 4])),
                FARule::new(StateSet::new(vec![2, 3]), 'b', StateSet::new(vec![3, 4])),
                FARule::new(StateSet::new(vec![3, 4]), 'a', StateSet::new(vec![4, 5])),
                FARule::new(StateSet::new(vec![3, 4]), 'b', StateSet::new(vec![4, 5])),
            ],
        };
        let mut nfa = NFA {
            current_states: cs,
            accept_states: acs,
            rulebook,
        };
        assert!(!nfa.accepting());
        nfa.read_character('b');
        assert!(!nfa.accepting());
        nfa.read_character('a');
        assert!(!nfa.accepting());
        nfa.read_character('b');
        assert!(nfa.accepting());
    }

    #[test]
    fn test_follow_free_moves_set() {
        let rulebook = NFARuleBook {
            rules: vec![
                FARule::new(
                    StateSet::new(vec![1, 2]),
                    '\u{029e}',
                    StateSet::new(vec![2, 3]),
                ),
                FARule::new(
                    StateSet::new(vec![1, 2]),
                    '\u{029e}',
                    StateSet::new(vec![4, 5]),
                ),
                FARule::new(StateSet::new(vec![2, 3]), 'a', StateSet::new(vec![3, 4])),
                FARule::new(StateSet::new(vec![3, 4]), 'a', StateSet::new(vec![2, 3])),
                FARule::new(StateSet::new(vec![4, 5]), 'a', StateSet::new(vec![5, 6])),
                FARule::new(StateSet::new(vec![5, 6]), 'a', StateSet::new(vec![6, 7])),
                FARule::new(StateSet::new(vec![6, 7]), 'a', StateSet::new(vec![4, 5])),
            ],
        };
        let mut s = HashSet::new();
        s.insert(StateSet::new(vec![1, 2]));
        let mut r = HashSet::new();
        r.insert(StateSet::new(vec![1, 2]));
        r.insert(StateSet::new(vec![2, 3]));
        r.insert(StateSet::new(vec![4, 5]));
        assert_eq!(rulebook.follow_free_moves(&s), r);

        let mut s = HashSet::new();
        s.insert(StateSet::new(vec![1, 2]));
        s.insert(StateSet::new(vec![5, 6]));
        let mut r = HashSet::new();
        r.insert(StateSet::new(vec![1, 2]));
        r.insert(StateSet::new(vec![2, 3]));
        r.insert(StateSet::new(vec![4, 5]));
        r.insert(StateSet::new(vec![5, 6]));
        assert_eq!(rulebook.follow_free_moves(&s), r);
    }

    #[test]
    fn test_nfa_design_int() {
        let rulebook = NFARuleBook {
            rules: vec![
                FARule::new(StateInt::new(1), '\u{029e}', StateInt::new(2)),
                FARule::new(StateInt::new(1), '\u{029e}', StateInt::new(4)),
                FARule::new(StateInt::new(2), 'a', StateInt::new(3)),
                FARule::new(StateInt::new(3), 'a', StateInt::new(2)),
                FARule::new(StateInt::new(4), 'a', StateInt::new(5)),
                FARule::new(StateInt::new(5), 'a', StateInt::new(6)),
                FARule::new(StateInt::new(6), 'a', StateInt::new(4)),
            ],
        };
        let mut accept_states = HashSet::new();
        accept_states.insert(StateInt::new(2));
        accept_states.insert(StateInt::new(4));
        let nfa_design = NFADesign {
            start_state: StateInt::new(1),
            accept_states,
            rulebook,
        };
        assert!(nfa_design.accepts(String::from("aa")));
        assert!(nfa_design.accepts(String::from("aaa")));
        assert!(!nfa_design.accepts(String::from("aaaaa")));
        assert!(nfa_design.accepts(String::from("aaaaaa")));
    }

    #[test]
    fn test_nfa_design_set() {
        let rulebook = NFARuleBook {
            rules: vec![
                FARule::new(
                    StateSet::new(vec![1, 2]),
                    '\u{029e}',
                    StateSet::new(vec![2, 3]),
                ),
                FARule::new(
                    StateSet::new(vec![1, 2]),
                    '\u{029e}',
                    StateSet::new(vec![4, 5]),
                ),
                FARule::new(StateSet::new(vec![2, 3]), 'a', StateSet::new(vec![3, 4])),
                FARule::new(StateSet::new(vec![3, 4]), 'a', StateSet::new(vec![2, 3])),
                FARule::new(StateSet::new(vec![4, 5]), 'a', StateSet::new(vec![5, 6])),
                FARule::new(StateSet::new(vec![5, 6]), 'a', StateSet::new(vec![6, 7])),
                FARule::new(StateSet::new(vec![6, 7]), 'a', StateSet::new(vec![4, 5])),
            ],
        };
        let mut accept_states = HashSet::new();
        accept_states.insert(StateSet::new(vec![2, 3]));
        accept_states.insert(StateSet::new(vec![4, 5]));
        let nfa_design = NFADesign {
            start_state: StateSet::new(vec![1, 2]),
            accept_states,
            rulebook,
        };
        assert!(nfa_design.accepts(String::from("aa")));
        assert!(nfa_design.accepts(String::from("aaa")));
        assert!(!nfa_design.accepts(String::from("aaaaa")));
        assert!(nfa_design.accepts(String::from("aaaaaa")));
    }

    #[test]
    fn test_nfa_design_to_nfa() {
        let rulebook = NFARuleBook {
            rules: vec![
                FARule::new(StateInt::new(1), 'a', StateInt::new(1)),
                FARule::new(StateInt::new(1), 'a', StateInt::new(2)),
                FARule::new(StateInt::new(1), '\u{029e}', StateInt::new(2)),
                FARule::new(StateInt::new(2), 'b', StateInt::new(3)),
                FARule::new(StateInt::new(3), 'b', StateInt::new(1)),
                FARule::new(StateInt::new(3), '\u{029e}', StateInt::new(2)),
            ],
        };
        let mut accept_states = HashSet::new();
        accept_states.insert(StateInt::new(3));
        let nfa_design = NFADesign {
            start_state: StateInt::new(1),
            accept_states,
            rulebook,
        };
        let mut r = HashSet::new();
        r.insert(StateInt::new(1));
        r.insert(StateInt::new(2));
        assert_eq!(nfa_design.to_nfa_init().get_current_states(), r);

        let mut s = HashSet::new();
        s.insert(StateInt::new(2));
        let mut r = HashSet::new();
        r.insert(StateInt::new(2));
        assert_eq!(nfa_design.to_nfa(s).get_current_states(), r);

        let mut s = HashSet::new();
        s.insert(StateInt::new(3));
        let mut r = HashSet::new();
        r.insert(StateInt::new(2));
        r.insert(StateInt::new(3));
        assert_eq!(nfa_design.to_nfa(s).get_current_states(), r);
    }

    #[test]
    fn test_simulation() {
        let rulebook = NFARuleBook {
            rules: vec![
                FARule::new(StateInt::new(1), 'a', StateInt::new(1)),
                FARule::new(StateInt::new(1), 'a', StateInt::new(2)),
                FARule::new(StateInt::new(1), '\u{029e}', StateInt::new(2)),
                FARule::new(StateInt::new(2), 'b', StateInt::new(3)),
                FARule::new(StateInt::new(3), 'b', StateInt::new(1)),
                FARule::new(StateInt::new(3), '\u{029e}', StateInt::new(2)),
            ],
        };
        let mut accept_states = HashSet::new();
        accept_states.insert(StateInt::new(3));
        let nfa_design = NFADesign {
            start_state: StateInt::new(1),
            accept_states,
            rulebook,
        };
        let simulation = NFASimulation { nfa_design };
        let mut s = HashSet::new();
        s.insert(StateInt::new(1));
        s.insert(StateInt::new(2));
        let mut r = HashSet::new();
        r.insert(StateInt::new(1));
        r.insert(StateInt::new(2));
        assert_eq!(
            simulation.next_state(StateSet::from_hashset(s), 'a'),
            StateSet::from_hashset(r)
        );

        let mut s = HashSet::new();
        s.insert(StateInt::new(1));
        s.insert(StateInt::new(2));
        let mut r = HashSet::new();
        r.insert(StateInt::new(2));
        r.insert(StateInt::new(3));
        assert_eq!(
            simulation.next_state(StateSet::from_hashset(s), 'b'),
            StateSet::from_hashset(r)
        );

        let mut s = HashSet::new();
        s.insert(StateInt::new(2));
        s.insert(StateInt::new(3));
        let mut r = HashSet::new();
        r.insert(StateInt::new(1));
        r.insert(StateInt::new(2));
        r.insert(StateInt::new(3));
        assert_eq!(
            simulation.next_state(StateSet::from_hashset(s), 'b'),
            StateSet::from_hashset(r)
        );

        let mut s = HashSet::new();
        s.insert(StateInt::new(1));
        s.insert(StateInt::new(2));
        s.insert(StateInt::new(3));
        let mut r = HashSet::new();
        r.insert(StateInt::new(1));
        r.insert(StateInt::new(2));
        r.insert(StateInt::new(3));
        assert_eq!(
            simulation.next_state(StateSet::from_hashset(s), 'b'),
            StateSet::from_hashset(r)
        );

        let mut s = HashSet::new();
        s.insert(StateInt::new(1));
        s.insert(StateInt::new(2));
        s.insert(StateInt::new(3));
        let mut r = HashSet::new();
        r.insert(StateInt::new(1));
        r.insert(StateInt::new(2));
        assert_eq!(
            simulation.next_state(StateSet::from_hashset(s), 'a'),
            StateSet::from_hashset(r)
        );
    }

    #[test]
    fn test_simulation_rule_for() {
        let rulebook = NFARuleBook {
            rules: vec![
                FARule::new(StateInt::new(1), 'a', StateInt::new(1)),
                FARule::new(StateInt::new(1), 'a', StateInt::new(2)),
                FARule::new(StateInt::new(1), '\u{029e}', StateInt::new(2)),
                FARule::new(StateInt::new(2), 'b', StateInt::new(3)),
                FARule::new(StateInt::new(3), 'b', StateInt::new(1)),
                FARule::new(StateInt::new(3), '\u{029e}', StateInt::new(2)),
            ],
        };
        assert_eq!(rulebook.alphabet(), vec!['a', 'b']);
        let mut accept_states = HashSet::new();
        accept_states.insert(StateInt::new(3));
        let nfa_design = NFADesign {
            start_state: StateInt::new(1),
            accept_states,
            rulebook,
        };
        let simulation = NFASimulation { nfa_design };
        let mut s = HashSet::new();
        s.insert(StateInt::new(1));
        s.insert(StateInt::new(2));
        let mut rules = HashSet::new();
        rules.insert(FARule {
            state: StateSet::new(vec![1, 2]),
            character: 'a',
            next_state: StateSet::new(vec![1, 2]),
        });
        rules.insert(FARule {
            state: StateSet::new(vec![1, 2]),
            character: 'a',
            next_state: StateSet::new(vec![1, 2]),
        });
        rules.insert(FARule {
            state: StateSet::new(vec![1, 2]),
            character: 'b',
            next_state: StateSet::new(vec![2, 3]),
        });
        assert_eq!(simulation.rules_for(StateSet::from_hashset(s)), rules);

        let mut s = HashSet::new();
        s.insert(StateSet::new(vec![1, 2]));
        let mut expected_set = HashSet::new();
        expected_set.insert(StateSet::new(vec![]));
        expected_set.insert(StateSet::new(vec![1, 2]));
        expected_set.insert(StateSet::new(vec![2, 3]));
        expected_set.insert(StateSet::new(vec![1, 2, 3]));
        let mut rules = HashSet::new();
        rules.insert(FARule {
            state: StateSet::new(vec![1, 2]),
            character: 'a',
            next_state: StateSet::new(vec![1, 2]),
        });
        rules.insert(FARule {
            state: StateSet::new(vec![1, 2]),
            character: 'b',
            next_state: StateSet::new(vec![2, 3]),
        });
        rules.insert(FARule {
            state: StateSet::new(vec![2, 3]),
            character: 'a',
            next_state: StateSet::new(vec![]),
        });
        rules.insert(FARule {
            state: StateSet::new(vec![2, 3]),
            character: 'b',
            next_state: StateSet::new(vec![1, 2, 3]),
        });
        rules.insert(FARule {
            state: StateSet::new(vec![]),
            character: 'a',
            next_state: StateSet::new(vec![]),
        });
        rules.insert(FARule {
            state: StateSet::new(vec![]),
            character: 'b',
            next_state: StateSet::new(vec![]),
        });
        rules.insert(FARule {
            state: StateSet::new(vec![1, 2, 3]),
            character: 'a',
            next_state: StateSet::new(vec![1, 2]),
        });
        rules.insert(FARule {
            state: StateSet::new(vec![1, 2, 3]),
            character: 'b',
            next_state: StateSet::new(vec![1, 2, 3]),
        });
        assert_eq!(
            simulation.discover_states_and_rules(s),
            (expected_set, rules)
        );
    }

    #[test]
    fn test_simulation_to_dfa() {
        let rulebook = NFARuleBook {
            rules: vec![
                FARule::new(StateInt::new(1), 'a', StateInt::new(1)),
                FARule::new(StateInt::new(1), 'a', StateInt::new(2)),
                FARule::new(StateInt::new(1), '\u{029e}', StateInt::new(2)),
                FARule::new(StateInt::new(2), 'b', StateInt::new(3)),
                FARule::new(StateInt::new(3), 'b', StateInt::new(1)),
                FARule::new(StateInt::new(3), '\u{029e}', StateInt::new(2)),
            ],
        };
        assert_eq!(rulebook.alphabet(), vec!['a', 'b']);
        let mut accept_states = HashSet::new();
        accept_states.insert(StateInt::new(3));
        let nfa_design = NFADesign {
            start_state: StateInt::new(1),
            accept_states,
            rulebook,
        };
        let simulation = NFASimulation { nfa_design };
        let dfa_design = simulation.to_dfa_design();
        assert!(!dfa_design.accepts(String::from("aaa")));
        assert!(dfa_design.accepts(String::from("aab")));
        assert!(dfa_design.accepts(String::from("bbbabb")));
    }
}
