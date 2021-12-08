use std::collections::HashSet;

#[derive(Clone)]
struct SevenSegmentDisplay {
    inputs: Vec<HashSet<char>>,
    outputs: Vec<HashSet<char>>,
    digits: [HashSet<char>; 10],
}

impl SevenSegmentDisplay {
    fn get_digits(&mut self) {
        self.digits[8] = ('a'..='g').collect();
        if let Some(one) = self.inputs.iter().find(|input| input.len() == 2) {
            self.digits[1] = one.clone();
        }
        if let Some(seven) = self.inputs.iter().find(|input| input.len() == 3) {
            self.digits[7] = seven.clone();
        }
        if let Some(four) = self.inputs.iter().find(|input| input.len() == 4) {
            self.digits[4] = four.clone();
        }
        if let Some(nine) = self
            .inputs
            .iter()
            .filter(|input| input.len() == 6)
            .find(|input| input.is_superset(&self.digits[4]))
        {
            self.digits[9] = nine.clone();
        }
        if let Some(zero) = self
            .inputs
            .iter()
            .filter(|input| input.len() == 6)
            .find(|&input| input.is_superset(&self.digits[1]) && *input != self.digits[9])
        {
            self.digits[0] = zero.clone();
        }
        if let Some(six) = self
            .inputs
            .iter()
            .filter(|input| input.len() == 6)
            .find(|&input| *input != self.digits[9] && *input != self.digits[0])
        {
            self.digits[6] = six.clone();
        }
        if let Some(three) = self
            .inputs
            .iter()
            .filter(|input| input.len() == 5)
            .find(|&input| input.is_superset(&self.digits[1]) && input.is_superset(&self.digits[7]))
        {
            self.digits[3] = three.clone();
        }
        if let Some(five) = self
            .inputs
            .iter()
            .filter(|input| input.len() == 5)
            .find(|&input| input.is_subset(&self.digits[6]))
        {
            self.digits[5] = five.clone();
        }
        if let Some(two) = self
            .inputs
            .iter()
            .filter(|input| input.len() == 5)
            .find(|&input| *input != self.digits[3] && *input != self.digits[5])
        {
            self.digits[2] = two.clone();
        }

        assert!(self.digits.iter().all(|digit| !digit.is_empty()));
    }
}

pub fn get_part_two(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| {
            let (inputs, outputs) = line.split_once(" | ").unwrap();
            let inputs = inputs
                .split_ascii_whitespace()
                .map(|input| input.chars().collect())
                .collect();
            let outputs = outputs
                .split_ascii_whitespace()
                .map(|output| output.chars().collect())
                .collect();

            SevenSegmentDisplay {
                inputs,
                outputs,
                digits: Default::default(),
            }
        })
        .map(|mut display| {
            display.get_digits();
            display
                .outputs
                .iter()
                .map(|output| {
                    display
                        .digits
                        .iter()
                        .enumerate()
                        .find(|(_, digit)| &output == digit)
                        .unwrap()
                        .0
                })
                .fold(0, |acc, digit| (acc * 10) + digit)
        })
        .sum()
}

pub fn get_part_one(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| {
            let (_, outputs) = line.split_once(" | ").unwrap();
            let outputs = outputs
                .split_ascii_whitespace()
                .map(|output| output.chars().collect())
                .collect();

            SevenSegmentDisplay {
                inputs: Vec::new(),
                outputs,
                digits: Default::default(),
            }
        })
        .map(|display| {
            display
                .outputs
                .iter()
                .filter(|output| matches!(output.len(), 2 | 3 | 4 | 7))
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
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(26, get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(61229, get_part_two(INPUT));
    }
}
