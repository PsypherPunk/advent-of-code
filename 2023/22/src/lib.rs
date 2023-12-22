use std::str::FromStr;

struct Brick {
    x: (usize, usize),
    y: (usize, usize),
    z: (usize, usize),
}

impl FromStr for Brick {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = s
            .splitn(6, &[',', '~'])
            .map(|n| n.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?;

        match &numbers[..] {
            [ax, ay, az, bx, by, bz] => Ok(Self {
                x: (*ax, *bx),
                y: (*ay, *by),
                z: (*az, *bz),
            }),
            _ => Err(format!("invalid line: {}", s)),
        }
    }
}

impl Brick {
    fn covers(&self, other: &Self) -> bool {
        self.x.0.max(other.x.0) <= self.x.1.min(other.x.1)
            && self.y.0.max(other.y.0) <= self.y.1.min(other.y.1)
    }
}

pub fn get_part_one(input: &str) -> Result<usize, String> {
    let mut bricks = input
        .trim()
        .lines()
        .map(Brick::from_str)
        .collect::<Result<Vec<_>, _>>()?;

    bricks.sort_unstable_by_key(|brick| brick.z.0);

    // TODO: ick.
    for i in 0..bricks.len() {
        let mut min_z = 1;

        for j in 0..i {
            if bricks[i].covers(&bricks[j]) {
                min_z = min_z.max(bricks[j].z.1 + 1);
            }
        }

        let height = bricks[i].z.1 - bricks[i].z.0;
        bricks[i].z.0 = min_z;
        bricks[i].z.1 = min_z + height;
    }

    // TODO: double-ick.
    let mut below = vec![Vec::new(); bricks.len()];
    for i in 0..bricks.len() {
        for j in 0..i {
            if bricks[i].covers(&bricks[j]) && bricks[i].z.0 == bricks[j].z.1 + 1 {
                below[i].push(j);
            }
        }
    }

    // TODO: bleurgh.
    let mut safe = vec![true; below.len()];
    for underneath in below {
        if underneath.len() == 1 {
            safe[underneath[0]] = false;
        }
    }

    Ok(safe.iter().filter(|&brick| *brick).count())
}

pub fn get_part_two(_input: &str) -> Result<usize, String> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(5), get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(2), get_part_two(INPUT));
    }
}
