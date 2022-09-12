use std::collections::HashMap;

pub struct Claim {
    id: usize,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

peg::parser! {
    pub grammar claims() for str {
        rule _() = [' ' | '\n']*

        rule integer() -> usize
            = n:$("-"* ['0'..='9']+) {? n.parse().or(Err("Invalid integer.")) }

        rule id() -> usize
            = "#" id:integer()
                { id }

        rule position() -> (usize, usize)
            = left:integer() "," top:integer()
                { (left, top) }

        rule dimensions() -> (usize, usize)
            = width:integer() "x" height:integer()
                { (width, height) }

        rule claim() -> Claim
            = id:id() " @ " position:position() ": " dimensions:dimensions()
                {
                    Claim {
                        id,
                        left: position.0,
                        top: position.1,
                        width: dimensions.0,
                        height: dimensions.1,
                    }
                }

        pub rule claims() -> Vec<Claim>
            = claim:claim() ++ _
              _
                { claim }
    }
}

fn get_inches(claims: &[Claim]) -> HashMap<(usize, usize), Vec<usize>> {
    let mut inches = HashMap::new();

    for claim in claims {
        for x in claim.left..(claim.left + claim.width) {
            for y in claim.top..(claim.top + claim.height) {
                let entry = inches.entry((x, y)).or_insert_with(Vec::new);
                entry.push(claim.id);
            }
        }
    }

    inches
}

pub fn get_overlapping_inches_count(input: &str) -> usize {
    let claims = claims::claims(input).unwrap();
    let inches = get_inches(&claims);

    inches.values().filter(|&ids| ids.len() >= 2).count()
}

pub fn get_non_overlapping_claim_id(input: &str) -> usize {
    let claims = claims::claims(input).unwrap();
    let inches = get_inches(&claims);

    let claim = claims
        .iter()
        .find(|&claim| {
            for x in claim.left..(claim.left + claim.width) {
                for y in claim.top..(claim.top + claim.height) {
                    if inches.get(&(x, y)).unwrap().len() > 1 {
                        return false;
                    }
                }
            }
            true
        })
        .unwrap();

    claim.id
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2"#;

    #[test]
    fn test_part_one() {
        assert_eq!(4, get_overlapping_inches_count(&INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(3, get_non_overlapping_claim_id(&INPUT));
    }
}
