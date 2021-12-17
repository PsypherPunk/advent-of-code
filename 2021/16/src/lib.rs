use std::io::{Read, Result as IoResult};
use std::str::FromStr;

struct Transmission {
    position: usize,
    bits: Vec<u8>,
}

#[derive(Debug)]
struct Packet {
    version: u64,
    values: Vec<u64>,
    type_id: u64,
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

impl Packet {
    fn get_value(&self) -> u64 {
        match self.type_id {
            0 => self.values.iter().sum(),
            1 => self.values.iter().product(),
            2 => *self.values.iter().min().unwrap(),
            3 => *self.values.iter().max().unwrap(),
            5 => (self.values[0] > self.values[1]) as u64,
            6 => (self.values[0] < self.values[1]) as u64,
            7 => (self.values[0] == self.values[1]) as u64,
            _ => panic!(),
        }
    }
}

fn get_packet(transmission: &mut Transmission) -> Packet {
    let version = transmission.get_number_from_bits(3);
    let type_id = transmission.get_number_from_bits(3);

    let mut packet = Packet {
        version,
        values: Vec::new(),
        type_id,
    };

    match type_id {
        4 => {
            let mut n = 0;
            let mut buf = [0; 5];
            loop {
                transmission.read_exact(&mut buf).unwrap();
                n = n << 4 | buf[1..].iter().fold(0, |a, b| a << 1 | *b as u64);
                if buf[0] == 0 {
                    packet.values.push(n);
                    return packet;
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
                        let sub = get_packet(transmission);
                        packet.version += sub.version;
                        packet.values.extend(sub.values);
                    }
                }
                1 => {
                    let packet_count = transmission.get_number_from_bits(11);
                    for _ in 0..packet_count {
                        let sub = get_packet(transmission);
                        packet.version += sub.version;
                        packet.values.extend(sub.values);
                    }
                }
                _ => panic!(),
            }
        }
    }

    packet.values = vec![packet.get_value()];

    packet
}

pub fn get_part_one(input: &str) -> u64 {
    let mut transmission = Transmission::from_str(input).unwrap();

    get_packet(&mut transmission).version
}

pub fn get_part_two(input: &str) -> u64 {
    let mut transmission = Transmission::from_str(input).unwrap();

    let packet = get_packet(&mut transmission);

    packet.values[0]
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
    fn test_part_one(transmission: &str, version_sum: u64) {
        assert_eq!(version_sum, get_part_one(transmission));
    }

    #[parameterized(transmission = {
        "C200B40A82",
        "04005AC33890",
        "880086C3E88112",
        "CE00C43D881120",
        "D8005AC2A8F0",
        "F600BC2D8F",
        "9C005AC2F8F0",
        "9C0141080250320F1802104A08",
    }, value = {
        3, 54, 7, 9, 1, 0, 0, 1,
    })]
    fn test_part_two(transmission: &str, value: u64) {
        assert_eq!(value, get_part_two(transmission));
    }
}
