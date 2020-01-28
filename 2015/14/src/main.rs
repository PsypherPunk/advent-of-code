use std::fs;

#[derive(Debug)]
struct Reindeer {
    name: String,
    speed_km_s: i32,
    duration: i32,
    rest: i32,
}

fn reindeer_distance_after_seconds(reindeer: &Reindeer, mut seconds: i32) -> i32 {
    let mut distance: i32 = 0;

    while seconds > 0 {
        for _ in 0..reindeer.duration {
            if seconds <= 0 {
                break;
            }
            seconds -= 1;
            distance += reindeer.speed_km_s;
        }
        seconds -= reindeer.rest;
    }

    distance
}

fn get_reindeer(input: &str) -> Vec<Reindeer> {
    input
        .trim()
        .lines()
        .map(|line| {
            let words = line.split_whitespace().collect::<Vec<&str>>();

            Reindeer {
                name: String::from(words[0]),
                speed_km_s: words[3].parse::<i32>().unwrap(),
                duration: words[6].parse::<i32>().unwrap(),
                rest: words[13].parse::<i32>().unwrap(),
            }
        })
        .collect()
}

fn get_winning_reindeer(input: &str, seconds: i32) -> i32 {
    let reindeer = get_reindeer(&input);

    reindeer
        .iter()
        .map(|r| reindeer_distance_after_seconds(r, seconds))
        .max()
        .unwrap()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "…after exactly 2503 seconds, what distance has the winning reindeer traveled? {}",
        get_winning_reindeer(&input, 2503),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seating() {
        let input = r#"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."#;

        let reindeer = get_reindeer(&input);

        assert_eq!(1120, reindeer_distance_after_seconds(&reindeer[0], 1000));
        assert_eq!(1056, reindeer_distance_after_seconds(&reindeer[1], 1000));
    }
}
