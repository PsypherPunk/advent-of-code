pub struct Stream {
    pub score: usize,
    pub garbage: usize,
}

pub fn get_stream_for_input(input: &str) -> Stream {
    let mut chars = input.chars().peekable();

    let mut depth = 0;
    let mut stream = Stream {
        score: 0,
        garbage: 0,
    };
    let mut garbage = false;

    while chars.peek().is_some() {
        let c = chars.next().unwrap();
        match c {
            '{' => {
                if !garbage {
                    depth += 1;
                } else {
                    stream.garbage += 1;
                }
            }
            '}' => {
                if !garbage {
                    stream.score += depth;
                    depth -= 1;
                } else {
                    stream.garbage += 1;
                }
            }
            '!' => {
                let _ = chars.next().unwrap();
            }
            '<' => {
                if garbage {
                    stream.garbage += 1;
                } else {
                    garbage = true;
                }
            }
            '>' => {
                garbage = false;
            }
            _ => {
                if garbage {
                    stream.garbage += 1;
                }
            }
        }
    }

    stream
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(get_stream_for_input("{}").score, 1);
        assert_eq!(get_stream_for_input("{{{}}}").score, 6);
        assert_eq!(get_stream_for_input("{{},{}}").score, 5);
        assert_eq!(get_stream_for_input("{{{},{},{{}}}}").score, 16);
        assert_eq!(get_stream_for_input("{<a>,<a>,<a>,<a>}").score, 1);
        assert_eq!(
            get_stream_for_input("{{<ab>},{<ab>},{<ab>},{<ab>}}").score,
            9
        );
        assert_eq!(
            get_stream_for_input("{{<!!>},{<!!>},{<!!>},{<!!>}}").score,
            9
        );
        assert_eq!(
            get_stream_for_input("{{<a!>},{<a!>},{<a!>},{<ab>}}").score,
            3
        );
    }

    #[test]
    fn test_part_two() {
        assert_eq!(get_stream_for_input("<>").garbage, 0);
        assert_eq!(get_stream_for_input("<random characters>").garbage, 17);
        assert_eq!(get_stream_for_input("<<<<>").garbage, 3);
        assert_eq!(get_stream_for_input("<{!>}>").garbage, 2);
        assert_eq!(get_stream_for_input("<!!>").garbage, 0);
        assert_eq!(get_stream_for_input("<!!!>>").garbage, 0);
        assert_eq!(get_stream_for_input(r#"<{o"i!a,<{i<a>"#).garbage, 10);
    }
}
