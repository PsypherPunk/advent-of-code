use std::collections::VecDeque;

pub fn get_part_one(input: &str) -> Result<usize, String> {
    let parts = input.split_whitespace().collect::<Vec<_>>();
    let players = parts[0].parse::<usize>().map_err(|err| err.to_string())?;
    let last_marble = parts[6].parse::<usize>().map_err(|err| err.to_string())?;

    let mut current_marble = 1;
    let mut circle = vec![0, 1];
    let mut elves = vec![0; players];

    (2..=last_marble).for_each(|marble| {
        if marble % 23 == 0 {
            elves[marble % players] += marble;
            current_marble = ((current_marble + circle.len()) - 7) % circle.len();
            elves[marble % players] += circle.remove(current_marble);
        } else {
            let position = (current_marble + 2) % circle.len();
            circle.insert(position, marble);

            current_marble = position;
        }
    });

    let winner = elves
        .into_iter()
        .max()
        .ok_or_else(|| "invalid score".to_owned())?;

    Ok(winner)
}

pub fn get_part_two(input: &str) -> Result<usize, String> {
    let parts = input.split_whitespace().collect::<Vec<_>>();
    let players = parts[0].parse::<usize>().map_err(|err| err.to_string())?;
    let last_marble = parts[6].parse::<usize>().map_err(|err| err.to_string())?;

    let mut circle = VecDeque::with_capacity(last_marble * 100);
    circle.push_back(0);
    circle.push_back(1);
    let mut elves = vec![0; players];

    (2..=(last_marble * 100)).try_for_each(|marble| {
        if marble % 23 == 0 {
            elves[marble % players] += marble;
            (0..7).try_for_each(|_| {
                let back = circle
                    .pop_back()
                    .ok_or_else(|| "pop_back error".to_owned())?;
                circle.push_front(back);

                Ok::<(), String>(())
            })?;
            elves[marble % players] += circle
                .pop_front()
                .ok_or_else(|| "pop_front error".to_owned())?;
        } else {
            (0..2).try_for_each(|_| {
                let front = circle
                    .pop_front()
                    .ok_or_else(|| "pop_front error".to_owned())?;
                circle.push_back(front);

                Ok::<(), String>(())
            })?;
            circle.push_front(marble);
        }

        Ok::<(), String>(())
    })?;

    let winner = elves
        .into_iter()
        .max()
        .ok_or_else(|| "invalid score".to_owned())?;

    Ok(winner)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(
            Ok(32),
            get_part_one("9 players; last marble is worth 25 points")
        );
        assert_eq!(
            Ok(8317),
            get_part_one("10 players; last marble is worth 1618 points")
        );
        assert_eq!(
            Ok(146373),
            get_part_one("13 players; last marble is worth 7999 points")
        );
        assert_eq!(
            Ok(2764),
            get_part_one("17 players; last marble is worth 1104 points")
        );
        assert_eq!(
            Ok(54718),
            get_part_one("21 players; last marble is worth 6111 points")
        );
        assert_eq!(
            Ok(37305),
            get_part_one("30 players; last marble is worth 5807 points")
        );
    }
}
