use std::collections::HashMap;

type BitMask = Vec<(usize, Option<bool>)>;
type Memory = HashMap<usize, usize>;

pub fn read_initialization_program(input: &str) -> usize {
    let mut memory = Memory::new();
    let mut bitmask = BitMask::new();

    input.trim().lines().for_each(|line| match line {
        mask if mask.starts_with("mask") => {
            bitmask = line[7..]
                .chars()
                .rev()
                .enumerate()
                .map(|(bit, char)| {
                    (
                        bit,
                        match char {
                            'X' => None,
                            '0' => Some(false),
                            '1' => Some(true),
                            _ => panic!("Invalid character: {}", char),
                        },
                    )
                })
                .collect();
        }
        mem if mem.starts_with("mem") => {
            let address_end = mem.chars().position(|c| c == ']').unwrap();
            let value_start = mem.chars().position(|c| c == '=').unwrap() + 2;

            let address = mem[4..address_end].parse().unwrap();
            let mut value = mem[value_start..].parse().unwrap();

            for (i, bit) in bitmask.iter().filter(|(_, bm)| bm.is_some()) {
                value = (value & !(1 << i)) | ((bit.unwrap() as usize) << i);
            }
            memory.insert(address, value);
        }
        _ => panic!("Invalid instruction: {}", line),
    });

    memory.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0"#;

        assert_eq!(165, read_initialization_program(&input));
    }
}
