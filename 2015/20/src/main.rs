use std::fs;

/// Build up a list of houses as a `Vec` of presents contained therein.
///
/// As a house will have at least ten times its number in presents,
/// we can realistically looking at numbers up to 1/10th of our input.
fn find_first_house_with_present_count(presents: usize) -> usize {
    let mut houses = vec![0; presents / 10];

    for elf in 1..(presents / 10) {
        let mut house = elf;
        while house < (presents / 10) {
            houses[house] += elf * 10;
            house += elf;
        }
    }
    houses.iter().position(|&p| p >= presents).unwrap()
}

fn find_first_house_with_new_present_count(presents: usize) -> usize {
    let mut houses = vec![0; presents / 10];

    for elf in 1..(presents / 11) {
        let mut house = elf;
        let mut visited = 0;
        while house < (presents / 11) && visited < 50 {
            houses[house] += elf * 11;
            house += elf;
            visited += 1;
        }
    }
    houses.iter().position(|&p| p >= presents).unwrap()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What is the lowest house number…to get at least as many presents as…your puzzle input? {}",
        find_first_house_with_present_count(input.trim().parse::<usize>().unwrap()),
    );

    println!(
        "What is the lowest house number…to get at least as many presents as…your puzzle input? {}",
        find_first_house_with_new_present_count(input.trim().parse::<usize>().unwrap()),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {}
}
