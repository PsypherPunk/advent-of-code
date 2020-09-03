use std::fs;

fn get_valid_triangle_count(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| {
            let abc = line.trim().split_whitespace().collect::<Vec<&str>>();
            (
                abc[0].parse::<usize>().unwrap(),
                abc[1].parse::<usize>().unwrap(),
                abc[2].parse::<usize>().unwrap(),
            )
        })
        .filter(|(a, b, c)| a + b > *c && b + c > *a && a + c > *b)
        .count()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "…how many of the listed triangles are possible? {}",
        get_valid_triangle_count(&input),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_5_10_25() {
        let input = "5 10 25";

        assert_eq!(0, get_valid_triangle_count(&input));
    }
}
