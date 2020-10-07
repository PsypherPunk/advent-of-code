use std::fs;

use itertools::Itertools;

const SAFE: char = '.';
const TRAP: char = '^';

fn get_next_row(row: &str) -> String {
    let mut chars: Vec<char> = row.chars().collect();
    chars.insert(0, SAFE);
    chars.push(SAFE);

    chars
        .iter()
        .tuple_windows::<(_, _, _)>()
        .map(|tiles| match tiles {
            (&TRAP, &TRAP, &SAFE) => TRAP,
            (&SAFE, &TRAP, &TRAP) => TRAP,
            (&TRAP, &SAFE, &SAFE) => TRAP,
            (&SAFE, &SAFE, &TRAP) => TRAP,
            _ => SAFE,
        })
        .collect()
}

fn get_map(first_row: &str, row_count: usize) -> String {
    let mut rows = vec![first_row.trim().to_string()];

    (1..row_count).for_each(|_| {
        rows.push(get_next_row(rows.last().unwrap()));
    });

    rows.join("\n")
}

fn get_safe_tile_count(map: &str) -> usize {
    map.lines()
        .map(|line| line.chars().filter(|ch| *ch == SAFE).count())
        .sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let map = get_map(&input, 40);

    println!(
        "…how many safe tiles are there? {}",
        get_safe_tile_count(&map),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3_rows() {
        let input = "..^^.";
        let expected = r#"..^^.
.^^^^
^^..^"#;

        assert_eq!(expected, get_map(input, 3));
    }

    #[test]
    fn test_10_rows() {
        let input = ".^^.^.^^^^";
        let expected = r#".^^.^.^^^^
^^^...^..^
^.^^.^.^^.
..^^...^^^
.^^^^.^^.^
^^..^.^^..
^^^^..^^^.
^..^^^^.^^
.^^^..^.^^
^^.^^^..^^"#;

        let map = get_map(&input, 10);
        assert_eq!(expected, map);
        assert_eq!(38, get_safe_tile_count(&map));
    }
}
