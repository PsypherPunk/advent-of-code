use std::cmp::Ordering;
use std::fs;

fn get_winner(elf_count: &str) -> usize {
    let elf_count = elf_count.trim().parse::<usize>().unwrap();

    let nearest_power_of_2 = 2_usize.pow((elf_count as f64).log2().floor() as u32);
    let l = elf_count - nearest_power_of_2;

    (2 * l) + 1
}

fn get_winner_opposite(elf_count: &str) -> usize {
    let elf_count = elf_count.trim().parse::<usize>().unwrap();

    let nearest_power_of_3 = 3_usize.pow((elf_count as f64).log(3.0).floor() as u32);
    let l = elf_count - nearest_power_of_3;

    if l == 0 {
        nearest_power_of_3
    } else {
        match l.cmp(&nearest_power_of_3) {
            Ordering::Less => l,
            Ordering::Greater => nearest_power_of_3 + (l - nearest_power_of_3) * 2,
            _ => panic!("Errrr…"),
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!("…which Elf gets all the presents? {}", get_winner(&input));

    println!(
        "…which Elf now gets all the presents? {}",
        get_winner_opposite(&input)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_5() {
        let input = "5";

        assert_eq!(get_winner(&input), 3);
    }

    #[test]
    fn test_5_opposite() {
        let input = "5";

        assert_eq!(get_winner_opposite(&input), 2);
    }
}
