struct SevenSegmentDisplay {
    inputs: Vec<String>,
    outputs: Vec<String>,
}

pub fn get_part_one(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| {
            let (inputs, outputs) = line.split_once(" | ").unwrap();
            let inputs = inputs
                .split_ascii_whitespace()
                .map(str::to_string)
                .collect();
            let outputs = outputs
                .split_ascii_whitespace()
                .map(str::to_string)
                .collect();

            SevenSegmentDisplay { inputs, outputs }
        })
        .map(|display| {
            display
                .outputs
                .iter()
                .filter(|output| [2, 4, 3, 7].contains(&output.len()))
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#;

    #[test]
    fn test_part_one() {
        assert_eq!(26, get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(1, 2)
    }
}
