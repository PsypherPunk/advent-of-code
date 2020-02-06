use std::fs;

use regex::Regex;
use std::collections::HashMap;

static MFCSAM: &str = r#"children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1"#;

fn get_mfcsam() -> HashMap<String, u8> {
    MFCSAM
        .lines()
        .map(|line| {
            let detection = line.split_whitespace().collect::<Vec<&str>>();
            (
                detection[0][..(detection[0].len() - 1)].to_string(),
                detection[1].parse::<u8>().unwrap(),
            )
        })
        .collect()
}

fn get_sues(input: &str) -> HashMap<String, HashMap<String, u8>> {
    let re = Regex::new(r#"^Sue (\d+): (\w+): (\d+), (\w+): (\d+), (\w+): (\d+)$"#).unwrap();

    input
        .trim()
        .lines()
        .map(|line| {
            let captures = re.captures(line).unwrap();
            let mut facts = HashMap::new();
            facts.insert(captures[2].to_string(), captures[3].parse::<u8>().unwrap());
            facts.insert(captures[4].to_string(), captures[5].parse::<u8>().unwrap());
            facts.insert(captures[6].to_string(), captures[7].parse::<u8>().unwrap());
            (captures[1].to_string(), facts)
        })
        .collect()
}

fn get_sue(input: &str) -> String {
    let mfcsam = get_mfcsam();
    let sues = get_sues(&input);

    let detected = sues
        .iter()
        .find(|&(_, facts)| {
            facts
                .iter()
                .map(|(k, v)| mfcsam.get(k).unwrap() == v)
                .all(|x| x)
        })
        .unwrap();

    detected.0.clone()
}

fn get_retroencabulated_sue(input: &str) -> String {
    let mfcsam = get_mfcsam();
    let sues = get_sues(&input);

    let cats = String::from("cats");
    let trees = String::from("trees");
    let pomeranians = String::from("pomeranians");
    let goldfish = String::from("goldfish");

    let detected = sues
        .iter()
        .find(|&(_, facts)| {
            facts
                .iter()
                .map(|(k, v)| match k {
                    k if *k == cats => v > mfcsam.get(k).unwrap(),
                    k if *k == trees => v > mfcsam.get(k).unwrap(),
                    k if *k == pomeranians => v < mfcsam.get(k).unwrap(),
                    k if *k == goldfish => v < mfcsam.get(k).unwrap(),
                    _ => mfcsam.get(k).unwrap() == v,
                })
                .all(|x| x)
        })
        .unwrap();

    detected.0.clone()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What is the number of the Sue that got you the gift? {}",
        get_sue(&input),
    );

    println!(
        "What is the number of the real Aunt Sue? {}",
        get_retroencabulated_sue(&input),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
