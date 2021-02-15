pub struct Spinlock {
    repetitions: usize,
    steps: usize,
    buffer: Vec<usize>,
    position: usize,
}

impl Spinlock {
    fn new(steps: usize, repetitions: usize) -> Self {
        Self {
            repetitions,
            steps,
            buffer: vec![0],
            position: 0,
        }
    }

    fn spin(&mut self) {
        for i in 1..=self.repetitions {
            self.position = (self.position + self.steps) % self.buffer.len();
            self.buffer.insert(self.position + 1, i);
            self.position += 1;
        }
    }

    pub fn from_str(input: &str, repetitions: usize) -> Self {
        Self::new(input.trim().parse().unwrap(), repetitions)
    }

    pub fn get_value_after_last_inserted(&mut self) -> usize {
        self.spin();
        self.buffer[self.position + 1]
    }

    pub fn get_value_after_zero(&mut self) -> usize {
        (1..self.repetitions)
            .filter(|buffer_len| {
                self.position = (self.position + self.steps) % buffer_len + 1;
                self.position == 1
            })
            .last()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let mut spinlock = Spinlock::new(3, 2017);

        assert_eq!(638, spinlock.get_value_after_last_inserted());
    }
}
