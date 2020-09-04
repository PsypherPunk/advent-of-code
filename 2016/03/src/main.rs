use std::fs;

fn get_ints(input: &str) -> Vec<Vec<usize>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>()
}

fn get_valid_triangle_count(input: &str) -> usize {
    get_ints(&input)
        .iter()
        .filter(|&abc| {
            abc[0] + abc[1] > abc[2] && abc[1] + abc[2] > abc[0] && abc[0] + abc[2] > abc[1]
        })
        .count()
}

fn get_valid_triangle_count_columnar(input: &str) -> usize {
    (0..3)
        .map(|column| {
            get_ints(&input).iter()
                .map(|row| row[column])
                .collect::<Vec<usize>>()
                .chunks(3)
                .filter(|&abc| {
                    abc[0] + abc[1] > abc[2] && abc[1] + abc[2] > abc[0] && abc[0] + abc[2] > abc[1]
                })
                .count()
        })
        .sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "…how many of the listed triangles are possible? {}",
        get_valid_triangle_count(&input),
    );

    println!(
        "…how many of the listed triangles are possible? {}",
        get_valid_triangle_count_columnar(&input),
    )
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
