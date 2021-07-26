#[derive(Clone, Debug, PartialEq)]
struct FARule {
    state: i32,
    character: char,
    next_state: i32,
}

impl FARule {
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
        (self.state == state) && (self.character == character)
    }
}

#[derive(Clone, Debug, PartialEq)]
struct DFARuleBook {
    rules: Vec<FARule>,
}

impl DFARuleBook {
    fn next_state(&self, state: i32, character: char) -> i32 {
        self.rule_for(state, character).follow()
    }

    fn rule_for(&self, state: i32, character: char) -> FARule {
        let filtered: Vec<&FARule> = self
            .rules
            .iter()
            .filter(|fr| fr.applies_to(state, character))
            .collect();
        if filtered.len() != 1 {
            unreachable!()
        } else {
            filtered[0].clone()
        }
    }

    fn inspect(&self) -> String {
        let mut s = String::from("");
        for r in self.rules.iter() {
            s += &format!("{}\n", r.inspect());
        }
        s
    }
}

#[derive(Clone, Debug, PartialEq)]
struct DFA {
    current_state: i32,
    accept_states: Vec<i32>,
    rulebook: DFARuleBook,
}

impl DFA {
    fn accepting(&self) -> bool {
        self.accept_states.contains(&self.current_state)
    }

    fn read_character(&mut self, character: char) {
        self.current_state = self.rulebook.next_state(self.current_state, character)
    }

    fn read_string(&mut self, string: String) -> Self {
        let chars: Vec<char> = string.chars().collect();
        for char in chars {
            self.read_character(char)
        }
        self.clone()
    }
}

struct DFADesign {
    start_state: i32,
    accept_states: Vec<i32>,
    rulebook: DFARuleBook,
}

impl DFADesign {
    fn to_dfa(&self) -> DFA {
        DFA {
            current_state: self.start_state,
            accept_states: self.accept_states.clone(),
            rulebook: self.rulebook.clone(),
        }
    }

    fn accepts(&self, string: String) -> bool {
        let mut dfa = self.to_dfa();
        dfa.read_string(string).accepting()
    }
}

fn main() {
    let rulebook = DFARuleBook {
        rules: vec![
            FARule {
                state: 1,
                character: 'a',
                next_state: 2,
            },
            FARule {
                state: 1,
                character: 'b',
                next_state: 1,
            },
            FARule {
                state: 2,
                character: 'a',
                next_state: 2,
            },
            FARule {
                state: 2,
                character: 'b',
                next_state: 3,
            },
            FARule {
                state: 3,
                character: 'a',
                next_state: 3,
            },
            FARule {
                state: 3,
                character: 'b',
                next_state: 3,
            },
        ],
    };
    println!("{}", rulebook.inspect());
    println!("(1, 'a') -> {}", rulebook.next_state(1, 'a'));
    println!("(1, 'b') -> {}", rulebook.next_state(1, 'b'));
    println!("(2, 'b') -> {}", rulebook.next_state(2, 'b'));

    let mut dfa = DFA {
        current_state: 1,
        accept_states: vec![3],
        rulebook: rulebook.clone(),
    };
    dfa.read_string(String::from("baaab"));
    println!("dfa.accepting() -> {}", dfa.accepting());

    let dfa_design = DFADesign {
        start_state: 1,
        accept_states: vec![3],
        rulebook: rulebook.clone(),
    };
    println!(
        "dfa_design.accepts('a') -> {}",
        dfa_design.accepts(String::from("a"))
    );
    println!(
        "dfa_design.accepts('baa') -> {}",
        dfa_design.accepts(String::from("baa"))
    );
    println!(
        "dfa_design.accepts('baba') -> {}",
        dfa_design.accepts(String::from("baba"))
    );
}
