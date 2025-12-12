#[derive(Debug)]
struct Region {
    width: usize,
    height: usize,
    quantities: Vec<usize>,
}

#[derive(Debug)]
struct Summary {
    shapes: Vec<[[bool; 3]; 3]>,
    regions: Vec<Region>,
}

peg::parser! {
    pub grammar summary() for str {
        rule _() = [' ' | '\n']*

        rule integer() -> usize
            = n:$(['0'..='9']+)
                {? n.parse().or(Err("invalid integer")) }

        rule cell() -> bool
            = "#" { true } / "." { false }

        rule row() -> [bool;3]
            = a:cell() b:cell() c:cell() { [a, b, c] }

        rule shape() -> [[bool; 3]; 3]
            = ['0'..='9'] ":" _
              a:row()
              _
              b:row()
              _
              c:row()
              _
            {
                [a, b, c]
            }

        rule region() -> Region
            = width:integer() "x" height:integer() ":" _ quantities:(integer() ** " ") _
            {
                Region { width, height, quantities }
            }

        pub rule summary() -> Summary
            = shapes:(shape() ** _) _ regions:(region() ** _) _
            {
                Summary { shapes, regions }
            }
    }
}

pub fn get_part_one(input: &str) -> Result<usize, String> {
    let summary = summary::summary(input).map_err(|e| e.to_string())?;

    let regions = summary
        .regions
        .iter()
        .filter(|region| {
            region
                .quantities
                .iter()
                .zip(&summary.shapes)
                .map(|(quantity, shape)| {
                    shape.iter().flatten().filter(|&cell| *cell).count() * quantity
                })
                .sum::<usize>()
                <= region.width * region.height
        })
        .count();

    Ok(regions)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(2), get_part_one(INPUT));
    }
}
