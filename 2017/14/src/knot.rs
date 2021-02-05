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

    fn apply_lengths(&mut self, lengths: &[usize]) {
        lengths.iter().for_each(|length| self.apply_length(*length));
    }

    fn generate_sparse_hash(&mut self, lengths: &[usize]) {
        (0..64).for_each(|_| self.apply_lengths(lengths));
    }

    fn get_dense_hash(&self) -> String {
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

    pub fn hash(&mut self, input: &str) -> String {
        let mut lengths = input.trim().chars().map(|c| c as usize).collect::<Vec<_>>();
        lengths.extend(vec![17, 31, 73, 47, 23]);

        self.generate_sparse_hash(&lengths);
        self.get_dense_hash()
    }
}
