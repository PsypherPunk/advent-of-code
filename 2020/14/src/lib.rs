use std::collections::HashMap;

type BitMask = Vec<(usize, Option<bool>)>;
type Memory = HashMap<usize, usize>;

struct MemoryWrite {
    address: usize,
    value: usize,
}

fn get_bitmask(mask: &str) -> BitMask {
    mask.chars()
        .rev()
        .enumerate()
        .map(|(index, bit)| {
            (
                index,
                match bit {
                    'X' => None,
                    '0' => Some(false),
                    '1' => Some(true),
                    _ => panic!("Invalid character: {}", bit),
                },
            )
        })
        .collect()
}

fn get_memory_write(instruction: &str) -> MemoryWrite {
    let address_end = instruction.chars().position(|c| c == ']').unwrap();
    let value_start = instruction.chars().position(|c| c == '=').unwrap() + 2;

    let address = instruction[4..address_end].parse().unwrap();
    let value = instruction[value_start..].parse().unwrap();

    MemoryWrite { address, value }
}

pub fn read_initialization_program(input: &str) -> usize {
    let mut memory = Memory::new();
    let mut bitmask = BitMask::new();

    input.trim().lines().for_each(|line| match line {
        mask if mask.starts_with("mask") => {
            bitmask = get_bitmask(&line[7..]);
        }
        mem if mem.starts_with("mem") => {
            let mut write = get_memory_write(&mem);

            for (i, bit) in bitmask.iter().filter(|(_, bm)| bm.is_some()) {
                write.value = (write.value & !(1 << i)) | ((bit.unwrap() as usize) << i);
            }
            memory.insert(write.address, write.value);
        }
        _ => panic!("Invalid instruction: {}", line),
    });

    memory.values().sum()
}

pub fn read_initialization_program_v2(input: &str) -> usize {
    let mut memory = Memory::new();
    let mut bitmask = BitMask::new();

    input.trim().lines().for_each(|line| match line {
        mask if mask.starts_with("mask") => {
            bitmask = get_bitmask(&line[7..]);
        }
        mem if mem.starts_with("mem") => {
            let write = get_memory_write(&mem);

            let mut addresses = vec![write.address];
            for (index, bit) in bitmask.iter() {
                match bit {
                    Some(true) => {
                        let bits = 1 << index;
                        addresses.iter_mut().for_each(|address| *address |= bits);
                    }
                    Some(false) => {}
                    None => {
                        let bits = 1 << index;
                        let new = addresses
                            .iter()
                            .map(|address| *address ^ bits)
                            .collect::<Vec<_>>();
                        addresses.extend(new);
                    }
                }
            }
            for address in addresses {
                memory.insert(address, write.value);
            }
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

    #[test]
    fn test_part_two() {
        let input = r#"mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1"#;

        assert_eq!(208, read_initialization_program_v2(&input));
    }
}
