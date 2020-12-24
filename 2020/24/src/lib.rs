use std::collections::HashMap;
use std::str::FromStr;

use num::complex::Complex;

type Directions = HashMap<Vec<char>, Complex<isize>>;

enum Tile {
    Black,
    White,
}

pub struct Lobby {
    tiles: HashMap<Complex<isize>, Tile>,
}

fn get_directions() -> Directions {
    vec![
        (vec!['n', 'e'], Complex::new(1, -1)),
        (vec!['e'], Complex::new(1, 0)),
        (vec!['s', 'e'], Complex::new(0, 1)),
        (vec!['s', 'w'], Complex::new(-1, 1)),
        (vec!['w'], Complex::new(-1, 0)),
        (vec!['n', 'w'], Complex::new(0, -1)),
    ]
    .into_iter()
    .collect()
}

impl FromStr for Lobby {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let directions = get_directions();

        let mut tiles = HashMap::new();

        s.trim().lines().for_each(|line| {
            let mut location = Complex::new(0, 0);
            let mut chars = line.chars();

            while let Some(c) = chars.next() {
                let direction = match c {
                    'e' | 'w' => directions.get(&vec![c]).unwrap(),
                    'n' | 's' => directions.get(&vec![c, chars.next().unwrap()]).unwrap(),
                    _ => panic!(r#"¯\_(ツ)_/¯"#),
                };
                location += direction;
            }
            let entry = tiles.entry(location).or_insert(Tile::White);
            *entry = match entry {
                Tile::White => Tile::Black,
                Tile::Black => Tile::White,
            };
        });

        Ok(Self { tiles })
    }
}

impl Lobby {
    pub fn get_black_tile_count(&self) -> usize {
        self.tiles
            .values()
            .filter(|tile| matches!(tile, Tile::Black))
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew"#;

    #[test]
    fn test_reference() {
        let lobby = Lobby::from_str("nwwswee").unwrap();

        assert_eq!(1, lobby.get_black_tile_count());
    }

    #[test]
    fn test_reference_invert() {
        let lobby = Lobby::from_str(
            r#"nwwswee
nwwswee"#,
        )
        .unwrap();

        assert_eq!(0, lobby.get_black_tile_count());
    }

    #[test]
    fn test_part_one() {
        let lobby = Lobby::from_str(&INPUT).unwrap();

        assert_eq!(10, lobby.get_black_tile_count());
    }
}
