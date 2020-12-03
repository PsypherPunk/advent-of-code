use std::fs;

fn get_tree_count(input: &str, right: usize, down: usize) -> usize {
    input
        .trim()
        .lines()
        .enumerate()
        .filter(|(num, _)| num % down == 0)
        .map(|(_, line)| line)
        .enumerate()
        .map(|(num, line)| {
            let pos = (num * right) % line.len();
            match line.chars().nth(pos).unwrap() {
                '.' => 0,
                '#' => 1,
                _ => panic!("Hmmmm…this shouldn't happen 🤨"),
            }
        })
        .sum()
}

fn get_slopes_count(input: &str) -> usize {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(right, down)| get_tree_count(&input, *right, *down))
        .product()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "…how many trees would you encounter? {}",
        get_tree_count(&input, 3, 1),
    );

    println!(
        "What do you get if you multiply together the number of trees encountered on each of the listed slopes? {}",
        get_slopes_count(&input),
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

        assert_eq!(7, get_tree_count(&input, 3, 1));
    }

    #[test]
    fn test_part_two() {
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

        assert_eq!(336, get_slopes_count(&input));
    }
}
