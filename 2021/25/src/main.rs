use std::fs;

use argh::FromArgs;

use ::day25::*;

#[derive(FromArgs)]
#[argh(description = "Advent of Code 2021, day 25")]
struct Args {
    #[argh(switch, short = 'd', description = "whether to display animation")]
    display: bool,
}

fn main() {
    let args: Args = argh::from_env();
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "What is the first step on which no sea cucumbers move? {}",
        get_part_one(&input, args.display),
    );
}
