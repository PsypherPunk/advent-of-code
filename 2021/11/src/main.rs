use std::fs;

use argh::FromArgs;

use ::day11::*;

#[derive(FromArgs)]
#[argh(description = "Advent of Code 2021, day 11")]
struct Args {
    #[argh(switch, short = 'd', description = "whether to display animation")]
    display: bool,
}

fn main() -> Result<(), String> {
    let args: Args = argh::from_env();
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    println!(
        "How many total flashes are there after 100 steps? {}",
        get_part_one(&input, args.display)?,
    );

    println!(
        "What is the first step during which all octopuses flash? {}",
        get_part_two(&input, args.display)?,
    );

    Ok(())
}
