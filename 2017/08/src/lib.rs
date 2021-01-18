use std::cmp::Ordering;
use std::collections::HashMap;

pub struct Cpu<'a> {
    pub registers: HashMap<&'a str, isize>,
    pub highest: isize,
}

pub fn get_cpu(input: &str) -> Cpu {
    let mut registers = HashMap::new();
    let mut highest = 0;

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
            highest = highest.max(*reg);
        }
    });

    Cpu { registers, highest }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10"#;

    #[test]
    fn test_part_one() {
        let cpu = get_cpu(&INPUT);

        assert_eq!(1, *cpu.registers.values().max().unwrap());
    }

    #[test]
    fn test_part_two() {
        let cpu = get_cpu(&INPUT);

        assert_eq!(10, cpu.highest);
    }
}
