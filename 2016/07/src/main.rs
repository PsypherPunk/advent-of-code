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

    if matches.iter().any(|(hypernet, abba)| *hypernet && *abba) {
        return false;
    }
    if matches.iter().any(|(hypernet, abba)| !*hypernet && *abba) {
        return true;
    }

    false
}

fn supports_ssl(ip: &str) -> bool {
    let matches = ip
        .split(&['[', ']'][..])
        .enumerate()
        .map(|(index, section)| {
            (
                index % 2 != 0,
                section
                    .chars()
                    .collect::<Vec<char>>()
                    .windows(3)
                    .filter(|&window| window[0] != window[1] && window[0] == window[2])
                    .map(|aba| aba.iter().collect::<String>())
                    .collect::<Vec<String>>(),
            )
        })
        .collect::<Vec<(bool, Vec<String>)>>();

    let hypernet_matches = matches
        .iter()
        .filter(|(hypernet, aba)| *hypernet && !aba.is_empty())
        .flat_map(|(_, aba)| aba.clone())
        .collect::<Vec<String>>();

    matches
        .into_iter()
        .filter(|(hypernet, aba)| !*hypernet && !aba.is_empty())
        .flat_map(|(_, aba)| aba)
        .any(|aba| {
            let aba = aba.chars().collect::<Vec<char>>();
            let supernet_bab = [aba[1], aba[0], aba[1]].iter().collect::<String>();
            hypernet_matches.iter().any(|bab| *bab == supernet_bab)
        })
}

fn get_ips_support_tls_count(input: &str) -> usize {
    input
        .trim()
        .lines()
        .filter(|line| supports_tls(line))
        .count()
}

fn get_ips_support_ssl_count(input: &str) -> usize {
    input
        .trim()
        .lines()
        .filter(|line| supports_ssl(line))
        .count()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "How many IPs in your puzzle input support TLS? {}",
        get_ips_support_tls_count(&input),
    );

    println!(
        "How many IPs in your puzzle input support SSL? {}",
        get_ips_support_ssl_count(&input),
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

    #[test]
    fn test_aba() {
        let input = "aba[bab]xyz";

        assert_eq!(1, get_ips_support_ssl_count(&input));
    }

    #[test]
    fn test_xyx() {
        let input = "xyx[xyx]xyx";

        assert_eq!(0, get_ips_support_ssl_count(&input));
    }

    #[test]
    fn test_eke() {
        let input = "aaa[kek]eke";

        assert_eq!(1, get_ips_support_ssl_count(&input));
    }

    #[test]
    fn test_zaz() {
        let input = "zazbz[bzb]cdb";

        assert_eq!(1, get_ips_support_ssl_count(&input));
    }
}
