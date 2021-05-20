use std::collections::{HashSet, VecDeque};

type Component = (usize, usize);

pub fn get_strength(components: &[Component]) -> usize {
    components.iter().fold(0, |acc, &(a, b)| acc + a + b)
}

pub fn get_components(s: &str) -> HashSet<Component> {
    s.trim()
        .lines()
        .map(|line| match line.split('/').collect::<Vec<_>>()[..] {
            [a, b] => (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()),
            _ => panic!(r#"¯\_(ツ)_/¯"#),
        })
        .collect()
}

pub fn get_bridges(components: &HashSet<Component>) -> Vec<Vec<Component>> {
    let mut stack = VecDeque::new();
    let mut bridges = Vec::new();

    components.iter().filter(|(a, _)| *a == 0).for_each(|zero| {
        let mut remaining = components.clone();
        remaining.remove(zero);
        stack.push_front((vec![*zero], remaining));
    });

    while !stack.is_empty() {
        let (bridge, remaining) = stack.pop_front().unwrap();
        let last = bridge.last().unwrap();
        let usable = remaining
            .iter()
            .filter(|&(a, b)| *a == last.1 || *b == last.1)
            .collect::<Vec<_>>();

        if usable.is_empty() {
            bridges.push(bridge);
            continue;
        }

        usable.iter().for_each(|&component| {
            let mut remaining = remaining.clone();
            remaining.remove(&component);

            let component = match component.0 == last.1 {
                true => *component,
                false => (component.1, component.0),
            };
            let mut bridge = bridge.clone();
            bridge.push(component);
            stack.push_front((bridge, remaining));
        });
    }

    bridges
}

pub fn get_strongest_bridge(bridges: &mut Vec<Vec<Component>>) -> usize {
    bridges.sort_by(|a, b| get_strength(a).cmp(&get_strength(b)));

    get_strength(bridges.last().unwrap())
}

pub fn get_strongest_longest_bridge(bridges: &mut Vec<Vec<Component>>) -> usize {
    bridges.sort_by(|a, b| {
        a.len()
            .cmp(&b.len())
            .then(get_strength(a).cmp(&get_strength(b)))
    });

    get_strength(bridges.last().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10"#;

    #[test]
    fn test_part_one() {
        let components = get_components(&INPUT);

        let mut bridges = get_bridges(&components);

        assert_eq!(31, get_strongest_bridge(&mut bridges));
    }

    #[test]
    fn test_part_two() {
        let components = get_components(&INPUT);

        let mut bridges = get_bridges(&components);

        assert_eq!(19, get_strongest_longest_bridge(&mut bridges));
    }
}
