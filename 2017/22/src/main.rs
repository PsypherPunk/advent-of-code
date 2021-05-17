use std::fs;
use std::str::FromStr;

use ::day22::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let mut cluster = Cluster::from_str(&input).unwrap();
    cluster.bursts_of_activity(10_000);

    println!(
        "…how many bursts cause a node to become infected? {}",
        cluster.get_infections(),
    );
}
