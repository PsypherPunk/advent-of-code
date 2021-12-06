fn get_lanternfish_count_after(lanternfish: &mut Vec<usize>, days: usize) -> usize {
    let mut timer_count = [0; 9];

    for fish in lanternfish {
        timer_count[*fish] += 1;
    }

    for _ in 0..days {
        let new_fish = timer_count[0];
        for timer in 0..timer_count.len() - 1 {
            timer_count[timer] = timer_count[timer + 1];
        }
        timer_count[8] = new_fish;
        timer_count[6] += new_fish;
    }

    timer_count.iter().sum()
}

pub fn get_part_one(input: &str) -> usize {
    let mut lanternfish = input
        .trim()
        .split(',')
        .map(|digit| digit.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    get_lanternfish_count_after(&mut lanternfish, 80)
}

pub fn get_part_two(input: &str) -> usize {
    let mut lanternfish = input
        .trim()
        .split(',')
        .map(|digit| digit.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    get_lanternfish_count_after(&mut lanternfish, 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3,4,3,1,2";

    #[test]
    fn test_part_one() {
        assert_eq!(5_934, get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(26_984_457_539, get_part_two(INPUT));
    }
}
