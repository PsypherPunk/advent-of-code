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
            line
                .chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| (x as isize, y as isize))
        })
        .collect::<Vec<(isize, isize)>>()
}

fn get_best_asteroid(asteroids: Vec<(isize, isize)>) -> (Option<(isize, isize)>, usize) {
    let mut best_asteroid = (None, 0);

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
        if targets.keys().len() > best_asteroid.1 {
            best_asteroid = (Some(*source), targets.keys().len());
        }
    }
    best_asteroid
}

fn main() {
    let input = read_input();
    let asteroids = get_asteroids(&input);
    let best_asteroid = get_best_asteroid(asteroids);
    println!("How many other asteroids can be detected from that location? {}", best_asteroid.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3_4() {
        let input = String::from(r#".#..#
.....
#####
....#
...##
"#);
        let asteroids = get_asteroids(&input);
        assert_eq!(get_best_asteroid(asteroids).0.unwrap(), (3, 4));
    }

    #[test]
    fn test_5_8() {
        let input = String::from(r#"......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####"#);
        let asteroids = get_asteroids(&input);
        assert_eq!(get_best_asteroid(asteroids).0.unwrap(), (5, 8));
    }

    #[test]
    fn test_6_3() {
        let input = String::from(r#".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#.."#);
        let asteroids = get_asteroids(&input);
        assert_eq!(get_best_asteroid(asteroids).0.unwrap(), (6, 3));
    }

    #[test]
    fn test_11_13() {
        let input = String::from(r#".#..##.###...#######
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
###.##.####.##.#..##"#);
        let asteroids = get_asteroids(&input);
        assert_eq!(get_best_asteroid(asteroids).0.unwrap(), (11, 13));
    }
}
