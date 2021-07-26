use std::collections::HashSet;

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
    //fn inspect(&self) -> String {
    //    format!(
    //        "#<FARule #{} --#{}--> #{}>",
    //        self.state.to_string(),
    //        self.character.to_string(),
    //        self.next_state.to_string()
    //    )
    //}

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


fn main() {
    let rulebook = NFARuleBook {
        rules: vec![
            FARule::new(1, 'a', 1), FARule::new(1, 'b', 1), FARule::new(1, 'b', 2),
            FARule::new(2, 'a', 3), FARule::new(2, 'b', 3),
            FARule::new(3, 'a', 4), FARule::new(3, 'b', 4)
        ]
    };
    let mut current = HashSet::new();
    current.insert(1);
    let mut accepting = HashSet::new();
    accepting.insert(4);
    let mut nfa = NFA {
        current_states: current,
        accept_states: accepting,
        rulebook
    };

    println!("init, accepting? : {}", nfa.accepting());
    nfa.read_character('b');
    println!("read b, accepting? : {}", nfa.accepting());
    nfa.read_character('a');
    println!("read a, accepting? : {}", nfa.accepting());
    nfa.read_character('b');
    println!("read b, accepting? : {}", nfa.accepting());

    let rulebook = NFARuleBook {
        rules: vec![
            FARule::new(1, 'a', 1), FARule::new(1, 'b', 1), FARule::new(1, 'b', 2),
            FARule::new(2, 'a', 3), FARule::new(2, 'b', 3),
            FARule::new(3, 'a', 4), FARule::new(3, 'b', 4)
        ]
    };
    let mut current = HashSet::new();
    current.insert(1);
    let mut accepting = HashSet::new();
    accepting.insert(4);
    let mut nfa = NFA {
        current_states: current,
        accept_states: accepting,
        rulebook
    };

    println!("new nfa, accepting? : {}", nfa.accepting());
    nfa.read_string(String::from("bbbbb"));
    println!("read bbbbb; accepting? : {}", nfa.accepting());
}
