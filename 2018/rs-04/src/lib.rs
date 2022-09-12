use std::collections::HashMap;

type Guard = HashMap<usize, usize>;

fn get_guards(input: &str) -> HashMap<usize, Guard> {
    let mut lines = input.trim().lines().collect::<Vec<_>>();
    lines.sort_unstable();

    let mut guards = HashMap::new();
    let mut current_guard = 0;
    let mut asleep = 0;
    let mut wakes;

    for line in lines {
        let parts = line.trim().split_whitespace().collect::<Vec<_>>();
        if line.trim().ends_with("begins shift") {
            current_guard = parts[3][1..].parse().unwrap();
        }
        if line.trim().ends_with("asleep") {
            asleep = parts[1][3..=4].parse().unwrap();
        }
        if line.trim().ends_with("up") {
            wakes = parts[1][3..=4].parse().unwrap();

            let guard = guards.entry(current_guard).or_insert_with(HashMap::new);
            for minute in asleep..wakes {
                let count = guard.entry(minute).or_insert(0);
                *count += 1;
            }
        }
    }

    guards
}

pub fn get_strategy_one(input: &str) -> usize {
    let guards = get_guards(input);

    let (sleepiest_guard, _) = guards
        .iter()
        .max_by_key(|&(_, minutes)| minutes.values().sum::<usize>())
        .unwrap();

    let (sleepiest_minute, _) = guards
        .get(sleepiest_guard)
        .unwrap()
        .iter()
        .max_by_key(|&(_, count)| count)
        .unwrap();

    sleepiest_guard * sleepiest_minute
}

pub fn get_strategy_two(input: &str) -> usize {
    let guards = get_guards(input);

    let (sleepiest_guard, minutes) = guards
        .iter()
        .max_by_key(|&(_, minutes)| minutes.values().max().unwrap())
        .unwrap();
    
    let (frequent_minute, _) = minutes
        .iter()
        .max_by_key(|&(_, count)| count)
        .unwrap();

    sleepiest_guard * frequent_minute
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up"#;

    #[test]
    fn test_part_one() {
        assert_eq!(240, get_strategy_one(&INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(4455, get_strategy_two(&INPUT));
    }
}
