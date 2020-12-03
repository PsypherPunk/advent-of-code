use std::fs;

fn get_tree_count(input: &str) -> usize {
    input
        .trim()
        .lines()
        .enumerate()
        .map(|(num, line)| {
            let pos = (num * 3) % line.len();
            match line.chars().nth(pos).unwrap() {
                '.' => 0,
                '#' => 1,
                _ => panic!("Hmmmm…this shouldn't happen 🤨"),
            }
        })
        .sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "…how many trees would you encounter? {}",
        get_tree_count(&input),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"#;

        assert_eq!(7, get_tree_count(&input));
    }
}
