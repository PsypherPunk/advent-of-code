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

    pub fn generate_sparse_hash(&mut self, lengths: &[usize]) {
        (0..64).for_each(|_| self.apply_lengths(lengths));
    }

    pub fn get_dense_hash(&self) -> String {
        let dense_hash = self
            .list
            .chunks(16)
            .map(|chunk| chunk.iter().fold(0, |acc, number| acc ^ number))
            .collect::<Vec<_>>();

        dense_hash
            .iter()
            .map(|number| format!("{:02x}", number))
            .collect::<Vec<_>>()
            .join("")
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

pub fn get_ascii_lengths(input: &str) -> Vec<usize> {
    let mut ascii_codes = input.trim().chars().map(|c| c as usize).collect::<Vec<_>>();
    ascii_codes.extend(vec![17, 31, 73, 47, 23]);

    ascii_codes
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

    #[test]
    fn test_part_two_ascii_lengths() {
        let input = "1,2,3";

        assert_eq!(
            vec![49, 44, 50, 44, 51, 17, 31, 73, 47, 23],
            get_ascii_lengths(&input),
        );
    }

    #[test]
    fn test_part_two_empty_string() {
        let input = "";

        let mut knot_hash = KnotHash::new(0, 255);
        knot_hash.generate_sparse_hash(&get_ascii_lengths(&input));
        knot_hash.get_dense_hash();

        assert_eq!(
            "a2582a3a0e66e6e86e3812dcb672a272",
            knot_hash.get_dense_hash(),
        );
    }

    #[test]
    fn test_part_two_aoc_2017() {
        let input = "AoC 2017";

        let mut knot_hash = KnotHash::new(0, 255);
        knot_hash.generate_sparse_hash(&get_ascii_lengths(&input));
        knot_hash.get_dense_hash();

        assert_eq!(
            "33efeb34ea91902bb2f59c9920caa6cd",
            knot_hash.get_dense_hash(),
        );
    }
}
