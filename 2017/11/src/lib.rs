use std::cmp;

use num::complex::Complex;

pub fn read_path(input: &str) -> (Complex<isize>, isize) {
    input
        .trim()
        .split(',')
        .fold((Complex::new(0, 0), 0), |(position, max), step| {
            let position = position
                + match step {
                    "nw" => Complex::new(-1, 0),
                    "n" => Complex::new(0, -1),
                    "ne" => Complex::new(1, -1),
                    "se" => Complex::new(1, 0),
                    "s" => Complex::new(0, 1),
                    "sw" => Complex::new(-1, 1),
                    _ => panic!(r#"¯\_(ツ)_/¯"#),
                };

            (position, cmp::max(get_steps(position), max))
        })
}

pub fn get_steps(position: Complex<isize>) -> isize {
    (position.re.abs() + position.im.abs() + (position.re + position.im).abs()) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(3, get_steps(read_path("ne,ne,ne").0));
        assert_eq!(0, get_steps(read_path("ne,ne,sw,sw").0));
        assert_eq!(2, get_steps(read_path("ne,ne,s,s").0));
        assert_eq!(3, get_steps(read_path("se,sw,se,sw,sw").0));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(3, read_path("ne,ne,ne").1);
        assert_eq!(2, read_path("ne,ne,sw,sw").1);
        assert_eq!(2, read_path("ne,ne,s,s").1);
        assert_eq!(3, read_path("se,sw,se,sw,sw").1);
    }
}
