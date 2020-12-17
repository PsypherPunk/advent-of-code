fn get_a016754(n: isize) -> isize {
    ((2 * n) + 1).pow(2)
}

pub fn get_manhattan_distance(n: isize) -> isize {
    let upper = (0..).find(|a| get_a016754(*a) >= n).unwrap();
    let lower = upper - 1;

    let distance_around = n - get_a016754(lower);
    let weird_number_that_serves_some_purpose = ((get_a016754(upper) as f64).sqrt() - 1.0) as isize;
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
}
