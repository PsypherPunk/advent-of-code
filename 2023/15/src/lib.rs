enum Operation {
    Remove,
    Insert(usize),
}

struct Step<'a> {
    label: &'a str,
    operation: Operation,
    hash: usize,
}

peg::parser! {
    pub grammar manual() for str {
        rule with_slice<T>(r: rule<T>) -> (T, &'input str)
            = value:&r() input:$(r()) { (value, input) }

        rule _() = [' ' | '\n']*

        rule integer() -> usize
            = n:$(['0'..='9']+)
                {? n.parse().or(Err("invalid integer")) }

        rule label() -> &'input str
            = l:$(['a'..='z']+)

        rule insert() -> Step<'input>
            = label:label()
              "="
              focal_length:integer()
              {
                Step {
                    label,
                    operation: Operation::Insert(focal_length),
                    hash: hash(label),
                }
              }

        rule remove() -> Step<'input>
            = label:label()
              "-"
              {
                Step {
                    label,
                    operation: Operation::Remove,
                    hash: hash(label),
                }
              }

        pub rule steps() -> Vec<Step<'input>>
            = steps:(insert() / remove()) ++ ","

    }
}

fn hash(string: &str) -> usize {
    string
        .as_bytes()
        .iter()
        .fold(0, |acc, c| ((acc + *c as usize) * 17) % 256)
}

pub fn get_part_one(input: &str) -> Result<usize, String> {
    let sum = input.trim().split(',').map(hash).sum();

    Ok(sum)
}

pub fn get_part_two(input: &str) -> Result<usize, String> {
    let steps = manual::steps(input.trim()).map_err(|e| e.to_string())?;

    let boxes = steps
        .iter()
        .try_fold(vec![vec![]; 256], |mut boxes, step| {
            let box_: &mut Vec<&Step> = boxes
                .get_mut(step.hash)
                .ok_or(format!("no boxes at {}", step.hash))?;

            match step.operation {
                Operation::Remove => {
                    if let Some(index) = box_.iter().position(|lens| lens.label == step.label) {
                        box_.remove(index);
                    }
                }
                Operation::Insert(_) => match box_.iter().position(|lens| lens.label == step.label)
                {
                    Some(index) => box_[index] = step,
                    None => {
                        box_.push(step);
                    }
                },
            };

            Ok::<Vec<Vec<_>>, String>(boxes)
        })?;

    let focusing_power = boxes
        .iter()
        .enumerate()
        .filter(|(_, box_)| !box_.is_empty())
        .map(|(box_number, box_)| {
            box_.iter()
                .enumerate()
                .map(|(slot, step)| match step.operation {
                    Operation::Insert(focal_length) => (box_number + 1) * (slot + 1) * focal_length,
                    _ => unreachable!(),
                })
                .sum::<usize>()
        })
        .sum();

    Ok(focusing_power)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#;

    #[test]
    fn test_hash() {
        assert_eq!(52, hash("HASH"));
    }

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(1320), get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(145), get_part_two(INPUT));
    }
}
