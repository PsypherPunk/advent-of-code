use std::fs::File;
use std::io::prelude::*;

fn read_input() -> String {
    let filename = "input.txt";
    match File::open(filename) {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            content
        }
        Err(error) => {
            panic!("Error opening file {}: {}", filename, error);
        }
    }
}

fn get_nonce(secret_key: String, prefix_count: usize) -> usize {
    for nonce in 1.. {
        let input = format!("{}{}", &secret_key, nonce.to_string().as_str());
        let digest = format!("{:x}", md5::compute(input.as_bytes()));
        if digest.starts_with(&"0".repeat(prefix_count)) {
            return nonce;
        }
    }
    0
}

fn main() {
    let input = read_input();

    let nonce = get_nonce(String::from(input.trim()), 5);
    println!("…five zeroes… {}", nonce);

    let nonce = get_nonce(String::from(input.trim()), 6);
    println!("…six zeroes… {}", nonce);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abcdef() {
        assert_eq!(get_nonce(String::from("abcdef"), 5), 609043);
    }

    #[test]
    fn test_pqrstuv() {
        assert_eq!(get_nonce(String::from("pqrstuv"), 5), 1048970);
    }
}
