use num::complex::Complex;
use std::collections::HashMap;

fn get_neighbours(square: (i32, i32)) -> Vec<(i32, i32)> {
    let (x, y) = square;

    (-1..=1)
        .flat_map(|dx| (-1..=1).map(move |dy| (x + dx, y + dy)))
        .filter(|neighbour| *neighbour != square)
        .collect()
}

pub fn get_first_square_spiral_sum_greater_than(n: i32) -> i32 {
    let mut squares = HashMap::new();

    let mut position = Complex::new(0, 0);
    squares.insert((position.re, position.im), 1);

    let mut facing = Complex::new(1, 0);

    let mut value = 0;
    while value < n {
        position += facing;
        let neighbours = get_neighbours((position.re, position.im));

        value = neighbours
            .iter()
            .map(|neighbour| match squares.get(neighbour) {
                Some(value) => *value,
                None => 0,
            })
            .sum();
        squares.insert((position.re, position.im), value);

        let left = position + (facing * Complex::i().powi(-1));
        if !squares.contains_key(&(left.re, left.im)) {
            facing *= Complex::i().powi(-1);
        }
    }
    value
}

fn get_a016754(n: i32) -> i32 {
    ((2 * n) + 1).pow(2)
}

pub fn get_manhattan_distance(n: i32) -> i32 {
    let upper = (0..).find(|a| get_a016754(*a) >= n).unwrap();
    let lower = upper - 1;

    let distance_around = n - get_a016754(lower);
    let weird_number_that_serves_some_purpose = ((get_a016754(upper) as f64).sqrt() - 1.0) as i32;
    let distance_from_edge = distance_around % weird_number_that_serves_some_purpose;
    let distance_from_centre =
        (distance_from_edge - (weird_number_that_serves_some_purpose / 2)).abs();

    distance_from_centre + upper
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(3, get_manhattan_distance(12));
        assert_eq!(2, get_manhattan_distance(23));
        assert_eq!(31, get_manhattan_distance(1024));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(23, get_first_square_spiral_sum_greater_than(20));
        assert_eq!(747, get_first_square_spiral_sum_greater_than(700));
    }
}
