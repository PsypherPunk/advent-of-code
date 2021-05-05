use std::fs;
use std::str::FromStr;

use ::day19::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let mut routing_diagram = RoutingDiagram::from_str(&input).unwrap();
    routing_diagram.follow_diagram();

    println!(
        "What letters will it see…? {}",
        routing_diagram.get_letters(),
    );

    println!(
        "How many steps…? {}",
        routing_diagram.get_steps(),
    );
}
