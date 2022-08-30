#![allow(unused)]

type State = i32;

#[derive(Clone, PartialEq, Debug)]
struct Tape {
    left: Vec<char>,
    middle: char,
    right: Vec<char>,
    blank: char,
}

impl Tape {
    fn new(left: Vec<char>, middle: char, right: Vec<char>, blank: char) -> Self {
        Tape {
            left,
            middle,
            right,
            blank,
        }
    }

    fn write(&self, character: char) -> Self {
        Tape::new(self.left.clone(), character, self.right.clone(), self.blank)
    }

    fn move_head_left(&self) -> Self {
        let mut left = self.left.clone();
        let mut right = self.right.clone();
        let middle = if let Some(m) = left.pop() {
            m
        } else {
            self.blank
        };
        right.insert(0, self.middle);
        Tape::new(left, middle, right, self.blank)
    }

    fn move_head_right(&self) -> Self {
        let mut left = self.left.clone();
        let mut right = self.right.clone();
        let middle = if let Some(m) = right.first() {
            *m
        } else {
            self.blank
        };
        if right.len() > 0 {
            right.remove(0);
        }
        left.push(self.middle);
        Tape::new(left, middle, right, self.blank)
    }
}

#[derive(Clone, PartialEq, Debug)]
struct TMConfiguration {
    state: State,
    tape: Tape,
}

impl TMConfiguration {
    fn new(state: State, tape: Tape) -> Self {
        TMConfiguration { state, tape }
    }
}

#[derive(Clone, PartialEq, Debug)]
struct TMRule {
    state: State,
    character: char,
    next_state: State,
    write_character: char,
    direction: Direction,
}

impl TMRule {
    fn new(
        state: State,
        character: char,
        next_state: State,
        write_character: char,
        direction: Direction,
    ) -> Self {
        TMRule {
            state,
            character,
            next_state,
            write_character,
            direction,
        }
    }
    fn applies_to(&self, configuration: &TMConfiguration) -> bool {
        self.state == configuration.state && self.character == configuration.tape.middle
    }
    fn follow(&self, configuration: &TMConfiguration) -> TMConfiguration {
        TMConfiguration::new(self.next_state, self.next_tape(configuration))
    }
    fn next_tape(&self, configuration: &TMConfiguration) -> Tape {
        let written_tape = configuration.tape.write(self.write_character);
        return match self.direction {
            Direction::Right => written_tape.move_head_right(),
            Direction::Left => written_tape.move_head_left(),
        };
    }
}

#[derive(Clone, PartialEq, Debug)]
enum Direction {
    Right,
    Left,
}

#[derive(Clone, PartialEq, Debug)]
struct DTMRulebook {
    rules: Vec<TMRule>,
}

impl DTMRulebook {
    fn next_configuration(&self, configuration: &TMConfiguration) -> TMConfiguration {
        self.rule_for(configuration).unwrap().follow(configuration)
    }
    fn rule_for(&self, configuration: &TMConfiguration) -> Option<TMRule> {
        self.rules
            .clone()
            .into_iter()
            .find(|rule| rule.applies_to(configuration))
    }
    fn applies_to(&self, configuration: &TMConfiguration) -> bool {
        self.rule_for(configuration).is_some()
    }
}

struct DTM {
    current_configuration: TMConfiguration,
    accept_states: Vec<State>,
    rulebook: DTMRulebook,
}

impl DTM {
    fn accepting(&self) -> bool {
        self.accept_states
            .contains(&self.current_configuration.state)
    }
    fn step(&mut self) {
        self.current_configuration = self
            .rulebook
            .next_configuration(&self.current_configuration);
    }
    fn run(&mut self) {
        loop {
            if self.accepting() || self.is_stuck() {
                break;
            }
            self.step();
        }
    }
    fn is_stuck(&self) -> bool {
        !self.accepting() && !self.rulebook.applies_to(&self.current_configuration)
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tape() {
        let tape = Tape::new(vec!['1', '0', '1'], '1', vec![], '_');
        assert_eq!('1', tape.middle);

        assert_eq!(
            Tape::new(vec!['1', '0'], '1', vec!['1'], '_'),
            tape.move_head_left()
        );
        assert_eq!(
            Tape::new(vec!['1', '0', '1'], '0', vec![], '_'),
            tape.write('0')
        );
        assert_eq!(
            Tape::new(vec!['1', '0', '1', '1'], '_', vec![], '_'),
            tape.move_head_right()
        );
        assert_eq!(
            Tape::new(vec!['1', '0', '1', '1'], '0', vec![], '_'),
            tape.move_head_right().write('0')
        );
    }

    #[test]
    fn test_rule() {
        let rule = TMRule::new(1, '0', 2, '1', Direction::Right);
        assert!(rule.applies_to(&TMConfiguration::new(
            1,
            Tape::new(vec![], '0', vec![], '_')
        )));
        assert!(!rule.applies_to(&TMConfiguration::new(
            1,
            Tape::new(vec![], '1', vec![], '_')
        )));
        assert!(!rule.applies_to(&TMConfiguration::new(
            2,
            Tape::new(vec![], '0', vec![], '_')
        )));
    }

    #[test]
    fn test_follow() {
        let rule = TMRule::new(1, '0', 2, '1', Direction::Right);
        assert_eq!(
            rule.follow(&TMConfiguration::new(
                1,
                Tape::new(vec![], '0', vec![], '_')
            )),
            TMConfiguration::new(2, Tape::new(vec!['1'], '_', vec![], '_'))
        );
    }

    #[test]
    fn test_dtmrulebook() {
        let rulebook = DTMRulebook {
            rules: vec![
                TMRule::new(1, '0', 2, '1', Direction::Right),
                TMRule::new(1, '1', 1, '0', Direction::Left),
                TMRule::new(1, '_', 2, '1', Direction::Right),
                TMRule::new(2, '0', 2, '0', Direction::Right),
                TMRule::new(2, '1', 2, '1', Direction::Right),
                TMRule::new(2, '_', 3, '_', Direction::Left),
            ],
        };
        let mut configuration =
            TMConfiguration::new(1, Tape::new(vec!['1', '0', '1'], '1', vec![], '_'));
        configuration = rulebook.next_configuration(&configuration);
        assert_eq!(
            TMConfiguration::new(1, Tape::new(vec!['1', '0'], '1', vec!['0'], '_')),
            configuration
        );
        configuration = rulebook.next_configuration(&configuration);
        assert_eq!(
            TMConfiguration::new(1, Tape::new(vec!['1',], '0', vec!['0', '0'], '_')),
            configuration
        );
        configuration = rulebook.next_configuration(&configuration);
        assert_eq!(
            TMConfiguration::new(2, Tape::new(vec!['1', '1'], '0', vec!['0'], '_')),
            configuration
        );
    }

    #[test]
    fn test_dtm() {
        let rulebook = DTMRulebook {
            rules: vec![
                TMRule::new(1, '0', 2, '1', Direction::Right),
                TMRule::new(1, '1', 1, '0', Direction::Left),
                TMRule::new(1, '_', 2, '1', Direction::Right),
                TMRule::new(2, '0', 2, '0', Direction::Right),
                TMRule::new(2, '1', 2, '1', Direction::Right),
                TMRule::new(2, '_', 3, '_', Direction::Left),
            ],
        };
        let tape = Tape::new(vec!['1', '0', '1'], '1', vec![], '_');
        let mut dtm = DTM {
            current_configuration: TMConfiguration::new(1, tape),
            accept_states: vec![3],
            rulebook,
        };
        assert!(!dtm.accepting());
        dtm.step();
        assert_eq!(
            TMConfiguration::new(1, Tape::new(vec!['1', '0'], '1', vec!['0'], '_')),
            dtm.current_configuration
        );
        assert!(!dtm.accepting());
        dtm.run();
        assert_eq!(
            TMConfiguration::new(3, Tape::new(vec!['1', '1', '0'], '0', vec!['_'], '_')),
            dtm.current_configuration
        );
        assert!(dtm.accepting());
    }

    #[test]
    fn test_dtm_stuck() {
        // given
        let rulebook = DTMRulebook {
            rules: vec![
                TMRule::new(1, '0', 2, '1', Direction::Right),
                TMRule::new(1, '1', 1, '0', Direction::Left),
                TMRule::new(1, '_', 2, '1', Direction::Right),
                TMRule::new(2, '0', 2, '0', Direction::Right),
                TMRule::new(2, '1', 2, '1', Direction::Right),
                TMRule::new(2, '_', 3, '_', Direction::Left),
            ],
        };
        let tape = Tape::new(vec!['1', '2', '1'], '1', vec![], '_');
        let mut dtm = DTM {
            current_configuration: TMConfiguration::new(1, tape),
            accept_states: vec![3],
            rulebook,
        };

        // when
        dtm.run();

        // then
        assert_eq!(
            TMConfiguration::new(1, Tape::new(vec!['1'], '2', vec!['0', '0'], '_')),
            dtm.current_configuration
        );
        assert!(!dtm.accepting());
        assert!(dtm.is_stuck());
    }

    #[test]
    fn test_aaabbbccc() {
        // given
        let rulebook = DTMRulebook {
            rules: vec![
                TMRule::new(1, 'X', 1, 'X', Direction::Right),
                TMRule::new(1, 'a', 2, 'X', Direction::Right),
                TMRule::new(1, '_', 6, '_', Direction::Left),
                TMRule::new(2, 'a', 2, 'a', Direction::Right),
                TMRule::new(2, 'X', 2, 'X', Direction::Right),
                TMRule::new(2, 'b', 3, 'X', Direction::Right),
                TMRule::new(3, 'b', 3, 'b', Direction::Right),
                TMRule::new(3, 'X', 3, 'X', Direction::Right),
                TMRule::new(3, 'c', 4, 'X', Direction::Right),
                TMRule::new(4, 'c', 4, 'c', Direction::Right),
                TMRule::new(4, '_', 5, '_', Direction::Left),
                TMRule::new(5, 'a', 5, 'a', Direction::Left),
                TMRule::new(5, 'b', 5, 'b', Direction::Left),
                TMRule::new(5, 'c', 5, 'c', Direction::Left),
                TMRule::new(5, 'X', 5, 'X', Direction::Left),
                TMRule::new(5, '_', 1, '_', Direction::Right),
            ],
        };
        let tape = Tape::new(
            vec![],
            'a',
            vec!['a', 'a', 'b', 'b', 'b', 'c', 'c', 'c'],
            '_',
        );
        let mut dtm = DTM {
            current_configuration: TMConfiguration::new(1, tape),
            accept_states: vec![6],
            rulebook,
        };

        // when
        for _ in 0..10 {
            dtm.step();
        }
        // then
        assert_eq!(
            TMConfiguration::new(
                5,
                Tape::new(
                    vec!['X', 'a', 'a', 'X', 'b', 'b', 'X', 'c'],
                    'c',
                    vec!['_'],
                    '_'
                )
            ),
            dtm.current_configuration
        );

        // when
        for _ in 0..25 {
            dtm.step();
        }
        // then
        assert_eq!(
            TMConfiguration::new(
                5,
                Tape::new(
                    vec!['_', 'X', 'X', 'a'],
                    'X',
                    vec!['X', 'b', 'X', 'X', 'c', '_'],
                    '_'
                )
            ),
            dtm.current_configuration
        );

        // when
        dtm.run();
        // then
        assert_eq!(
            TMConfiguration::new(
                6,
                Tape::new(
                    vec!['_', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X'],
                    'X',
                    vec!['_'],
                    '_'
                )
            ),
            dtm.current_configuration
        );
    }
}
