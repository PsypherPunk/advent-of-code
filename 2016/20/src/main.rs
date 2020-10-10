use std::cmp::{max, Ordering};
use std::fs;

type IpRange = (u32, u32);
type Blocklist = Vec<IpRange>;

fn get_blocklist(input: &str) -> Blocklist {
    let mut ip_ranges: Blocklist = input
        .trim()
        .lines()
        .map(|line| {
            let nums = line.trim().split('-').collect::<Vec<&str>>();
            (
                nums[0].parse::<u32>().unwrap(),
                nums[1].parse::<u32>().unwrap(),
            )
        })
        .collect();

    ip_ranges.sort_by(|a, b| a.0.cmp(&b.0));

    ip_ranges
}

fn get_lowest_ip_address(blocklist: &[IpRange]) -> u32 {
    let mut candidate = 0;

    for (lower, upper) in blocklist.iter() {
        if let (Ordering::Less, Ordering::Less) = (candidate.cmp(&lower), candidate.cmp(&upper)) {
            return candidate;
        }

        candidate = max(upper + 1, candidate);
    }

    panic!("Errr…couldn't find an IP.");
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");
    let blocklist = get_blocklist(&input);

    println!(
        "…what is the lowest-valued IP that is not blocked? {}",
        get_lowest_ip_address(&blocklist)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0_9() {
        let input = r#"5-8
0-2
4-7"#;
        let blocklist = get_blocklist(&input);

        assert_eq!(3, get_lowest_ip_address(&blocklist));
    }
}
