use std::fs;

#[derive(Debug)]
enum State {
    Flying(i32),
    Resting(i32),
}

#[derive(Debug)]
struct Reindeer {
    name: String,
    speed_km_s: i32,
    distance: i32,
    duration: i32,
    rest: i32,
    points: u32,
    state: State,
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

fn reindeer_points_after_seconds(reindeer: &mut Vec<Reindeer>, seconds: i32) {
    for _ in 0..seconds {
        for deer in reindeer.iter_mut() {
            deer.state = match deer.state {
                State::Flying(remaining) => {
                    if remaining > 0 {
                        deer.distance += deer.speed_km_s;
                        State::Flying(remaining - 1)
                    } else {
                        State::Resting(deer.rest - 1)
                    }
                }
                State::Resting(remaining) => {
                    if remaining > 0 {
                        State::Resting(remaining - 1)
                    } else {
                        deer.distance += deer.speed_km_s;
                        State::Flying(deer.duration - 1)
                    }
                }
            };
        }

        let leading_distance = reindeer.iter().map(|deer| deer.distance).max().unwrap();
        reindeer
            .iter_mut()
            .filter(|deer| deer.distance == leading_distance)
            .for_each(|deer| deer.points += 1);
    }
}

fn get_lead_reindeer(input: &str, seconds: i32) -> u32 {
    let mut reindeer = get_reindeer(&input);

    reindeer_points_after_seconds(&mut reindeer, seconds);

    reindeer.sort_by(|l, r| l.points.cmp(&r.points));

    reindeer.last().unwrap().points
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
                distance: 0,
                duration: words[6].parse::<i32>().unwrap(),
                rest: words[13].parse::<i32>().unwrap(),
                points: 0,
                state: State::Flying(words[6].parse::<i32>().unwrap()),
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
    );

    println!(
        "…how many points does the winning reindeer have? {}",
        get_lead_reindeer(&input, 2503),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."#;

        let reindeer = get_reindeer(&input);

        assert_eq!(1120, reindeer_distance_after_seconds(&reindeer[0], 1000));
        assert_eq!(1056, reindeer_distance_after_seconds(&reindeer[1], 1000));
    }

    #[test]
    fn test_part2() {
        let input = r#"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."#;

        let mut reindeer = get_reindeer(&input);
        reindeer_points_after_seconds(&mut reindeer, 1000);

        assert_eq!(312, reindeer[0].points);
        assert_eq!(689, reindeer[1].points);
    }
}
