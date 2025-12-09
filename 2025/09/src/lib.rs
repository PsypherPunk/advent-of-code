use std::cmp::Reverse;

struct Tile {
    x: usize,
    y: usize,
}

fn get_tiles(input: &str) -> Result<Vec<Tile>, String> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').ok_or("invalid line")?;
            let x = x.parse::<usize>().map_err(|e| e.to_string())?;
            let y = y.parse::<usize>().map_err(|e| e.to_string())?;

            Ok(Tile { x, y })
        })
        .collect::<Result<Vec<_>, String>>()
}

impl Tile {
    fn corners(&self, other: &Self) -> (Self, Self) {
        (
            Self {
                x: self.x.min(other.x),
                y: self.y.min(other.y),
            },
            Self {
                x: self.x.max(other.x),
                y: self.y.max(other.y),
            },
        )
    }
}

pub fn get_part_one(input: &str) -> Result<usize, String> {
    let tiles = get_tiles(input)?;

    let max = tiles
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            tiles
                .iter()
                .skip(i + 1)
                .map(move |b| (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1))
        })
        .max()
        .ok_or("invalid input")?;

    Ok(max)
}

pub fn get_part_two(input: &str) -> Result<usize, String> {
    let tiles = get_tiles(input)?;

    let rectangles = {
        let mut rectangles = tiles
            .iter()
            .enumerate()
            .flat_map(|(i, a)| {
                tiles
                    .iter()
                    .skip(i + 1)
                    .map(move |b| (a, b, (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1)))
            })
            .collect::<Vec<_>>();

        rectangles.sort_unstable_by_key(|&(_, _, area)| Reverse(area));
        rectangles
    };

    let (_, _, area) = rectangles
        .iter()
        .find(|(a, b, _)| {
            let (rect_min, rect_max) = a.corners(b);

            let crosses_edge =
                tiles
                    .iter()
                    .zip(tiles.iter().cycle().skip(1))
                    .any(|(edge_start, edge_end)| {
                        if edge_start.x == edge_end.x {
                            let edge_x = edge_start.x;
                            let (edge_y_min, edge_y_max) =
                                (edge_start.y.min(edge_end.y), edge_start.y.max(edge_end.y));

                            let rectangle_spans_edge_x = rect_min.x < edge_x && rect_max.x > edge_x;

                            let y_ranges_overlap =
                                !(rect_min.y >= edge_y_max || rect_max.y <= edge_y_min);

                            rectangle_spans_edge_x && y_ranges_overlap
                        } else if edge_start.y == edge_end.y {
                            let edge_y = edge_start.y;
                            let (edge_x_min, edge_x_max) =
                                (edge_start.x.min(edge_end.x), edge_start.x.max(edge_end.x));

                            let rectangle_spans_edge_y = rect_min.y < edge_y && rect_max.y > edge_y;

                            let x_ranges_overlap =
                                !(rect_min.x >= edge_x_max || rect_max.x <= edge_x_min);

                            rectangle_spans_edge_y && x_ranges_overlap
                        } else {
                            unreachable!("edges should be axis-aligned")
                        }
                    });

            !crosses_edge
        })
        .ok_or("cannot find largest area".to_string())?;

    Ok(*area)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(50), get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(24), get_part_two(INPUT));
    }
}
