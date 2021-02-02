use num::complex::Complex;

pub fn read_path(input: &str) -> Complex<isize> {
    input
        .trim()
        .split(',')
        .fold(Complex::new(0, 0), |position, step| {
            position + match step {
                "nw" => Complex::new(-1, 0),
                "n" => Complex::new(0, -1),
                "ne" => Complex::new(1, -1),
                "se" => Complex::new(1, 0),
                "s" => Complex::new(0, 1),
                "sw" => Complex::new(-1, 1),
                _ => panic!(r#"¯\_(ツ)_/¯"#),
            }
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
        assert_eq!(3, get_steps(read_path("ne,ne,ne")));
        assert_eq!(0, get_steps(read_path("ne,ne,sw,sw")));
        assert_eq!(2, get_steps(read_path("ne,ne,s,s")));
        assert_eq!(3, get_steps(read_path("se,sw,se,sw,sw")));
    }
}
