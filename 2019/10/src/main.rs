use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use num::integer::gcd;

fn read_input() -> String {
    let filename = "input.txt";
    match File::open(filename) {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            content
        }
        Err(error) => {
            panic!("Error opening file {}: {}", filename, error);
        }
    }
}

fn get_asteroids(input: &str) -> Vec<(isize, isize)> {
    input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| (x as isize, y as isize))
        })
        .collect::<Vec<(isize, isize)>>()
}

/// As the `target` is already relative to the origin, we need only
/// calculate the arctangent.
fn get_angle(target: &(isize, isize)) -> f64 {
    let angle = f64::atan2(target.1 as f64, target.0 as f64).to_degrees() + 90_f64;
    ((angle % 360.0) + 360.0) % 360.0
}

fn get_best_asteroid(
    asteroids: Vec<(isize, isize)>,
) -> (
    (isize, isize),
    HashMap<(isize, isize), Vec<(isize, (isize, isize))>>,
) {
    let mut best_asteroid: (
        (isize, isize),
        HashMap<(isize, isize), Vec<(isize, (isize, isize))>>,
    ) = ((0, 0), HashMap::new());

    for source in asteroids.iter() {
        let mut targets: HashMap<(isize, isize), Vec<(isize, (isize, isize))>> = HashMap::new();
        for target in asteroids.iter() {
            if source == target {
                continue;
            }
            let mut dx = (target.0 - source.0) as isize;
            let mut dy = (target.1 - source.1) as isize;
            let dist = gcd(dx.abs(), dy.abs());
            dx /= dist;
            dy /= dist;
            let distances = targets.entry((dx, dy)).or_insert(Vec::new());
            distances.push((dist, *target));
        }
        if targets.keys().len() > best_asteroid.1.keys().len() {
            best_asteroid = (*source, targets);
        }
    }
    best_asteroid
}

fn destroy(
    station: (
        (isize, isize),
        HashMap<(isize, isize), Vec<(isize, (isize, isize))>>,
    ),
) -> Vec<(f64, (isize, isize), isize, (isize, isize))> {
    let mut targets = station.1
        .iter()
        .flat_map(|(k, distances)| {
            let angle = get_angle(&k);
            distances
                .iter()
                .map(|(distance, coord)| (angle, *k, *distance, *coord))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    targets
        .sort_by(|a, b| {
            a.0.partial_cmp(&b.0).unwrap().then(a.2.cmp(&b.2))
//            a.2.cmp(&b.2).then(a.0.partial_cmp(&b.0).unwrap())
        });

    targets
}

fn main() {
    let input = read_input();
    let asteroids = get_asteroids(&input);
    let best_asteroid = get_best_asteroid(asteroids);
    println!(
        "How many other asteroids can be detected from that location? {}",
        best_asteroid.1.keys().len()
    );
    println!("{:?}", best_asteroid.0);

    let destruction_order = destroy(best_asteroid);
    for (i, asteroid) in destruction_order.iter().enumerate() {
        println!("{}, {:?}", i, asteroid.3);
    }
    let two_hundredth = destruction_order.get(199).unwrap();
    println!(
        "…what do you get if you multiply its X coordinate by 100 and then add its Y coordinate? {}",
        ((two_hundredth.3).0 * 100) + (two_hundredth.3).1,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3_4() {
        let input = String::from(
            r#".#..#
.....
#####
....#
...##
"#,
        );
        let asteroids = get_asteroids(&input);
        assert_eq!(get_best_asteroid(asteroids).0, (3, 4));
    }

    #[test]
    fn test_5_8() {
        let input = String::from(
            r#"......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####"#,
        );
        let asteroids = get_asteroids(&input);
        assert_eq!(get_best_asteroid(asteroids).0, (5, 8));
    }

    #[test]
    fn test_6_3() {
        let input = String::from(
            r#".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#.."#,
        );
        let asteroids = get_asteroids(&input);
        assert_eq!(get_best_asteroid(asteroids).0, (6, 3));
    }

    #[test]
    fn test_11_13() {
        let input = String::from(
            r#".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##"#,
        );
        let asteroids = get_asteroids(&input);
        assert_eq!(get_best_asteroid(asteroids).0, (11, 13));
    }

    #[test]
    fn test_8_3() {
        let input = String::from(
            r#".#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....#...###..
..#.#.....#....##"#,
        );
        let asteroids = get_asteroids(&input);
        let best_asteroid = get_best_asteroid(asteroids);
        assert_eq!(best_asteroid.0, (8, 3));

        let destruction = destroy(best_asteroid);
        println!("{:?}", destruction);
        assert_eq!(destruction.get(0).unwrap().3, (8, 1));
    }

    #[test]
    fn test_get_angle() {
        assert_eq!(get_angle(&(0, -2)), 0_f64);
        assert_eq!(get_angle(&(2, 0)), 90_f64);
        assert_eq!(get_angle(&(0, 2)), 180_f64);
        assert_eq!(get_angle(&(-2, 0)), 270_f64);
    }
}
