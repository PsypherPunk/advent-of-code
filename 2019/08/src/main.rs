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

fn build_image(width: usize, height: usize, data: &String) -> Vec<Vec<Vec<u32>>> {
    let mut image: Vec<Vec<Vec<u32>>> = Vec::new();

    let mut pixels = data.trim().chars();

    'outer: loop {
        let mut layer: Vec<Vec<u32>> = vec![vec![0; width]; height];
        for y in 0..height {
            for x in 0..width {
                let digit = match pixels.next() {
                    Some(p) => p.to_digit(10).unwrap(),
                    None => break 'outer,
                };
                layer[y][x] = digit;
            }
        }
        image.push(layer);
    }

    image
}

fn main() {
    let input = read_input();
    let image = build_image(25, 6, &input);
    let layer_with_fewest_zeroes = image
        .iter()
        .min_by_key(|layer| {
            layer
                .iter()
                .flat_map(|row| row.iter())
                .filter(|pixel| **pixel == 0)
                .count()
        })
        .unwrap();
    let one_count = layer_with_fewest_zeroes
        .iter()
        .flat_map(|row| row.iter())
        .filter(|pixel| **pixel == 1)
        .count();
    let two_count = layer_with_fewest_zeroes
        .iter()
        .flat_map(|row| row.iter())
        .filter(|pixel| **pixel == 2)
        .count();
    println!("…what is the number of 1 digits multiplied by the number of 2 digits? {}", one_count * two_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_123456789012() {
        let image_data = String::from("123456789012");
        let width = 3;
        let height = 2;
        let image = build_image(width, height, &image_data);
        assert_eq!(
            image,
            vec![
                vec![vec![1, 2, 3], vec![4, 5, 6]],
                vec![vec![7, 8, 9], vec![0, 1, 2]],
            ],
        )
    }
}
