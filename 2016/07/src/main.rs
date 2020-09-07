use std::fs;

fn supports_tls(ip: &str) -> bool {
    let matches = ip
        .split(&['[', ']'][..])
        .enumerate()
        .map(|(index, section)| {
            (
                index % 2 != 0,
                section
                    .chars()
                    .collect::<Vec<char>>()
                    .windows(4)
                    .any(|window| {
                        window[0] != window[1] && window[0] == window[3] && window[1] == window[2]
                    }),
            )
        })
        .collect::<Vec<(bool, bool)>>();

    if matches
        .iter()
        .find(|(hypernet, abba)| *hypernet && *abba)
        .is_some()
    {
        return false;
    }
    if matches
        .iter()
        .find(|(hypernet, abba)| !*hypernet && *abba)
        .is_some()
    {
        return true;
    }

    false
}

fn get_ips_support_tls_count(input: &str) -> usize {
    input
        .trim()
        .lines()
        .filter(|line| supports_tls(line))
        .count()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "How many IPs in your puzzle input support TLS? {}",
        get_ips_support_tls_count(&input),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abba() {
        let input = "abba[mnop]qrst";

        assert_eq!(1, get_ips_support_tls_count(&input));
    }

    #[test]
    fn test_bddb() {
        let input = "abcd[bddb]xyyx";

        assert_eq!(0, get_ips_support_tls_count(&input));
    }

    #[test]
    fn test_aaaa() {
        let input = "aaaa[qwer]tyui";

        assert_eq!(0, get_ips_support_tls_count(&input));
    }

    #[test]
    fn test_oxxo() {
        let input = "ioxxoj[asdfgh]zxcvbn";

        assert_eq!(1, get_ips_support_tls_count(&input));
    }
}
