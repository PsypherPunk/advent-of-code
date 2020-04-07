use std::fs;

type RowColumn = (usize, usize);

const FIRST_CODE: usize = 20_151_125;
const MULTIPLE: usize = 252_533;
const DIVISOR: usize = 33_554_393;

fn get_index(row_column: RowColumn) -> usize {
    let (row, column) = row_column;
    (((row + column - 1).pow(2) + row + column - 1) / 2) - ((row + column - 1) - column)
}

fn get_row_column(input: &str) -> RowColumn {
    let words = input.trim().split_whitespace().collect::<Vec<&str>>();

    (
        words[15][..words[15].len() - 1].parse::<usize>().unwrap(),
        words[17][..words[17].len() - 1].parse::<usize>().unwrap(),
    )
}

fn get_code(index: usize) -> usize {
    let mut result = FIRST_CODE;
    for _ in 1..index {
        result = (result * MULTIPLE) % DIVISOR;
    }

    result
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let row_column = get_row_column(&input);
    let index = get_index(row_column);

    println!("What code do you give the machine? {}", get_code(index),)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_index() {
        assert_eq!(1, get_index((1, 1)));
        assert_eq!(2, get_index((2, 1)));
        assert_eq!(3, get_index((1, 2)));
        assert_eq!(6, get_index((1, 3)));
        assert_eq!(10, get_index((1, 4)));
        assert_eq!(15, get_index((1, 5)));
        assert_eq!(21, get_index((1, 6)));
    }

    #[test]
    fn test_row_column() {
        assert_eq!(20151125, get_code(1));
        assert_eq!(31916031, get_code(2));
        assert_eq!(18749137, get_code(3));
        assert_eq!(16080970, get_code(4));
    }
}
