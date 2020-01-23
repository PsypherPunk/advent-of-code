use itertools::Itertools;

fn look_and_say(source: &str) -> String {
    source
        .chars()
        .group_by(|&c| c)
        .into_iter()
        .map(|(c, group)| format!("{}{}", group.count(), c))
        .collect()
}

fn main() {
    let input = "1113222113";

    let mut output = String::from(input);
    for _ in 0..40 {
        output = look_and_say(&output);
    }
    println!("What is the length of the result? {}", &output.len());

    let mut output = String::from(input);
    for _ in 0..50 {
        output = look_and_say(&output);
    }
    println!("What is the length of the new result? {}", &output.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!("11", look_and_say("1"));
    }

    #[test]
    fn test_11() {
        assert_eq!("21", look_and_say("11"));
    }

    #[test]
    fn test_21() {
        assert_eq!("1211", look_and_say("21"));
    }

    #[test]
    fn test_1211() {
        assert_eq!("111221", look_and_say("1211"));
    }

    #[test]
    fn test_111221() {
        assert_eq!("312211", look_and_say("111221"));
    }
}
