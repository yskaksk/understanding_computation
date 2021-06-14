struct FARule {
    state: i32,
    character: char,
    next_state: i32
}

impl FARule {
    fn inspect(&self) -> String {
        format!("#<FARule #{} --#{}--> #{}>", self.state.to_string(), self.character.to_string(), self.next_state.to_string())
    }

    fn follow(&self) -> i32 {
        self.next_state
    }

    fn applies_to(&self, state: i32, character: char) -> bool {
        (self.state == state) && (self.character == character)
    }
}

struct DFARuleBook {
    rules: Vec<FARule>
}

impl DFARuleBook {
}

fn main() {
    println!("Hello, world!");
}
