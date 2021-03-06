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

#[allow(clippy::needless_range_loop)]
fn build_image(width: usize, height: usize, data: &str) -> Vec<Vec<Vec<u32>>> {
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

fn render_image(width: usize, height: usize, image: &[Vec<Vec<u32>>]) -> Vec<Vec<Option<u32>>> {
    let mut render: Vec<Vec<Option<u32>>> = vec![vec![None; width]; height];

    for layer in image.iter() {
        for (y, row) in layer.iter().enumerate() {
            for (x, pixel) in row.iter().enumerate() {
                match render[y][x] {
                    Some(_) => {}
                    None => {
                        render[y][x] = match *pixel {
                            0 => Some(0),
                            1 => Some(1),
                            2 => None,
                            _ => panic!("Oops!"),
                        }
                    }
                }
            }
        }
    }

    render
}

fn draw_image(render: &[Vec<Option<u32>>]) {
    for row in render.iter() {
        for pixel in row.iter() {
            let out = match pixel {
                Some(0) => '\u{25a0}',
                Some(1) => '\u{25a1}',
                _ => panic!("Oops!"),
            };
            print!("{}", out);
        }
        println!();
    }
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
    println!(
        "…what is the number of 1 digits multiplied by the number of 2 digits? {}",
        one_count * two_count
    );
    let render = render_image(25, 6, &image);
    draw_image(&render);
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
        );
    }

    #[test]
    fn test_0222112222120000() {
        let image_data = String::from("0222112222120000");
        let width = 2;
        let height = 2;
        let image = build_image(width, height, &image_data);
        assert_eq!(
            image,
            vec![
                vec![vec![0, 2], vec![2, 2]],
                vec![vec![1, 1], vec![2, 2]],
                vec![vec![2, 2], vec![1, 2]],
                vec![vec![0, 0], vec![0, 0]],
            ],
        );
        assert_eq!(
            render_image(width, height, &image),
            vec![vec![Some(0), Some(1)], vec![Some(1), Some(0)]],
        );
    }
}
