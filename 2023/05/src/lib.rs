#[derive(Debug)]
pub struct MapRange {
    destination_range_start: usize,
    source_range_start: usize,
    range_length: usize,
}

pub struct Map {
    ranges: Vec<MapRange>,
}

pub struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<Map>,
}

impl Almanac {
    fn get_seed_location_number(&self, seed: usize) -> usize {
        self.maps.iter().fold(seed, |current, map| {
            map.ranges
                .iter()
                .find_map(|range| range.map_seed(current))
                .unwrap_or(current)
        })
    }

    fn get_optimum_located_seeds(&self) -> Vec<usize> {
        self.maps.iter().rev().fold(vec![], |mut acc, map| {
            acc = acc
                .iter()
                .map(|mapping| {
                    map.ranges
                        .iter()
                        .find_map(|range| range.dees_pam(*mapping))
                        .unwrap_or(*mapping)
                })
                .collect::<Vec<_>>();

            let starts = map
                .ranges
                .iter()
                .map(|range| range.source_range_start)
                .collect::<Vec<_>>();

            acc.extend(starts);

            acc
        })
    }
}

impl MapRange {
    fn map_seed(&self, seed: usize) -> Option<usize> {
        if seed >= self.source_range_start && seed < self.source_range_start + self.range_length {
            let offset = seed - self.source_range_start;
            Some(self.destination_range_start + offset)
        } else {
            None
        }
    }

    fn dees_pam(&self, seed: usize) -> Option<usize> {
        if seed >= self.destination_range_start
            && seed < self.destination_range_start + self.range_length
        {
            let offset = seed - self.destination_range_start;
            Some(self.source_range_start + offset)
        } else {
            None
        }
    }
}

peg::parser! {
    pub grammar almanac() for str {
        rule _() = [' ' | '\n']*

        rule integer() -> usize
            = n:$(['0'..='9']+)
                {? n.parse().or(Err("invalid integer")) }

        rule word() -> &'input str
            = w:$(['a'..='z']+)
                { w }

        rule range() -> MapRange
            = destination_range_start:integer() _
              source_range_start:integer() _
              range_length:integer()
              {
                MapRange {
                    destination_range_start,
                    source_range_start,
                    range_length,
                }
              }

        rule map() -> Map
            = word() ++ "-"
              " map:" _
              ranges:range() ++ _
              {
                Map {
                    ranges
                }
              }

        rule seeds() -> Vec<usize>
            = "seeds: "
              seeds:integer() ++ _
              {
                seeds
              }

        pub rule almanac() -> Almanac
            = seeds:seeds()
              _
              maps:map() ++ _
              {
                  Almanac {
                      seeds,
                      maps,
                  }
              }


    }
}

pub fn get_part_one(input: &str) -> Result<usize, String> {
    let almanac = almanac::almanac(input.trim()).map_err(|e| e.to_string())?;

    let lowest_location = almanac
        .seeds
        .iter()
        .map(|seed| almanac.get_seed_location_number(*seed))
        .min()
        .ok_or("could not find lowest location".to_owned())?;

    Ok(lowest_location)
}

pub fn get_part_two(input: &str) -> Result<usize, String> {
    let almanac = almanac::almanac(input.trim()).map_err(|e| e.to_string())?;

    let seed_ranges = almanac
        .seeds
        .chunks(2)
        .map(|pair| pair[0]..(pair[0] + pair[1]))
        .collect::<Vec<_>>();

    let optimium_located_seeds = almanac.get_optimum_located_seeds();

    let min = optimium_located_seeds
        .iter()
        .filter(|seed| {
            seed_ranges
                .iter()
                .any(|seed_range| seed_range.contains(seed))
        })
        .map(|&seed| almanac.get_seed_location_number(seed))
        .min()
        .ok_or("could not find lowest location".to_owned())?;

    Ok(min)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(35), get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(46), get_part_two(INPUT));
    }
}
