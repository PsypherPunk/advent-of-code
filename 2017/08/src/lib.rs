use std::cmp::Ordering;
use std::collections::HashMap;

pub fn get_registers(input: &str) -> HashMap<&str, isize> {
    let mut registers = HashMap::new();

    input.trim().lines().for_each(|line| {
        let (reg, inc_dec, amt, ca, op, cb) =
            match line.trim().split_whitespace().collect::<Vec<_>>()[..] {
                [reg, inc_dec, amt, _, ca, op, cb] => (reg, inc_dec, amt, ca, op, cb),
                _ => panic!(r#"¯\_(ツ)_/¯"#),
            };
        let ca = registers.entry(ca).or_insert(0);
        let condition = match (*ca).cmp(&cb.parse::<isize>().unwrap()) {
            Ordering::Less => ["<", "<=", "!="].contains(&op),
            Ordering::Equal => ["<=", ">=", "=="].contains(&op),
            Ordering::Greater => [">", ">=", "!="].contains(&op),
        };
        if condition {
            let reg = registers.entry(reg).or_insert(0);
            match inc_dec {
                "inc" => *reg += amt.parse::<isize>().unwrap(),
                "dec" => *reg -= amt.parse::<isize>().unwrap(),
                _ => panic!(r#"¯\_(ツ)_/¯"#),
            }
        }
    });

    registers
}

pub fn get_highest_value(registers: &HashMap<&str, isize>) -> isize {
    *registers.values().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10"#;

        let registers = get_registers(&input);

        assert_eq!(1, get_highest_value(&registers));
    }
}
