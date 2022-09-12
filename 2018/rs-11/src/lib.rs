use std::array;

type Grid = [[isize; 300]; 300];

fn get_power_level(x: isize, y: isize, grid_serial_number: isize) -> isize {
    let rack_id = x + 10;

    let mut power_level = rack_id * y;
    power_level += grid_serial_number;
    power_level *= rack_id;
    power_level /= 100;
    power_level %= 10;
    power_level -= 5;

    power_level
}

pub fn get_part_one(input: &str) -> Result<String, String> {
    let grid_serial_number = input
        .trim()
        .parse::<isize>()
        .map_err(|err| err.to_string())?;

    let grid: Grid = array::from_fn(|y| {
        array::from_fn(|x| get_power_level(x as isize, y as isize, grid_serial_number))
    });

    let (x, y, _) = (1..299)
        .flat_map(|y| {
            (1..299).map(move |x| {
                let total_power = [
                    (x - 1, y - 1),
                    (x, y - 1),
                    (x + 1, y - 1),
                    (x - 1, y),
                    (x, y),
                    (x + 1, y),
                    (x - 1, y + 1),
                    (x, y + 1),
                    (x + 1, y + 1),
                ]
                .map(|(x, y)| grid[y][x])
                .into_iter()
                .sum::<isize>();

                (x - 1, y - 1, total_power)
            })
        })
        .max_by(|(_, _, a), (_, _, b)| a.cmp(b))
        .ok_or_else(|| "unable to find square with largest total power".to_owned())?;

    Ok(format!("{},{}", x, y))
}

pub fn get_part_two(input: &str) -> Result<String, String> {
    let grid_serial_number = input
        .trim()
        .parse::<isize>()
        .map_err(|err| err.to_string())?;

    let grid: Grid = array::from_fn(|y| {
        array::from_fn(|x| get_power_level(x as isize, y as isize, grid_serial_number))
    });

    let mut max = (0, 0, 0, 0);

    for len in 0..20 {
        // "stack overflow" if this is a map too.
        let mut current_max = (0, 0, 0);

        for y in 0..(300 - len) {
            for x in 0..(300 - len) {
                let mut total_power = 0;
                for dy in 0..len {
                    for dx in 0..len {
                        total_power += grid[y + dy][x + dx];
                    }
                }

                if total_power > current_max.2 {
                    current_max = (x, y, total_power);
                }
            }
        }

        if current_max.2 > max.3 {
            max = (current_max.0, current_max.1, len, current_max.2);
        }
    }

    Ok(format!("{},{},{}", max.0, max.1, max.2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_level() {
        assert_eq!(4, get_power_level(3, 5, 8));
        assert_eq!(-5, get_power_level(122, 79, 57));
        assert_eq!(0, get_power_level(217, 196, 39));
        assert_eq!(4, get_power_level(101, 153, 71));
    }

    #[test]
    fn test_part_one() {
        assert_eq!(Ok("33,45".to_string()), get_part_one("18"));
        assert_eq!(Ok("21,61".to_string()), get_part_one("42"));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok("90,269,16".to_string()), get_part_two("18"));
        assert_eq!(Ok("232,251,12".to_string()), get_part_two("42"));
    }
}
