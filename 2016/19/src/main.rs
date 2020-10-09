use std::fs;

fn get_winner(elf_count: &str) -> usize {
    let elf_count = elf_count.trim().parse::<usize>().unwrap();

    let nearest_power_of_2 = 2_usize.pow((elf_count as f64).log2().floor() as u32);
    let l = elf_count - nearest_power_of_2;

    (2 * l) + 1
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!("…which Elf gets all the presents? {}", get_winner(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_5() {
        let input = "5";

        assert_eq!(get_winner(&input), 3);
    }
}
