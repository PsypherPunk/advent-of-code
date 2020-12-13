use std::str::FromStr;

use ring_algorithm::chinese_remainder_theorem;

pub struct Notes {
    earliest_timestamp: isize,
    bus_ids: Vec<Option<isize>>,
}

impl FromStr for Notes {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (earliest_timestamp, bus_ids) = match s.trim().lines().collect::<Vec<&str>>()[..] {
            [a, b, ..] => (a, b),
            _ => unreachable!(),
        };
        let earliest_timestamp = earliest_timestamp.parse().unwrap();
        let bus_ids = bus_ids
            .split(',')
            .map(|bus_id| match bus_id {
                "x" => None,
                id => Some(id.parse().unwrap()),
            })
            .collect();

        Ok(Notes {
            earliest_timestamp,
            bus_ids,
        })
    }
}

impl Notes {
    pub fn get_earliest_bus_wait(&self) -> (isize, isize) {
        self.bus_ids
            .iter()
            .filter(|bus_id| bus_id.is_some())
            .map(|bus_id| {
                let bus_id = bus_id.unwrap();
                (bus_id, bus_id - (self.earliest_timestamp % bus_id))
            })
            .min_by(|(_, a), (_, b)| a.cmp(&b))
            .unwrap()
    }

    pub fn get_earliest_timestamp_with_offset(&self) -> isize {
        let bus_ids = self
            .bus_ids
            .iter()
            .filter(|bus_id| bus_id.is_some())
            .map(|bus_id| bus_id.unwrap())
            .collect::<Vec<isize>>();
        let offsets = self
            .bus_ids
            .iter()
            .enumerate()
            .filter(|(_, bus_id)| bus_id.is_some())
            .map(|(offset, _)| offset as isize)
            .collect::<Vec<isize>>();

        chinese_remainder_theorem(&offsets, &bus_ids).unwrap().abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"939
7,13,x,x,59,x,31,19"#;

    #[test]
    fn test_part_one() {
        let notes = Notes::from_str(&INPUT).unwrap();

        let (bus_id, wait) = notes.get_earliest_bus_wait();

        assert_eq!(295, bus_id * wait);
    }

    #[test]
    fn test_part_two() {
        let notes = Notes::from_str(&INPUT).unwrap();

        assert_eq!(1068781, notes.get_earliest_timestamp_with_offset());
    }
}
