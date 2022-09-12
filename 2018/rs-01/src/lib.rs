use std::collections::HashSet;

pub fn get_resulting_frequency(input: &str) -> isize {
    input
        .trim()
        .lines()
        .map(|line| line.parse::<isize>().unwrap())
        .sum()
}

pub fn get_first_duplicated_frequency(input: &str) -> isize {
    let mut seen = HashSet::new();
    let mut current = 0;

    for change in input
        .trim()
        .lines()
        .cycle()
        .map(|line| line.parse::<isize>().unwrap())
    {
        seen.insert(current);
        current += change;
        if seen.contains(&current) {
            return current;
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use parameterized::parameterized;

    #[parameterized(input = {
        r#"+1
-2
+3
+1"#, r#"+1
+1
+1"#, r#"+1
+1
-2"#, r#"-1
-2
-3"#
    }, frequency = {
        3, 3, 0, -6
    })]
    fn test_part_one(input: &str, frequency: isize) {
        assert_eq!(frequency, get_resulting_frequency(&input));
    }

    #[parameterized(input = {
        r#"+1
-2
+3
+1"#, r#"+1
-1"#, r#"+3
+3
+4
-2
-4"#, r#"-6
+3
+8
+5
-6"#, r#"+7
+7
-2
-7
-4"#
    }, frequency = {
        2, 0, 10, 5, 14
    })]
    fn test_part_two(input: &str, frequency: isize) {
        assert_eq!(frequency, get_first_duplicated_frequency(&input));
    }
}
