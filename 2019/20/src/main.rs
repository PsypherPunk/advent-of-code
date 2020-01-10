use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

#[derive(Debug)]
struct Maze {
    passages: HashMap<Point, Vec<Point>>,
    start: Point,
    end: Point,
}

#[derive(Debug)]
struct RecursiveMaze {
    passages: HashMap<Point, Vec<(Point, Edge)>>,
    start: (Point, Edge),
    end: (Point, Edge),
}

type Point = (usize, usize);

#[derive(Clone, Copy, Debug, PartialEq)]
enum Edge {
    Inner,
    Outer,
    Normal,
}

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

/// Derive a map of coordinates to a list of adjacent, accessible
/// coordinates in recursive space.
fn get_recursive_passages(map: &HashMap<Point, char>) -> HashMap<Point, Vec<(Point, Edge)>> {
    map.iter()
        .filter(|&(_, &c)| c == '.')
        .map(|(&(x, y), _)| {
            let adjacent = [(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)]
                .iter()
                .filter(|&&adj| match map.get(&adj) {
                    Some(c) => *c == '.',
                    None => false,
                })
                .map(|&adj| (adj, Edge::Normal))
                .collect::<Vec<(Point, Edge)>>();
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

/// Make each coordinate corresponding to a portal accessible to the
/// other at the alternative edge.
fn connect_recursive_portals(
    passages: &mut HashMap<Point, Vec<(Point, Edge)>>,
    portals: HashMap<String, Vec<(Point, Edge)>>,
) {
    portals.iter().for_each(|(_, points)| {
        passages
            .entry(points[0].0)
            .or_insert_with(Vec::new)
            .push(points[1]);
        passages
            .entry(points[1].0)
            .or_insert_with(Vec::new)
            .push(points[0]);
    });
}

/// Derive a maze, comprising a series of passages plus start and end
/// points.
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

/// Derive a recursive maze, comprising a series of passages at various
/// levels, plus start and end points.
fn get_recursive_maze(input: &str) -> RecursiveMaze {
    let map = get_map(input);
    let mut passages = get_recursive_passages(&map);

    let mut portals: HashMap<String, Vec<(Point, Edge)>> = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if y == 0 || x == 0 || !c.is_uppercase() {
                continue;
            }
            let edge_type = if x < 3 || y < 3 || x > (line.len() - 3) || y > (line.len() - 3) {
                Edge::Outer
            } else {
                Edge::Inner
            };
            if passages.contains_key(&(x, y - 1)) {
                let name: String = vec![c, *map.get(&(x, y + 1)).unwrap()]
                    .into_iter()
                    .collect();
                let points = portals.entry(name).or_insert_with(Vec::new);
                points.push(((x, y - 1), edge_type));
            } else if passages.contains_key(&(x + 1, y)) {
                let name: String = vec![*map.get(&(x - 1, y)).unwrap(), c]
                    .into_iter()
                    .collect();
                let points = portals.entry(name).or_insert_with(Vec::new);
                points.push(((x + 1, y), edge_type));
            } else if passages.contains_key(&(x, y + 1)) {
                let name: String = vec![*map.get(&(x, y - 1)).unwrap(), c]
                    .into_iter()
                    .collect();
                let points = portals.entry(name).or_insert_with(Vec::new);
                points.push(((x, y + 1), edge_type));
            } else if passages.contains_key(&(x - 1, y)) {
                let name: String = vec![c, *map.get(&(x + 1, y)).unwrap()]
                    .into_iter()
                    .collect();
                let points = portals.entry(name).or_insert_with(Vec::new);
                points.push(((x - 1, y), edge_type));
            }
        }
    }

    let start = portals.remove("AA");
    let end = portals.remove("ZZ");

    connect_recursive_portals(&mut passages, portals);

    RecursiveMaze {
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

/// Standard breadth-first search over the maze.
///
/// Disallow ascending levels from the outermost (_"…when at the
/// outermost level…all other outer labeled tiles are effectively walls."_).
fn recursive_breadth_first_search(maze: &RecursiveMaze) -> usize {
    let mut queue = VecDeque::new();
    let mut discovered: HashSet<(Point, usize)> = HashSet::new();

    discovered.insert((maze.start.0, 0));
    queue.push_back((maze.start, 0, 0));

    while !queue.is_empty() {
        let (current, level, distance) = queue.pop_front().unwrap();
        if level == 0 && current.0 == maze.end.0 {
            return distance;
        }
        for &(adjacent, edge) in maze.passages.get(&current.0).unwrap() {
            let new_level = match edge {
                Edge::Inner => {
                    if level == 0 {
                        continue;
                    }
                    level - 1
                }
                Edge::Outer => {
                    level + 1
                }
                Edge::Normal => level,
            };
            if !discovered.contains(&(adjacent, new_level)) {
                discovered.insert((adjacent, new_level));
                queue.push_back(((adjacent, edge), new_level, distance + 1));
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
    );

    let recursive_maze = get_recursive_maze(&input);
    println!(
        "…how many steps does it take to get from the open tile marked AA to the open tile marked ZZ, both at the outermost layer? {}",
        recursive_breadth_first_search(&recursive_maze),
    );
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

    #[test]
    fn test_26_recursive() {
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
        let maze = get_recursive_maze(&input);
        assert_eq!(26, recursive_breadth_first_search(&maze));
    }

    #[test]
    fn test_396_recursive() {
        let input = String::from(
            r#"             Z L X W       C
             Z P Q B       K
  ###########.#.#.#.#######.###############
  #...#.......#.#.......#.#.......#.#.#...#
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###
  #.#...#.#.#...#.#.#...#...#...#.#.......#
  #.###.#######.###.###.#.###.###.#.#######
  #...#.......#.#...#...#.............#...#
  #.#########.#######.#.#######.#######.###
  #...#.#    F       R I       Z    #.#.#.#
  #.###.#    D       E C       H    #.#.#.#
  #.#...#                           #...#.#
  #.###.#                           #.###.#
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#
CJ......#                           #.....#
  #######                           #######
  #.#....CK                         #......IC
  #.###.#                           #.###.#
  #.....#                           #...#.#
  ###.###                           #.#.#.#
XF....#.#                         RF..#.#.#
  #####.#                           #######
  #......CJ                       NM..#...#
  ###.#.#                           #.###.#
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#
  #.....#        F   Q       P      #.#.#.#
  ###.###########.###.#######.#########.###
  #.....#...#.....#.......#...#.....#.#...#
  #####.#.###.#######.#######.###.###.#.#.#
  #.......#.......#.#.#.#.#...#...#...#.#.#
  #####.###.#####.#.#.#.#.###.###.#.###.###
  #.......#.....#.#...#...............#...#
  #############.#.#.###.###################
               A O F   N
               A A D   M                     "#,
        );
        let maze = get_recursive_maze(&input);
        assert_eq!(396, recursive_breadth_first_search(&maze));
    }
}
