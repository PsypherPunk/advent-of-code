use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

#[derive(Debug)]
struct Maze {
    passages: HashMap<Point, Vec<Point>>,
    start: Point,
    end: Point,
}

type Point = (usize, usize);

/// Build a map of coordinates to characters.
fn get_map(input: &str) -> HashMap<Point, char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| ((x, y), c))
                .collect::<Vec<(Point, char)>>()
        })
        .collect::<HashMap<Point, char>>()
}

/// Derive a map of coordinates to a list of adjacent, accessible
/// coordinates.
fn get_passages(map: &HashMap<Point, char>) -> HashMap<Point, Vec<Point>> {
    map.iter()
        .filter(|&(_, &c)| c == '.')
        .map(|(&(x, y), _)| {
            let adjacent = [(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)]
                .iter()
                .filter(|&&adj| match map.get(&adj) {
                    Some(c) => *c == '.',
                    None => false,
                })
                .cloned()
                .collect::<Vec<Point>>();
            ((x, y), adjacent)
        })
        .collect()
}

/// Make each coordinate corresponding to a portal accessible to the
/// other.
fn connect_portals(
    passages: &mut HashMap<Point, Vec<Point>>,
    portals: HashMap<String, Vec<Point>>,
) {
    portals.iter().for_each(|(_, points)| {
        passages
            .entry(points[0])
            .or_insert_with(Vec::new)
            .push(points[1]);
        passages
            .entry(points[1])
            .or_insert_with(Vec::new)
            .push(points[0]);
    });
}

fn get_maze(input: &str) -> Maze {
    let map = get_map(input);
    let mut passages = get_passages(&map);

    let mut portals: HashMap<String, Vec<Point>> = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if y == 0 || x == 0 || !c.is_uppercase() {
                continue;
            }
            if passages.contains_key(&(x, y - 1)) {
                let name: String = vec![c, *map.get(&(x, y + 1)).unwrap()]
                    .into_iter()
                    .collect();
                let points = portals.entry(name).or_insert_with(Vec::new);
                points.push((x, y - 1));
            }
            if passages.contains_key(&(x + 1, y)) {
                let name: String = vec![*map.get(&(x - 1, y)).unwrap(), c]
                    .into_iter()
                    .collect();
                let points = portals.entry(name).or_insert_with(Vec::new);
                points.push((x + 1, y));
            }
            if passages.contains_key(&(x, y + 1)) {
                let name: String = vec![*map.get(&(x, y - 1)).unwrap(), c]
                    .into_iter()
                    .collect();
                let points = portals.entry(name).or_insert_with(Vec::new);
                points.push((x, y + 1));
            }
            if passages.contains_key(&(x - 1, y)) {
                let name: String = vec![c, *map.get(&(x + 1, y)).unwrap()]
                    .into_iter()
                    .collect();
                let points = portals.entry(name).or_insert_with(Vec::new);
                points.push((x - 1, y));
            }
        }
    }

    let start = portals.remove("AA");
    let end = portals.remove("ZZ");

    connect_portals(&mut passages, portals);

    Maze {
        passages,
        start: *start.unwrap().first().unwrap(),
        end: *end.unwrap().first().unwrap(),
    }
}

fn breadth_first_search(maze: &Maze) -> usize {
    let mut queue = VecDeque::new();
    let mut discovered: HashSet<&Point> = HashSet::new();

    discovered.insert(&maze.start);
    queue.push_back((&maze.start, 0));

    while !queue.is_empty() {
        let (&next, distance) = queue.pop_front().unwrap();
        if next == maze.end {
            return distance;
        }
        for adjacent in maze.passages.get(&next).unwrap() {
            if !discovered.contains(&adjacent) {
                discovered.insert(&adjacent);
                queue.push_back((adjacent, distance + 1));
            }
        }
    }
    panic!("Oops.");
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");
    let maze = get_maze(&input);
    println!(
        "…how many steps does it take to get from the open tile marked AA to the open tile marked ZZ? {}",
        breadth_first_search(&maze),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_23() {
        let input = String::from(
            r#"         A
         A
  #######.#########
  #######.........#
  #######.#######.#
  #######.#######.#
  #######.#######.#
  #####  B    ###.#
BC...##  C    ###.#
  ##.##       ###.#
  ##...DE  F  ###.#
  #####    G  ###.#
  #########.#####.#
DE..#######...###.#
  #.#########.###.#
FG..#########.....#
  ###########.#####
             Z
             Z       "#,
        );
        let maze = get_maze(&input);
        assert_eq!(23, breadth_first_search(&maze));
    }

    #[test]
    fn test_58() {
        let input = String::from(
            r#"                   A
                   A
  #################.#############
  #.#...#...................#.#.#
  #.#.#.###.###.###.#########.#.#
  #.#.#.......#...#.....#.#.#...#
  #.#########.###.#####.#.#.###.#
  #.............#.#.....#.......#
  ###.###########.###.#####.#.#.#
  #.....#        A   C    #.#.#.#
  #######        S   P    #####.#
  #.#...#                 #......VT
  #.#.#.#                 #.#####
  #...#.#               YN....#.#
  #.###.#                 #####.#
DI....#.#                 #.....#
  #####.#                 #.###.#
ZZ......#               QG....#..AS
  ###.###                 #######
JO..#.#.#                 #.....#
  #.#.#.#                 ###.#.#
  #...#..DI             BU....#..LF
  #####.#                 #.#####
YN......#               VT..#....QG
  #.###.#                 #.###.#
  #.#...#                 #.....#
  ###.###    J L     J    #.#.###
  #.....#    O F     P    #.#...#
  #.###.#####.#.#####.#####.###.#
  #...#.#.#...#.....#.....#.#...#
  #.#####.###.###.#.#.#########.#
  #...#.#.....#...#.#.#.#.....#.#
  #.###.#####.###.###.#.#.#######
  #.#.........#...#.............#
  #########.###.###.#############
           B   J   C
           U   P   P
"#,
        );
        let maze = get_maze(&input);
        assert_eq!(58, breadth_first_search(&maze));
    }
}
