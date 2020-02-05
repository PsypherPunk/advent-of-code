use std::cmp;
use std::collections::HashMap;
use std::fs;

use regex::Regex;

#[derive(Debug)]
struct Cookie {
    calories: i32,
    score: i32,
}

#[derive(Debug)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

fn get_cookie_scores(input: &str) -> Vec<Cookie> {
    let ingredients = get_ingredients(&input);
    let mut cookies = Vec::new();

    for frosting in 1..=97 {
        for candy in 1..=97 {
            for butterscotch in 1..=97 {
                let sugar = 100 - frosting - candy - butterscotch;
                let capacity = frosting * ingredients.get("Frosting").unwrap().capacity
                    + candy * ingredients.get("Candy").unwrap().capacity
                    + butterscotch * ingredients.get("Butterscotch").unwrap().capacity
                    + sugar * ingredients.get("Sugar").unwrap().capacity;
                let durability = frosting * ingredients.get("Frosting").unwrap().durability
                    + candy * ingredients.get("Candy").unwrap().durability
                    + butterscotch * ingredients.get("Butterscotch").unwrap().durability
                    + sugar * ingredients.get("Sugar").unwrap().durability;
                let flavor = frosting * ingredients.get("Frosting").unwrap().flavor
                    + candy * ingredients.get("Candy").unwrap().flavor
                    + butterscotch * ingredients.get("Butterscotch").unwrap().flavor
                    + sugar * ingredients.get("Sugar").unwrap().flavor;
                let texture = frosting * ingredients.get("Frosting").unwrap().texture
                    + candy * ingredients.get("Candy").unwrap().texture
                    + butterscotch * ingredients.get("Butterscotch").unwrap().texture
                    + sugar * ingredients.get("Sugar").unwrap().texture;
                let calories = frosting * ingredients.get("Frosting").unwrap().calories
                    + candy * ingredients.get("Candy").unwrap().calories
                    + butterscotch * ingredients.get("Butterscotch").unwrap().calories
                    + sugar * ingredients.get("Sugar").unwrap().calories;
                cookies.push(Cookie {
                    calories: cmp::max(calories, 0),
                    score: cmp::max(capacity, 0)
                        * cmp::max(durability, 0)
                        * cmp::max(flavor, 0)
                        * cmp::max(texture, 0),
                });
            }
        }
    }
    cookies.sort_by(|a, b| a.score.cmp(&b.score));
    cookies
}

fn get_ingredients(input: &str) -> HashMap<String, Ingredient> {
    let re = Regex::new(r#"^(\w+): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)$"#).unwrap();

    input
        .trim()
        .lines()
        .map(|line| {
            let captures = re.captures(line.trim()).unwrap();
            (
                String::from(&captures[1]),
                Ingredient {
                    capacity: captures[2].parse::<i32>().unwrap(),
                    durability: captures[3].parse::<i32>().unwrap(),
                    flavor: captures[4].parse::<i32>().unwrap(),
                    texture: captures[5].parse::<i32>().unwrap(),
                    calories: captures[6].parse::<i32>().unwrap(),
                },
            )
        })
        .collect()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let cookies = get_cookie_scores(&input);

    println!(
        "…what is the total score of the highest-scoring cookie you can make? {}",
        cookies.last().unwrap().score,
    );

    let five_hundred = cookies.iter().filter(|cookie| cookie.calories == 500);
    println!(
        "…what is the total score of the highest-scoring cookie you can make with a calorie total of 500? {}",
        five_hundred.last().unwrap().score,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3"#;

        let ingredients = get_ingredients(&input);

        dbg!(ingredients);
    }
}
