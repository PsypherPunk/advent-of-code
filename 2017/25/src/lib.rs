use std::collections::HashMap;
use std::str::FromStr;

type Rules = HashMap<bool, Rule>;
type States = HashMap<char, Rules>;

pub struct TuringMachine {
    tape: HashMap<isize, bool>,
    cursor: isize,
    state: char,
    diagnostic: usize,
    states: States,
}

pub struct Rule {
    write: bool,
    slot: isize,
    next: char,
}

impl FromStr for TuringMachine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (state, diagnostic, states) = turing_parser::turing_machine(&s).unwrap();

        Ok(Self {
            tape: HashMap::new(),
            cursor: 0,
            state,
            diagnostic,
            states,
        })
    }
}

impl TuringMachine {
    fn run(&mut self) {
        let state = self.states.get(&self.state).unwrap();
        let value = self.tape.entry(self.cursor).or_insert(false);
        let rule = state.get(value).unwrap();

        *value = rule.write;
        self.cursor += rule.slot;
        self.state = rule.next;
    }

    pub fn get_diagnostic_checksum(&mut self) -> usize {
        for _ in 0..self.diagnostic {
            self.run();
        }

        self.tape.values().filter(|&value| *value).count()
    }
}

peg::parser! {
    grammar turing_parser() for str {
        rule _() = [' ' | '\n']*

        rule state_id() -> char
            = s:$(['A'..='Z']) {? s.chars().next().ok_or("Invalid state ID.") }

        rule bit() -> bool
            = b:$(['0' | '1']) { b == "1" }

        rule diagnostic() -> usize
            = n:$(['0'..='9']+) {? n.parse().or(Err("Invalid diagnostic iteration.")) }

        rule direction() -> isize
            = d:$("right" / "left") {? match d {
                    "right" => Ok(1),
                    "left" => Ok(-1),
                    _ => Err("Invalid direction."),
                } }

        rule begin_state() -> char
            = "Begin in state " s:state_id() "." { s }

        rule perform_diagnostic() -> usize
            = "Perform a diagnostic checksum after " n:diagnostic() " steps." { n }

        rule in_state() -> char
            = "In state " s:state_id() ":" { s }

        rule state_rules() -> (bool, Rule)
            = "If the current value is " current:bit() ":"
              _
              "- Write the value " write:bit() "."
              _
              "- Move one slot to the " slot:direction() "."
              _
              "- Continue with state " next:state_id() "."
                { (current, Rule { write, slot, next } ) }

        rule state() -> (char, Rules)
            = in_state:in_state()
              _
              rules:state_rules()
              ++ _
                { (in_state, rules.into_iter().collect()) }

        pub rule turing_machine() -> (char, usize, States)
            = begin:begin_state()
              _
              diagnostic:perform_diagnostic()
              _
              states:state()
              ++ _
              _
                { (begin, diagnostic, states.into_iter().collect()) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.
"#;

    #[test]
    fn test_part_one() {
        let mut turing_machine = TuringMachine::from_str(&INPUT).unwrap();

        assert_eq!(3, turing_machine.get_diagnostic_checksum());
    }
}
