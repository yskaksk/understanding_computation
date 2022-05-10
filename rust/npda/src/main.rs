#![allow(dead_code)]

use std::collections::HashSet;

use regex::Regex;

type State = i32;
const NIL: char = '\u{029e}';

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
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

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
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

#[derive(Clone, Eq, PartialEq, Debug)]
struct PDARule {
    state: State,
    character: char,
    next_state: State,
    pop_character: char,
    push_characters: Vec<char>,
}

impl PDARule {
    fn new(state: State, character: char, next_state: State, pop_character: char, push_characters: Vec<char>) -> Self {
        PDARule {
            state, character, next_state, pop_character, push_characters
        }
    }

    fn applies_to(&self, configuration: &PDAConfiguration, character: char) -> bool {
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

#[derive(Clone, Debug)]
struct NPDARulebook {
    rules: Vec<PDARule>
}

impl NPDARulebook {
    fn next_configurations(&self, configurations: &HashSet<PDAConfiguration>, character: char) -> HashSet<PDAConfiguration> {
        let mut r = HashSet::new();
        for c in configurations.iter() {
            for cc in self.follow_rules_for(c, character).into_iter() {
                r.insert(cc);
            }
        }
        return r
    }

    fn follow_rules_for(&self, configuration: &PDAConfiguration, character: char) -> Vec<PDAConfiguration> {
        self.rules_for(configuration, character).iter().map(|r| r.follow(configuration)).collect()
    }

    fn rules_for(&self, configuration: &PDAConfiguration, character: char) -> Vec<PDARule> {
        let rules: Vec<PDARule> = self.rules.clone().into_iter().filter(|r| r.applies_to(configuration, character)).collect();
        return rules
    }

    fn follow_free_moves(&self, configurations: &HashSet<PDAConfiguration>) -> HashSet<PDAConfiguration> {
        let more_configurations = self.next_configurations(configurations, NIL);
        if more_configurations.is_subset(configurations) {
            return configurations.clone()
        } else {
            let c: HashSet<PDAConfiguration> = configurations.union(&more_configurations).into_iter().map(|c| c.clone()).collect();
            return self.follow_free_moves(&c)
        }
    }
}

#[derive(Clone, Debug)]
struct NPDA {
    current_configurations: HashSet<PDAConfiguration>,
    accept_states: HashSet<State>,
    rulebook: NPDARulebook
}

impl NPDA {
    fn accepting(&self) -> bool {
        for c in self.get_current_configurations().iter() {
            if self.accept_states.contains(&c.state) {
                return true
            }
        }
        return false
    }

    fn read_character(&mut self, character: char) {
        self.current_configurations = self.rulebook.next_configurations(&self.get_current_configurations(), character)
    }

    fn read_string(&mut self, string: String) -> Self {
        for char in string.chars() {
            self.read_character(char)
        }
        self.clone()
    }

    fn get_current_configurations(&self) -> HashSet<PDAConfiguration> {
        self.rulebook.follow_free_moves(&self.current_configurations)
    }
}

struct NPDADesign {
    start_state: State,
    bottom_character: char,
    accept_states: HashSet<State>,
    rulebook: NPDARulebook
}

impl NPDADesign {
    fn accepts(&self, string: String) -> bool {
        let mut npda = self.to_npda();
        npda.read_string(string);
        return npda.accepting()
    }

    fn to_npda(&self) -> NPDA {
        let start_stack = Stack::new(vec![self.bottom_character]);
        let start_configuration = PDAConfiguration {
            state: self.start_state,
            stack: start_stack
        };
        let mut current_configurations = HashSet::new();
        current_configurations.insert(start_configuration);
        return NPDA {
            current_configurations,
            accept_states: self.accept_states.clone(),
            rulebook: self.rulebook.clone()
        }
    }
}

#[derive(Clone, Debug)]
struct GrammarRule {
    token: char,
    pattern: Regex
}

impl GrammarRule {
    fn new(token: char, pattern: &str) -> Self {
        GrammarRule {
            token,
            pattern: Regex::new(pattern).unwrap()
        }
    }
}

struct LexicalAnalyzer {
    code: String,
    grammar: Vec<GrammarRule>,
}

impl LexicalAnalyzer {
    fn grammars() -> Vec<GrammarRule> {
        return vec![
            GrammarRule::new('i', r"\Aif"),
            GrammarRule::new('e', r"\Aelse"),
            GrammarRule::new('w', r"\Awhile"),
            GrammarRule::new('d', r"\Ado-nothing"),
            GrammarRule::new('b', r"\Atrue"),
            //GrammarRule::new('b', r"\Afalse"),
            GrammarRule::new('(', r"\A\("),
            GrammarRule::new(')', r"\A\)"),
            GrammarRule::new('{', r"\A\{"),
            GrammarRule::new('}', r"\A\}"),
            GrammarRule::new(';', r"\A;"),
            GrammarRule::new('=', r"\A="),
            GrammarRule::new('+', r"\A\+"),
            GrammarRule::new('*', r"\A\*"),
            GrammarRule::new('<', r"\A<"),
            GrammarRule::new('n', r"\A[0-9]+"),
            GrammarRule::new('v', r"\A[a-z]+"),
        ]
    }

    fn new(code: String) -> Self {
        let grammar = Self::grammars();
        LexicalAnalyzer { code, grammar }
    }

    fn analyze(&mut self) -> Vec<char> {
        let mut tokens: Vec<char> = vec![];
        while self.more_tokens() {
            tokens.push(self.next_token());
        }
        return tokens
    }

    fn more_tokens(&self) -> bool {
        !self.code.is_empty()
    }

    fn next_token(&mut self) -> char {
        let (token, ma) = self.rule_matching(self.code.clone());
        self.code = self.string_after(ma);
        return token
    }

    fn rule_matching(&self, code: String) -> (char, String) {
        let matches = self.grammar.iter().map(|g| {
            if let Some(cap) = g.pattern.captures(&code) {
                cap.get(0).unwrap().as_str()
            } else {
                ""
            }
        });
        let rule_with_matches: Vec<_> = self.grammar.iter().zip(matches).filter(|(_, m)| !m.is_empty()).collect();
        self.rule_with_longest_match(rule_with_matches)
    }

    fn rule_with_longest_match(&self, rule_with_matches: Vec<(&GrammarRule, &str)>) -> (char, String) {
        let (g, m) = rule_with_matches.iter().max_by_key(|(_, m)| m.len()).unwrap();
        return match m {
            &"if" => ('i', "if".to_string()),
            &"else" => ('e', "else".to_string()),
            &"while" => ('w', "while".to_string()),
            &"do-nothing" => ('d', "do-nothing".to_string()),
            &"true" => ('b', "true".to_string()),
            &"false" => ('b', "false".to_string()),
            _ => (g.token, m.to_string())
        }
    }

    fn string_after(&self, ma: String) -> String {
        self.code.strip_prefix(&ma).unwrap().trim_start().to_string()
    }

}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use super::*;

    //#[test]
    //fn test_npda() {
    //    let rulebook = NPDARulebook {
    //        rules: vec![
    //            PDARule::new(1, 'a', 1, '$', vec!['a', '$']),
    //            PDARule::new(1, 'a', 1, 'a', vec!['a', 'a']),
    //            PDARule::new(1, 'a', 1, 'b', vec!['a', 'b']),
    //            PDARule::new(1, 'b', 1, '$', vec!['b', '$']),
    //            PDARule::new(1, 'b', 1, 'a', vec!['b', 'a']),
    //            PDARule::new(1, 'b', 1, 'b', vec!['b', 'b']),
    //            PDARule::new(1, NIL, 2, '$', vec!['$']),
    //            PDARule::new(1, NIL, 2, 'a', vec!['a']),
    //            PDARule::new(1, NIL, 2, 'b', vec!['b']),
    //            PDARule::new(2, 'a', 2, 'a', vec![]),
    //            PDARule::new(2, 'b', 2, 'b', vec![]),
    //            PDARule::new(2, NIL, 3, '$', vec!['$']),
    //        ]
    //    };
    //    let configuration = PDAConfiguration {
    //        state: 1,
    //        stack: Stack::new(vec!['$'])
    //    };
    //    let mut current_configurations = HashSet::new();
    //    current_configurations.insert(configuration);
    //    let mut accept_states = HashSet::new();
    //    accept_states.insert(3);

    //    let mut npda = NPDA {
    //        current_configurations,
    //        accept_states,
    //        rulebook
    //    };
    //    assert!(npda.accepting());
    //    npda.read_string(String::from("abb"));
    //    assert!(!npda.accepting());
    //    npda.read_character('a');
    //    assert!(npda.accepting());
    //}

    //#[test]
    //fn test_npdadesign() {
    //    let rulebook = NPDARulebook {
    //        rules: vec![
    //            PDARule::new(1, 'a', 1, '$', vec!['a', '$']),
    //            PDARule::new(1, 'a', 1, 'a', vec!['a', 'a']),
    //            PDARule::new(1, 'a', 1, 'b', vec!['a', 'b']),
    //            PDARule::new(1, 'b', 1, '$', vec!['b', '$']),
    //            PDARule::new(1, 'b', 1, 'a', vec!['b', 'a']),
    //            PDARule::new(1, 'b', 1, 'b', vec!['b', 'b']),
    //            PDARule::new(1, NIL, 2, '$', vec!['$']),
    //            PDARule::new(1, NIL, 2, 'a', vec!['a']),
    //            PDARule::new(1, NIL, 2, 'b', vec!['b']),
    //            PDARule::new(2, 'a', 2, 'a', vec![]),
    //            PDARule::new(2, 'b', 2, 'b', vec![]),
    //            PDARule::new(2, NIL, 3, '$', vec!['$']),
    //        ]
    //    };
    //    let mut accept_states = HashSet::new();
    //    accept_states.insert(3);
    //    let npdadesign = NPDADesign {
    //        start_state: 1,
    //        bottom_character: '$',
    //        accept_states, rulebook
    //    };
    //    assert!(npdadesign.accepts(String::from("abba")));
    //    assert!(npdadesign.accepts(String::from("babbaabbab")));
    //    assert!(!npdadesign.accepts(String::from("abb")));
    //    assert!(!npdadesign.accepts(String::from("baabaa")));
    //}

    //#[test]
    //fn test_lexicalanalyzaer() {
    //    let mut la = LexicalAnalyzer::new(String::from("y = x * 7"));
    //    assert!(la.more_tokens());
    //    assert_eq!(la.analyze(), vec!['v', '=', 'v', '*', 'n']);

    //    let mut la_while = LexicalAnalyzer::new(String::from("while (x < 5) {x = x * 3}"));
    //    assert_eq!(la_while.analyze(), vec!['w', '(', 'v', '<', 'n', ')', '{', 'v', '=', 'v', '*', 'n', '}']);

    //    let mut la_if = LexicalAnalyzer::new(String::from("if (x < 10) { y = true; x = 0 } else { do-nothing }"));
    //    assert_eq!(la_if.analyze(), vec!['i', '(', 'v', '<', 'n', ')', '{', 'v', '=', 'b', ';', 'v', '=', 'n', '}', 'e', '{', 'd', '}']);

    //    let la_empty = LexicalAnalyzer::new(String::from(""));
    //    assert!(!la_empty.more_tokens());
    //}

    #[test]
    fn test_cfg() {
        let start_rule = PDARule::new(1, NIL, 2, '$', vec!['$', '$']);
        let symbol_rules = vec![
            PDARule::new(2, NIL, 2, 'S', vec!['W']),
            PDARule::new(2, NIL, 2, 'S', vec!['A']),
            PDARule::new(2, NIL, 2, 'W', vec!['w', '(', 'E', ')', '{', 'S', '}']),
            PDARule::new(2, NIL, 2, 'A', vec!['v', '=', 'E']),
            PDARule::new(2, NIL, 2, 'E', vec!['L']),
            PDARule::new(2, NIL, 2, 'L', vec!['M', '<', 'L']),
            PDARule::new(2, NIL, 2, 'L', vec!['M']),
            PDARule::new(2, NIL, 2, 'M', vec!['T', '*', 'M']),
            PDARule::new(2, NIL, 2, 'M', vec!['T']),
            PDARule::new(2, NIL, 2, 'T', vec!['n']),
            PDARule::new(2, NIL, 2, 'T', vec!['v'])
        ];
        let token_rules: Vec<_> = LexicalAnalyzer::grammars().iter().map(|rule| PDARule::new(2, rule.token, 2, rule.token, vec![])).collect();
        let stop_rule = PDARule::new(2, NIL, 3, '$', vec!['$']);

        let mut rules: Vec<PDARule> = vec![];
        rules.push(start_rule);
        rules.extend(symbol_rules);
        rules.extend(token_rules);
        rules.push(stop_rule);
        let rulebook = NPDARulebook {
            rules
        };
        let mut accept_states = HashSet::new();
        accept_states.insert(3);
        let npda_design = NPDADesign {
            start_state: 1,
            bottom_character: '$',
            accept_states, rulebook
        };

        let token_string: String = LexicalAnalyzer::new(String::from("while (x < 5) { x = x * 3 }")).analyze().iter().collect();
        println!("token_string = {}", token_string);
        assert!(npda_design.accepts(token_string));
    }
}
