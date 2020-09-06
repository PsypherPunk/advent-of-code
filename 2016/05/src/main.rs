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

        if result[..2] == [0, 0] && (result[2] & 0xF0) >> 4 == 0 {
            let result_hex = hex::encode(result);
            password.push(result_hex.chars().nth(5).unwrap());
            if password.len() == 8 {
                break;
            }
        }
        index += 1;
    }

    password.iter().collect()
}

fn get_better_password(input: &str) -> String {
    let mut index = 0;
    let mut password: [Option<char>; 8] = [None, None, None, None, None, None, None, None];

    loop {
        let mut hasher = Md5::new();
        hasher.update(input.trim().as_bytes());
        hasher.update(index.to_string().as_bytes());

        let result = &hasher.finalize()[..];

        if result[..2] == [0, 0] && (result[2] & 0xF0) >> 4 == 0 {
            let position = (result[2] & 0x0F) as usize;
            if position <= 7 && password[position].is_none() {
                let result_hex = hex::encode(result);
                password[position] = Some(result_hex.chars().nth(6).unwrap());
                if password.iter().all(|x| x.is_some()) {
                    break;
                }
            }
        }
        index += 1;
    }

    password.iter().map(|o| o.unwrap()).collect()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "Given the actual Door ID, what is the password? {}",
        get_password(&input),
    );

    println!(
        "Given the actual Door ID and this new method, what is the password? {}",
        get_better_password(&input),
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

    #[test]
    fn test_05ace8e3() {
        let input = "abc";

        assert_eq!("05ace8e3", get_better_password(&input));
    }
}
