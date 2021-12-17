use std::io::{Read, Result as IoResult};
use std::str::FromStr;

struct Transmission {
    position: usize,
    bits: Vec<u8>,
}

impl FromStr for Transmission {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bits = s
            .trim()
            .chars()
            .flat_map(|c| {
                let n = c.to_digit(16).unwrap() as u8;
                vec![n >> 3 & 1, n >> 2 & 1, n >> 1 & 1, n & 1]
            })
            .collect::<Vec<_>>();

        Ok(Transmission { bits, position: 0 })
    }
}

impl Read for Transmission {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        buf.copy_from_slice(&self.bits[self.position..self.position + buf.len()]);

        self.position += buf.len();

        Ok(buf.len())
    }
}

impl Transmission {
    fn get_number_from_bits(&mut self, bits: usize) -> u64 {
        let mut buf = vec![0; bits];

        self.read_exact(&mut buf).unwrap();

        buf.iter().fold(0, |a, b| (a << 1) | *b as u64)
    }
}

fn get_version(transmission: &mut Transmission) -> u64 {
    let mut version = transmission.get_number_from_bits(3);

    let type_id = transmission.get_number_from_bits(3);

    match type_id {
        4 => {
            let mut n = 0;
            let mut buf = [0; 5];
            loop {
                transmission.read_exact(&mut buf).unwrap();
                n = n << 4 | buf[1..].iter().fold(0, |a, b| a << 1 | b);
                if buf[0] == 0 {
                    return version;
                }
            }
        }
        _ => {
            let length_type_id = transmission.get_number_from_bits(1);
            match length_type_id {
                0 => {
                    let packets_len = transmission.get_number_from_bits(15);
                    let start = transmission.position;
                    while transmission.position < start + packets_len as usize {
                        version += get_version(transmission);
                    }
                }
                1 => {
                    let packet_count = transmission.get_number_from_bits(11);
                    for _ in 0..packet_count {
                        version += get_version(transmission);
                    }
                }
                _ => panic!(),
            }
        }
    }

    version
}

pub fn get_part_one(input: &str) -> usize {
    let mut transmission = Transmission::from_str(input).unwrap();

    get_version(&mut transmission) as usize
}

#[cfg(test)]
mod tests {
    use parameterized::parameterized;

    use super::*;

    #[parameterized(transmission = {
        "D2FE28",
        "8A004A801A8002F478",
        "620080001611562C8802118E34",
        "C0015000016115A2E0802F182340",
        "A0016C880162017C3686B18A3D4780",
    }, version_sum = {
        6, 16, 12, 23, 31,
    })]
    fn test_part_one(transmission: &str, version_sum: usize) {
        assert_eq!(version_sum, get_part_one(transmission));
    }
}
