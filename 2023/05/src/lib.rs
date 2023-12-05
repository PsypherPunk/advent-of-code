pub struct Range {
    destination_range_start: usize,
    source_range_start: usize,
    range_length: usize,
}

pub struct Map {
    ranges: Vec<Range>,
}

pub struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<Map>,
}

impl Almanac {
    // TODO: this could be a `fold()`…?
    pub fn get_location_number(&self, seed: usize) -> usize {
        let mut current_value = seed;

        for category in self.maps.iter() {
            for range in &category.ranges {
                if range.source_range_start <= current_value
                    && current_value <= range.source_range_start + range.range_length
                {
                    current_value =
                        range.destination_range_start + (current_value - range.source_range_start);
                    break;
                }
            }
        }

        current_value
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

        rule range() -> Range
            = destination_range_start:integer() _
              source_range_start:integer() _
              range_length:integer()
              {
                Range {
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
        .map(|seed| almanac.get_location_number(*seed))
        .min()
        .ok_or("could not find lowest location".to_owned())?;

    Ok(lowest_location)
}

pub fn get_part_two(_input: &str) -> Result<usize, String> {
    Ok(0)
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
        assert_eq!(Ok(2), get_part_two(INPUT));
    }
}
