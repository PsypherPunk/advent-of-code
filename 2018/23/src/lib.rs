struct Position {
    x: isize,
    y: isize,
    z: isize,
}

pub struct Nanobot {
    position: Position,
    radius: usize,
}

impl Nanobot {
    fn get_manhattan_distance(&self, other: &Nanobot) -> usize {
        ((self.position.x - other.position.x).abs()
            + (self.position.y - other.position.y).abs()
            + (self.position.z - other.position.z).abs()) as usize
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
        .filter(|other| nanobot.get_manhattan_distance(&other) <= nanobot.radius)
        .count()
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
            .map(|nanobot| strongest.get_manhattan_distance(&nanobot))
            .collect::<Vec<_>>();

        assert_eq!(vec![0, 1, 4, 2, 5, 3, 3, 4, 5], distances);
    }

    #[test]
    fn test_part_one() {
        let nanobots = wrist_device::nanobots(&INPUT).unwrap();

        let strongest = get_strongest_nanobot(&nanobots);

        assert_eq!(7, get_nanobots_in_range(&strongest, &nanobots));
    }
}
