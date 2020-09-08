use std::fs;

fn get_decompressed(input: &str) -> String {
    let mut output = Vec::new();
    let mut decompress = Vec::new();

    let chars = input.trim().chars().collect::<Vec<char>>();
    let mut index = 0;
    let mut decompressing = false;

    while index < chars.len() {
        match chars[index] {
            '(' => {
                decompressing = true;
            }
            ')' => {
                decompressing = false;
                let instr = decompress.iter().collect::<String>();
                let instr = instr.split('x').collect::<Vec<_>>();
                let (count, times) = (
                    instr[0].parse::<usize>().unwrap(),
                    instr[1].parse::<usize>().unwrap(),
                );

                let to_duplicate = chars[(index + 1)..(index + 1 + count)].to_vec();
                for _ in 0..times {
                    output.append(&mut to_duplicate.clone());
                }
                index += count;
                decompress.truncate(0);
            }
            _ => {
                if decompressing {
                    decompress.push(chars[index]);
                } else {
                    output.push(chars[index]);
                }
            }
        }
        index += 1;
    }

    output.iter().collect()
}

fn get_decompressed_length_v2(input: &str) -> usize {
    let mut output = 0;
    let mut decompress = Vec::new();

    let chars = input.trim().chars().collect::<Vec<char>>();
    let mut index = 0;
    let mut decompressing = false;

    while index < chars.len() {
        match chars[index] {
            '(' => {
                decompressing = true;
            }
            ')' => {
                decompressing = false;
                let instr = decompress.iter().collect::<String>();
                let instr = instr.split('x').collect::<Vec<_>>();
                let (count, times) = (
                    instr[0].parse::<usize>().unwrap(),
                    instr[1].parse::<usize>().unwrap(),
                );

                let to_duplicate = chars[(index + 1)..(index + 1 + count)]
                    .iter()
                    .collect::<String>();
                let dec = get_decompressed_length_v2(&to_duplicate);
                output += dec * times;
                index += count;
                decompress.truncate(0);
            }
            _ => {
                if decompressing {
                    decompress.push(chars[index]);
                } else {
                    output += 1;
                }
            }
        }
        index += 1;
    }

    output
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What is the decompressed length of the file…? {}",
        get_decompressed(&input).len(),
    );

    println!(
        "What is the decompressed length of the file using this improved format? {}",
        get_decompressed_length_v2(&input),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advent() {
        let input = "ADVENT";

        assert_eq!("ADVENT", get_decompressed(&input));
    }

    #[test]
    fn test_abbbbbc() {
        let input = "A(1x5)BC";

        assert_eq!("ABBBBBC", get_decompressed(&input));
    }

    #[test]
    fn test_xyzxyzxyz() {
        let input = "(3x3)XYZ";

        assert_eq!("XYZXYZXYZ", get_decompressed(&input));
    }

    #[test]
    fn test_abcbcdefefg() {
        let input = "A(2x2)BCD(2x2)EFG";

        assert_eq!("ABCBCDEFEFG", get_decompressed(&input));
    }

    #[test]
    fn test_1x3a() {
        let input = "(6x1)(1x3)A";

        assert_eq!("(1x3)A", get_decompressed(&input));
    }

    #[test]
    fn test_x8x23x3abcy() {
        let input = "X(8x2)(3x3)ABCY";

        assert_eq!("X(3x3)ABC(3x3)ABCY", get_decompressed(&input));
    }

    #[test]
    fn test_xyzxyzxyz_v2() {
        let input = "(3x3)XYZ";

        assert_eq!("XYZXYZXYZ".len(), get_decompressed_length_v2(&input));
    }

    #[test]
    fn test_x8x23x3abcy_v2() {
        let input = "X(8x2)(3x3)ABCY";

        assert_eq!(
            "XABCABCABCABCABCABCY".len(),
            get_decompressed_length_v2(&input)
        );
    }

    #[test]
    fn test_27x1220x1213x147x101x12a() {
        let input = "(27x12)(20x12)(13x14)(7x10)(1x12)A";

        assert_eq!(241920, get_decompressed_length_v2(&input));
    }

    #[test]
    fn test_25x33x3abc2x3xy5x2pqrstx18x93x2two5x7seven() {
        let input = "(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN";

        assert_eq!(445, get_decompressed_length_v2(&input));
    }
}
