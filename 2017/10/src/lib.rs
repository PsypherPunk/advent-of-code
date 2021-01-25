pub struct KnotHash {
    list: Vec<usize>,
    position: usize,
    skip_size: usize,
}

impl KnotHash {
    pub fn new(start: usize, end: usize) -> Self {
        Self {
            list: (start..=end).collect(),
            position: 0,
            skip_size: 0,
        }
    }

    fn apply_length(&mut self, length: usize) {
        let mut numbers = vec![0; length];

        (0..length).for_each(|i| {
            let index = (self.position + i) % self.list.len();
            numbers[i] = self.list[index];
        });

        numbers.reverse();

        (0..length).for_each(|i| {
            let index = (self.position + i) % self.list.len();
            self.list[index] = numbers[i];
        });

        self.position = (self.position + length + self.skip_size) % self.list.len();
        self.skip_size += 1;
    }

    pub fn apply_lengths(&mut self, lengths: &[usize]) {
        lengths.iter().for_each(|length| self.apply_length(*length));
    }

    pub fn get_product_of_first_two_numbers(&self) -> usize {
        self.list[0] * self.list[1]
    }
}

pub fn get_lengths(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(',')
        .map(|number| number.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_single() {
        let mut knot_hash = KnotHash::new(0, 4);

        knot_hash.apply_length(3);

        assert_eq!(vec![2, 1, 0, 3, 4], knot_hash.list,);
        assert_eq!(3, knot_hash.position);
        assert_eq!(1, knot_hash.skip_size);
    }

    #[test]
    fn test_part_one_double() {
        let mut knot_hash = KnotHash::new(0, 4);

        for length in 3..=4 {
            knot_hash.apply_length(length);
        }

        assert_eq!(vec![4, 3, 0, 1, 2], knot_hash.list,);
        assert_eq!(3, knot_hash.position);
        assert_eq!(2, knot_hash.skip_size);
    }

    #[test]
    fn test_part_one_all() {
        let mut knot_hash = KnotHash::new(0, 4);

        for length in [3, 4, 1, 5].iter() {
            knot_hash.apply_length(*length);
        }

        assert_eq!(vec![3, 4, 2, 1, 0], knot_hash.list,);
        assert_eq!(4, knot_hash.position);
        assert_eq!(4, knot_hash.skip_size);
    }
}
