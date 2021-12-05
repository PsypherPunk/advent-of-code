#[derive(Clone, Debug)]
struct Board {
    numbers: Vec<Vec<Number>>,
    won: bool,
}

#[derive(Clone, Copy, Debug)]
struct Number {
    number: usize,
    marked: bool,
}

#[derive(Debug)]
pub struct Bingo {
    numbers: Vec<usize>,
    boards: Vec<Board>,
    winners: Vec<(usize, Board)>,
}

peg::parser! {
    pub grammar bingo() for str {
        rule _() = [' ' | '\n']*

        rule integer() -> usize
            = n:$(['0'..='9']+) {? n.parse().or(Err("Invalid integer.")) }

        rule numbers() -> Vec<usize>
            = integers:integer() ++ ","
                { integers }

        rule line() -> Vec<Number>
            = _
              integers:integer() **<5> _
                {
                    integers
                        .iter()
                        .map(|number| Number {
                            number: *number,
                            marked: false,
                        })
                        .collect()
                }

        rule board() -> Board
            = lines:line() **<5> _
                {
                    Board {
                        numbers: lines,
                        won: false,
                    }
                }

        pub rule bingo() -> Bingo
            = numbers:numbers()
              _
              boards:board() ++ _
              _
                {
                    Bingo {
                        numbers,
                        boards,
                        winners: Vec::new(),
                    }
                }
    }
}

impl Board {
    fn mark_draw(&mut self, draw: usize) {
        self.numbers.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|number| {
                if number.number == draw {
                    number.marked = true;
                }
            })
        });
    }

    fn is_winner(&self) -> bool {
        self.numbers
            .iter()
            .any(|row| row.iter().all(|number| number.marked))
            || (0..5).any(|column| {
                self.numbers
                    .iter()
                    .map(|row| row[column])
                    .all(|number| number.marked)
            })
    }

    fn get_unmarked_numbers(&self) -> Vec<usize> {
        self.numbers
            .iter()
            .flat_map(|row| {
                row.iter()
                    .filter(|number| !number.marked)
                    .map(|number| number.number)
            })
            .collect()
    }
}

impl Bingo {
    fn play_game(&mut self) {
        for draw in &self.numbers {
            self.boards
                .iter_mut()
                .filter(|board| !board.won)
                .for_each(|board| {
                    board.mark_draw(*draw);
                    if board.is_winner() {
                        self.winners.push((*draw, board.clone()));
                        board.won = true;
                    }
                });
        }
    }
}

pub fn get_part_one(input: &str) -> usize {
    let mut bingo = bingo::bingo(input).unwrap();

    bingo.play_game();

    let (draw, first_winner) = &bingo.winners.first().unwrap();
    let unmarked_sum = first_winner.get_unmarked_numbers().iter().sum::<usize>();

    draw * unmarked_sum
}

pub fn get_part_two(input: &str) -> usize {
    let mut bingo = bingo::bingo(input).unwrap();

    bingo.play_game();

    let (draw, last_winner) = &bingo.winners.last().unwrap();
    let unmarked_sum = last_winner.get_unmarked_numbers().iter().sum::<usize>();

    draw * unmarked_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(4512, get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(1924, get_part_two(INPUT));
    }
}
