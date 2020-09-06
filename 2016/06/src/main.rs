use std::collections::{BTreeMap, HashMap};
use std::fs;

struct Signal {
    signal: String,
}

impl Signal {
    fn from_string(input: &str) -> Self {
        Self {
            signal: input.to_string(),
        }
    }

    fn get_message(&self) -> String {
        let mut counter = BTreeMap::new();

        self.signal.lines().for_each(|line| {
            line.chars().enumerate().for_each(|(pos, ch)| {
                let pos_count = counter.entry(pos).or_insert(HashMap::new());
                let ch_count = pos_count.entry(ch).or_insert(0);
                *ch_count += 1;
            })
        });
        counter
            .into_iter()
            .map(|(_, ch_count)| {
                let mut chars = ch_count.into_iter().collect::<Vec<(char, usize)>>();
                chars.sort_by(|a, b| b.1.cmp(&a.1));
                chars.first().unwrap().0
            })
            .collect()
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let signal = Signal::from_string(&input);

    println!(
        "…what is the error-corrected version of the message being sent? {}",
        signal.get_message(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar
"#;
        let signal = Signal::from_string(&input);

        assert_eq!("easter", signal.get_message());
    }
}
