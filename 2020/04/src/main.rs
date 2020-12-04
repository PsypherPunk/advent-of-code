use std::collections::HashMap;
use std::fs;

const MANDATORY_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

type Passport<'a> = HashMap<&'a str, &'a str>;

trait ValidPassport {
    fn has_mandatory_fields(&self) -> bool;

    fn has_valid_byr(&self) -> bool;

    fn has_valid_iyr(&self) -> bool;

    fn has_valid_eyr(&self) -> bool;

    fn has_valid_hgt(&self) -> bool;

    fn has_valid_hcl(&self) -> bool;

    fn has_valid_ecl(&self) -> bool;

    fn has_valid_pid(&self) -> bool;

    fn is_valid(&self) -> bool {
        self.has_mandatory_fields()
            && self.has_valid_byr()
            && self.has_valid_iyr()
            && self.has_valid_eyr()
            && self.has_valid_hgt()
            && self.has_valid_hcl()
            && self.has_valid_ecl()
            && self.has_valid_pid()
    }
}

impl ValidPassport for HashMap<&str, &str> {
    fn has_mandatory_fields(&self) -> bool {
        MANDATORY_FIELDS
            .iter()
            .map(|field| self.contains_key(field))
            .all(|x| x)
    }

    fn has_valid_byr(&self) -> bool {
        let byr = self.get("byr").unwrap().parse::<usize>().unwrap();
        byr >= 1920 && byr <= 2002
    }

    fn has_valid_iyr(&self) -> bool {
        let iyr = self.get("iyr").unwrap().parse::<usize>().unwrap();
        iyr >= 2010 && iyr <= 2020
    }

    fn has_valid_eyr(&self) -> bool {
        let eyr = self.get("eyr").unwrap().parse::<usize>().unwrap();
        eyr >= 2020 && eyr <= 2030
    }

    fn has_valid_hgt(&self) -> bool {
        let hgt = self.get("hgt").unwrap();
        let num = hgt[..(hgt.len() - 2)].parse::<usize>();

        if hgt.ends_with("cm") {
            if let Ok(data) = num {
                return data >= 150 && data <= 193;
            }
        }

        if hgt.ends_with("in") {
            if let Ok(data) = num {
                return data >= 59 && data <= 76;
            }
        }

        false
    }

    fn has_valid_hcl(&self) -> bool {
        let hcl = self.get("hcl").unwrap();
        let hex = usize::from_str_radix(&hcl[1..], 16);

        hcl.starts_with('#') && hex.is_ok()
    }

    fn has_valid_ecl(&self) -> bool {
        let ecl = self.get("ecl").unwrap();

        ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(ecl)
    }

    fn has_valid_pid(&self) -> bool {
        let pid = self.get("pid").unwrap();

        pid.len() == 9 && pid.parse::<usize>().is_ok()
    }
}

fn get_valid_field_count(input: &str) -> usize {
    get_passports(&input)
        .iter()
        .filter(|passport| passport.has_mandatory_fields())
        .count()
}

fn get_valid_passport_count(input: &str) -> usize {
    get_passports(&input)
        .iter()
        .filter(|passport| passport.is_valid())
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
        get_valid_field_count(&input),
    );

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

        assert_eq!(2, get_valid_field_count(&input));
    }

    #[test]
    fn test_part_two_invalid() {
        let input = r#"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"#;

        assert_eq!(0, get_valid_passport_count(&input));
    }

    #[test]
    fn test_part_two_valid() {
        let input = r#"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"#;

        assert_eq!(4, get_valid_passport_count(&input));
    }
}
