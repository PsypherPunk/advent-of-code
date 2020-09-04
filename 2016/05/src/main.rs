use std::fs;

use md5::{Digest, Md5};

fn get_password(input: &str) -> String {
    let mut index = 0;
    let mut password = Vec::new();

    loop {
        let mut hasher = Md5::new();
        hasher.update(input.trim().as_bytes());
        hasher.update(index.to_string().as_bytes());

        let result = &hasher.finalize()[..];
        let result_hex = hex::encode(result);

        if result_hex.starts_with("00000") {
            password.push(result_hex.chars().nth(5).unwrap());
            if password.len() == 8 {
                break;
            }
        }
        index += 1;
    }

    password.iter().collect()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "Given the actual Door ID, what is the password? {}",
        get_password(&input),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abc() {
        let input = "abc";

        assert_eq!("18f47a30", get_password(&input));
    }
}
