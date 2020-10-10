use std::cmp::{max, Ordering};
use std::fs;

type IpRange = (u32, u32);

fn get_blocklist(input: &str) -> Vec<IpRange> {
    let mut ip_ranges: Vec<IpRange> = input
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

fn get_allowed_ip_count(blocklist: &[IpRange], limit: u32) -> u32 {
    let mut allowed_ips = 0;

    let mut current = 0;

    for (lower, upper) in blocklist.iter() {
        if let (Ordering::Less, Ordering::Less) = (current.cmp(&lower), current.cmp(&upper)) {
            allowed_ips += (lower - current) - 1;
        }
        current = match upper.cmp(&limit) {
            Ordering::Less => max(*upper, current),
            Ordering::Equal => limit,
            _ => current,
        };
    }
    if current < limit {
        allowed_ips += limit - current;
    }

    allowed_ips
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

    println!(
        "How many IPs are allowed by the blacklist? {}",
        get_allowed_ip_count(&blocklist, 4294967295),
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

    #[test]
    fn test_allowed_ip_count() {
        let input = r#"5-8
0-2
4-7"#;
        let blocklist = get_blocklist(&input);

        let allowed_ips = get_allowed_ip_count(&blocklist, 9);
        dbg!(&allowed_ips);
        assert_eq!(2, allowed_ips);
    }
}
