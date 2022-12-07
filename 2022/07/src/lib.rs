#![deny(clippy::expect_used, clippy::unwrap_used)]

use std::collections::HashMap;
use std::num::ParseIntError;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq)]
pub enum AdventOfCodeError {
    InvalidFileSystemError,
    ParseIntError(ParseIntError),
}

fn get_sizes(input: &str) -> Result<HashMap<PathBuf, usize>, AdventOfCodeError> {
    let mut cwd = PathBuf::new();

    input
        .trim()
        .lines()
        .try_fold(HashMap::new(), |mut acc, line| {
            let mut parts = line.split(' ');

            let (first, second, third) = (parts.next(), parts.next(), parts.next());

            match first {
                Some("$") => match second {
                    Some("cd") => match third {
                        Some("..") => {
                            cwd.pop();
                            Ok(acc)
                        }
                        Some(path) => {
                            cwd.push(path);
                            Ok(acc)
                        }
                        _ => Ok(acc),
                    },
                    _ => Ok(acc),
                },
                Some("dir") => Ok(acc),
                Some(size) => match second {
                    Some(path) => {
                        cwd.push(path);

                        match size.parse::<usize>() {
                            Ok(size) => {
                                cwd.ancestors().skip(1).for_each(|ancestor| {
                                    let entry = acc.entry(ancestor.to_path_buf()).or_insert(0);
                                    *entry += size;
                                });

                                cwd.pop();

                                Ok(acc)
                            }
                            Err(e) => Err(AdventOfCodeError::ParseIntError(e)),
                        }
                    }
                    None => Ok(acc),
                },
                None => Ok(acc),
            }
        })
}

pub fn get_part_one(input: &str) -> Result<usize, AdventOfCodeError> {
    let sizes = get_sizes(input)?;

    let sum = sizes.values().filter(|&size| *size <= 100_000).sum();

    Ok(sum)
}

pub fn get_part_two(input: &str) -> Result<usize, AdventOfCodeError> {
    let total_disk_space = 70_000_000;
    let required_disk_space = 30_000_000;

    let sizes = get_sizes(input)?;

    let unused_disk_space = total_disk_space
        - sizes
            .get(&PathBuf::from("/"))
            .ok_or(AdventOfCodeError::InvalidFileSystemError)?;

    let delete = *sizes
        .values()
        .filter(|&size| unused_disk_space + *size >= required_disk_space)
        .min()
        .ok_or(AdventOfCodeError::InvalidFileSystemError)?;

    Ok(delete)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
"#;

    #[test]
    fn test_part_one() {
        assert_eq!(Ok(95_437), get_part_one(INPUT));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(Ok(24_933_642), get_part_two(INPUT));
    }
}
