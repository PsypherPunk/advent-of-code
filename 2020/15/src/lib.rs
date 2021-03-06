pub fn get_numbers(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect()
}

pub fn get_nth_number_for_input(input: &[usize], nth: usize) -> usize {
    let mut turns = vec![0; nth];

    input.iter().enumerate().for_each(|(turn, number)| {
        turns[*number] = turn + 1;
    });

    let mut previous_number = 0;
    ((input.len() + 1)..nth).for_each(|current_turn| {
        let next_number = match turns[previous_number] {
            0 => 0,
            turn_last_seen => current_turn - turn_last_seen,
        };
        turns[previous_number] = current_turn;
        previous_number = next_number;
    });

    previous_number
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "0,3,6";

    #[test]
    fn test_part_one() {
        let numbers = get_numbers(&INPUT);

        assert_eq!(436, get_nth_number_for_input(&numbers, 2020));
    }

    #[test]
    fn test_part_two() {
        let numbers = get_numbers(&INPUT);

        assert_eq!(175594, get_nth_number_for_input(&numbers, 30000000));
    }
}
