#[derive(Clone, Copy, Debug)]
pub struct Position {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Clone, Copy, Debug)]
pub struct Nanobot {
    position: Position,
    radius: usize,
}

impl Default for Position {
    fn default() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }
}

impl Position {
    pub fn get_manhattan_distance(&self, other: &Position) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()) as usize
    }
}

impl Nanobot {
    fn overlaps(&self, other: &Nanobot) -> bool {
        self.position.get_manhattan_distance(&other.position) <= self.radius + other.radius
    }
}

peg::parser! {
    pub grammar wrist_device() for str {
        rule _() = [' ' | '\n']*

        rule integer() -> isize
            = n:$("-"* ['0'..='9']+) {? n.parse().or(Err("Invalid integer.")) }

        rule position() -> Position
            = "pos=<" x:integer() "," y:integer() "," z:integer() ">"
                { Position { x, y, z } }

        rule radius() -> usize
            = "r=" radius:integer()
                { radius as usize }

        rule nanobot() -> Nanobot
            = position:position() ", " radius:radius()
                { Nanobot { position, radius } }

        pub rule nanobots() -> Vec<Nanobot>
            = nanobots:nanobot() ++ _
                { nanobots }
    }
}

pub fn get_strongest_nanobot(nanobots: &[Nanobot]) -> &Nanobot {
    nanobots
        .iter()
        .max_by(|a, b| a.radius.cmp(&b.radius))
        .unwrap()
}

pub fn get_nanobots_in_range(nanobot: &Nanobot, nanobots: &[Nanobot]) -> usize {
    nanobots
        .iter()
        .filter(|other| nanobot.position.get_manhattan_distance(&other.position) <= nanobot.radius)
        .count()
}

pub fn get_teleportation_position_distance(nanobots: &[Nanobot]) -> usize {
    let most_overlapping_nanobots = nanobots
        .iter()
        .enumerate()
        .map(|(i, nanobot)| {
            let mut overlapping: Vec<Nanobot> = Vec::new();

            for other in nanobots
                .iter()
                .skip(i)
                .filter(|other| other.overlaps(nanobot))
            {
                if overlapping.iter().all(|b2| b2.overlaps(other)) {
                    overlapping.push(*other);
                }
            }

            overlapping
        })
        .max_by_key(|overlapping_nanobots| overlapping_nanobots.len())
        .unwrap();

    most_overlapping_nanobots
        .iter()
        .map(|nanobot| {
            nanobot
                .position
                .get_manhattan_distance(&Position::default())
                .saturating_sub(nanobot.radius)
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1"#;

    #[test]
    fn test_parser() {
        let nanobots = wrist_device::nanobots(&INPUT).unwrap();

        assert_eq!(9, nanobots.len());
        assert_eq!(4, nanobots[0].radius);
    }

    #[test]
    fn test_get_strongest() {
        let nanobots = wrist_device::nanobots(&INPUT).unwrap();

        let strongest = get_strongest_nanobot(&nanobots);

        assert_eq!(4, strongest.radius);
    }

    #[test]
    fn test_manhattan_distance() {
        let nanobots = wrist_device::nanobots(&INPUT).unwrap();

        let strongest = get_strongest_nanobot(&nanobots);
        let distances = nanobots
            .iter()
            .map(|nanobot| strongest.position.get_manhattan_distance(&nanobot.position))
            .collect::<Vec<_>>();

        assert_eq!(vec![0, 1, 4, 2, 5, 3, 3, 4, 5], distances);
    }

    #[test]
    fn test_part_one() {
        let nanobots = wrist_device::nanobots(&INPUT).unwrap();

        let strongest = get_strongest_nanobot(&nanobots);

        assert_eq!(7, get_nanobots_in_range(&strongest, &nanobots));
    }

    #[test]
    fn test_part_two() {
        let input = r#"pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5"#;

        let nanobots = wrist_device::nanobots(&input).unwrap();

        assert_eq!(36, get_teleportation_position_distance(&nanobots))
    }
}
