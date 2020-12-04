use std::collections::HashMap;
use std::fs;

const MANDATORY_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

type Passport<'a> = HashMap<&'a str, &'a str>;

trait ValidPassport {
    fn has_mandatory_fields(&self) -> bool;
}

impl ValidPassport for HashMap<&str, &str> {
    fn has_mandatory_fields(&self) -> bool {
        MANDATORY_FIELDS
            .iter()
            .map(|field| self.contains_key(field))
            .all(|x| x)
    }
}

fn get_valid_passport_count(input: &str) -> usize {
    get_passports(&input)
        .iter()
        .filter(|passport| passport.has_mandatory_fields())
        .count()
}

fn get_passports(input: &str) -> Vec<Passport> {
    input
        .trim()
        .split("\n\n")
        .map(|passport_lines| {
            passport_lines
                .split_whitespace()
                .map(|field| {
                    let field_data = field.split(':').collect::<Vec<&str>>();
                    (field_data[0], field_data[1])
                })
                .collect::<HashMap<&str, &str>>()
        })
        .collect()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "In your batch file, how many passports are valid? {}",
        get_valid_passport_count(&input),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"#;

        assert_eq!(2, get_valid_passport_count(&input));
    }
}
