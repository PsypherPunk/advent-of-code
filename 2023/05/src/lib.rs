use rayon::prelude::*;

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
    pub fn get_seed_location_number(&self, seed: usize) -> usize {
        self.maps.iter().fold(seed, |current, map| {
            match map.ranges.iter().find(|range| {
                range.source_range_start <= current
                    && current <= range.source_range_start + range.range_length
            }) {
                Some(range) => range.destination_range_start + (current - range.source_range_start),
                None => current,
            }
        })
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

// TODO: yeah, this is bad.
pub fn get_part_two(input: &str) -> Result<usize, String> {
    let almanac = almanac::almanac(input.trim()).map_err(|e| e.to_string())?;

    let lowest_location = almanac
        .seeds
        .chunks(2)
        .flat_map(|pair| pair[0]..(pair[0] + pair[1]))
        .par_bridge()
        .map(|seed| almanac.get_seed_location_number(seed))
        .min()
        .ok_or("could not find lowest location".to_owned())?;

    Ok(lowest_location)
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
