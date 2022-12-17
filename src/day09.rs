use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Debug, PartialEq)]
enum Instruction {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

type Coordinate = (isize, isize);

pub fn part1(input: String) {
    println!("{}", part1_impl(&input))
}

pub fn part2(input: String) {
    println!("{}", part2_impl(&input))
}

fn parse_lines(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let splits = line.split(" ").collect_vec();
            match splits[0] {
                "U" => Instruction::Up(splits[1].parse().unwrap()),
                "D" => Instruction::Down(splits[1].parse().unwrap()),
                "L" => Instruction::Left(splits[1].parse().unwrap()),
                "R" => Instruction::Right(splits[1].parse().unwrap()),
                _ => panic!("Unknown instruction type"),
            }
        })
        .collect_vec()
}

fn is_touching(c1: Coordinate, c2: Coordinate) -> bool {
    c1.0.abs_diff(c2.0) <= 1 && c1.1.abs_diff(c2.1) <= 1
}

fn move_towards_coordinate(target: Coordinate, mut origin: Coordinate) -> Coordinate {
    if !is_touching(target, origin) {
        if target.1 > origin.1 {
            origin.1 += 1;
        }
        if target.1 < origin.1 {
            origin.1 -= 1;
        }
        if target.0 > origin.0 {
            origin.0 += 1;
        }
        if target.0 < origin.0 {
            origin.0 -= 1;
        }
    }
    origin
}

fn move_rope_towards_coordinate(
    mut target: Coordinate,
    mut rope: Vec<Coordinate>,
) -> Vec<Coordinate> {
    let mut new_rope = Vec::new();
    let mut rope_iter = rope.iter_mut().peekable();
    while let Some(coord) = rope_iter.next() {
        let new_coord = move_towards_coordinate(target, *coord);
        new_rope.push(new_coord);
        target = new_coord;
    }
    new_rope
}

fn part1_impl(input: &str) -> usize {
    let instructions = parse_lines(input);
    let start_coordinate: Coordinate = (0, 0);
    let mut head_coordinate = start_coordinate;
    let mut tail_coordinate = start_coordinate;
    let mut coordinates_visited: HashMap<Coordinate, bool> = HashMap::new();
    instructions
        .into_iter()
        .for_each(|instruction| match instruction {
            Instruction::Up(amount) => {
                for _ in 0..amount {
                    head_coordinate.1 += 1;
                    tail_coordinate = move_towards_coordinate(head_coordinate, tail_coordinate);
                    coordinates_visited.insert(tail_coordinate, true);
                }
            }
            Instruction::Down(amount) => {
                for _ in 0..amount {
                    head_coordinate.1 -= 1;
                    tail_coordinate = move_towards_coordinate(head_coordinate, tail_coordinate);
                    coordinates_visited.insert(tail_coordinate, true);
                }
            }
            Instruction::Left(amount) => {
                for _ in 0..amount {
                    head_coordinate.0 -= 1;
                    tail_coordinate = move_towards_coordinate(head_coordinate, tail_coordinate);
                    coordinates_visited.insert(tail_coordinate, true);
                }
            }
            Instruction::Right(amount) => {
                for _ in 0..amount {
                    head_coordinate.0 += 1;
                    tail_coordinate = move_towards_coordinate(head_coordinate, tail_coordinate);
                    coordinates_visited.insert(tail_coordinate, true);
                }
            }
        });
    coordinates_visited.len()
}

fn part2_impl(input: &str) -> usize {
    let instructions = parse_lines(input);
    let start_coordinate: Coordinate = (0, 0);
    let mut head_coordinate = start_coordinate;
    let mut rope = vec![start_coordinate; 9];
    let mut coordinates_visited: HashSet<Coordinate> = HashSet::new();
    instructions.into_iter().for_each(|instruction| {
        match instruction {
            Instruction::Up(amount) => {
                for _ in 0..amount {
                    head_coordinate.1 += 1;
                    rope = move_rope_towards_coordinate(head_coordinate, rope.clone());
                    coordinates_visited.insert(rope[8]); 
                }
            }
            Instruction::Down(amount) => {
                for _ in 0..amount {
                    head_coordinate.1 -= 1;
                    rope = move_rope_towards_coordinate(head_coordinate, rope.clone());
                    coordinates_visited.insert(rope[8]); 
                }
            }
            Instruction::Left(amount) => {
                for _ in 0..amount {
                    head_coordinate.0 -= 1;
                    rope = move_rope_towards_coordinate(head_coordinate, rope.clone());
                    coordinates_visited.insert(rope[8]);
                }
            }
            Instruction::Right(amount) => {
                for _ in 0..amount {
                    head_coordinate.0 += 1;
                    rope = move_rope_towards_coordinate(head_coordinate, rope.clone());
                    coordinates_visited.insert(rope[8]); 
                }
            }
        }
    });
    coordinates_visited.len()
}
#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";

    #[test]
    fn test_part1() {
        assert_eq!(part1_impl(TEST_INPUT), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2_impl("R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20"), 36);
        assert_eq!(part2_impl(TEST_INPUT), 1);
    }

    #[test]
    fn test_parse_lines() {
        let parsed_result = parse_lines(TEST_INPUT);
        assert_eq!(parsed_result[0], Instruction::Right(4));
        assert_eq!(parsed_result[1], Instruction::Up(4));
        assert_eq!(parsed_result[2], Instruction::Left(3));
        assert_eq!(parsed_result[3], Instruction::Down(1));
        assert_eq!(parsed_result[4], Instruction::Right(4));
        assert_eq!(parsed_result[5], Instruction::Down(1));
        assert_eq!(parsed_result[6], Instruction::Left(5));
        assert_eq!(parsed_result[7], Instruction::Right(2));
    }

    #[test]
    fn test_is_touching() {
        assert_eq!(is_touching((3, 3), (2, 2)), true);
        assert_eq!(is_touching((3, 3), (1, 2)), false);
        assert_eq!(is_touching((3, 3), (3, 2)), true);
        assert_eq!(is_touching((3, 3), (3, 3)), true);
        assert_eq!(is_touching((3, 3), (4, 3)), true);
        assert_eq!(is_touching((3, 3), (4, 5)), false);
        assert_eq!(is_touching((3, 3), (5, 3)), false);
    }

    #[test]
    fn test_move_towards_coordinate() {
        assert_eq!(move_towards_coordinate((0, 0), (0, 0)), (0, 0));
        assert_eq!(move_towards_coordinate((1, 0), (0, 0)), (0, 0));
        assert_eq!(move_towards_coordinate((2, 0), (0, 0)), (1, 0));
        assert_eq!(move_towards_coordinate((3, 0), (1, 0)), (2, 0));
        assert_eq!(move_towards_coordinate((4, 0), (2, 0)), (3, 0));
        assert_eq!(move_towards_coordinate((4, 1), (3, 0)), (3, 0));
        assert_eq!(move_towards_coordinate((4, 2), (3, 0)), (4, 1));
        assert_eq!(move_towards_coordinate((4, 3), (4, 1)), (4, 2));
        assert_eq!(move_towards_coordinate((4, 4), (4, 2)), (4, 3));
        assert_eq!(move_towards_coordinate((3, 4), (4, 3)), (4, 3));
        assert_eq!(move_towards_coordinate((2, 4), (4, 3)), (3, 4));
        assert_eq!(move_towards_coordinate((1, 4), (3, 4)), (2, 4));
        assert_eq!(move_towards_coordinate((1, 3), (2, 4)), (2, 4));
        assert_eq!(move_towards_coordinate((2, 3), (2, 4)), (2, 4));
        assert_eq!(move_towards_coordinate((3, 3), (2, 4)), (2, 4));
        assert_eq!(move_towards_coordinate((4, 3), (2, 4)), (3, 3));
        assert_eq!(move_towards_coordinate((5, 3), (3, 3)), (4, 3));
        assert_eq!(move_towards_coordinate((5, 2), (4, 3)), (4, 3));
        assert_eq!(move_towards_coordinate((4, 2), (4, 3)), (4, 3));
        assert_eq!(move_towards_coordinate((3, 2), (4, 3)), (4, 3));
        assert_eq!(move_towards_coordinate((2, 2), (4, 3)), (3, 2));
        assert_eq!(move_towards_coordinate((1, 2), (3, 2)), (2, 2));
        assert_eq!(move_towards_coordinate((0, 2), (2, 2)), (1, 2));
        assert_eq!(move_towards_coordinate((1, 2), (1, 2)), (1, 2));
        assert_eq!(move_towards_coordinate((2, 2), (1, 2)), (1, 2));
    }

    #[test]
    fn test_move_rope_towards_coordinate() {
        assert_eq!(
            move_rope_towards_coordinate(
                (0, 0),
                vec![
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0)
                ]
            ),
            vec![
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0)
            ]
        );
        assert_eq!(
            move_rope_towards_coordinate(
                (1, 0),
                vec![
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0)
                ]
            ),
            vec![
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0)
            ]
        );
        assert_eq!(
            move_rope_towards_coordinate(
                (2, 0),
                vec![
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0)
                ]
            ),
            vec![
                (1, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0)
            ]
        );
        assert_eq!(
            move_rope_towards_coordinate(
                (3, 0),
                vec![
                    (1, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0)
                ]
            ),
            vec![
                (2, 0),
                (1, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0)
            ]
        );
        assert_eq!(
            move_rope_towards_coordinate(
                (4, 0),
                vec![
                    (2, 0),
                    (1, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0)
                ]
            ),
            vec![
                (3, 0),
                (2, 0),
                (1, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0)
            ]
        );
        assert_eq!(
            move_rope_towards_coordinate(
                (4, 1),
                vec![
                    (3, 0),
                    (2, 0),
                    (1, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0)
                ]
            ),
            vec![
                (3, 0),
                (2, 0),
                (1, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0)
            ]
        );
        assert_eq!(
            move_rope_towards_coordinate(
                (4, 2),
                vec![
                    (3, 0),
                    (2, 0),
                    (1, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0)
                ]
            ),
            vec![
                (4, 1),
                (3, 1),
                (2, 1),
                (1, 1),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0)
            ]
        );
        assert_eq!(
            move_rope_towards_coordinate(
                (4, 3),
                vec![
                    (4, 1),
                    (3, 1),
                    (2, 1),
                    (1, 1),
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0)
                ]
            ),
            vec![
                (4, 2),
                (3, 1),
                (2, 1),
                (1, 1),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0)
            ]
        );
        assert_eq!(
            move_rope_towards_coordinate(
                (4, 4),
                vec![
                    (4, 2),
                    (3, 1),
                    (2, 1),
                    (1, 1),
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0)
                ]
            ),
            vec![
                (4, 3),
                (4, 2),
                (3, 2),
                (2, 2),
                (1, 1),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0)
            ]
        );
        assert_eq!(
            move_rope_towards_coordinate(
                (3, 4),
                vec![
                    (4, 3),
                    (4, 2),
                    (3, 2),
                    (2, 2),
                    (1, 1),
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0)
                ]
            ),
            vec![
                (4, 3),
                (4, 2),
                (3, 2),
                (2, 2),
                (1, 1),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0)
            ]
        );
        assert_eq!(
            move_rope_towards_coordinate(
                (2, 4),
                vec![
                    (4, 3),
                    (4, 2),
                    (3, 2),
                    (2, 2),
                    (1, 1),
                    (0, 0),
                    (0, 0),
                    (0, 0),
                    (0, 0)
                ]
            ),
            vec![
                (3, 4),
                (3, 3),
                (3, 2),
                (2, 2),
                (1, 1),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0)
            ]
        );
        assert_eq!(
            move_rope_towards_coordinate(
                (12, 5),
                vec![
                    (11, 5),
                    (11, 5),
                    (11, 5),
                    (11, 5),
                    (11, 5),
                    (11, 5),
                    (11, 5),
                    (11, 5),
                    (11, 5),
                ]
            ),
            vec![
                (11, 5),
                (11, 5),
                (11, 5),
                (11, 5),
                (11, 5),
                (11, 5),
                (11, 5),
                (11, 5),
                (11, 5),
            ]
        );
        assert_eq!(
            move_rope_towards_coordinate(
                (13, 5),
                vec![
                    (11, 5),
                    (11, 5),
                    (11, 5),
                    (11, 5),
                    (11, 5),
                    (11, 5),
                    (11, 5),
                    (11, 5),
                    (11, 5),
                ]
            ),
            vec![
                (12, 5),
                (11, 5),
                (11, 5),
                (11, 5),
                (11, 5),
                (11, 5),
                (11, 5),
                (11, 5),
                (11, 5),
            ]
        );
        assert_eq!(
            move_rope_towards_coordinate(
                (14, 5),
                vec![
                    (12, 5),
                    (11, 5),
                    (11, 5),
                    (11, 5),
                    (11, 5),
                    (11, 5),
                    (11, 5),
                    (11, 5),
                    (11, 5),
                ]
            ),
            vec![
                (13, 5),
                (12, 5),
                (11, 5),
                (11, 5),
                (11, 5),
                (11, 5),
                (11, 5),
                (11, 5),
                (11, 5),
            ]
        );
        assert_eq!(
            move_rope_towards_coordinate(
                (15, 5),
                vec![
                    (13, 5),
                    (12, 5),
                    (11, 5),
                    (11, 5),
                    (11, 5),
                    (11, 5),
                    (11, 5),
                    (11, 5),
                    (11, 5),
                ]
            ),
            vec![
                (14, 5),
                (13, 5),
                (12, 5),
                (11, 5),
                (11, 5),
                (11, 5),
                (11, 5),
                (11, 5),
                (11, 5),
            ]
        );
        assert_eq!(
            move_rope_towards_coordinate(
                (16, 5),
                vec![
                    (14, 5),
                    (13, 5),
                    (12, 5),
                    (11, 5),
                    (11, 5),
                    (11, 5),
                    (11, 5),
                    (11, 5),
                    (11, 5),
                ]
            ),
            vec![
                (15, 5),
                (14, 5),
                (13, 5),
                (12, 5),
                (11, 5),
                (11, 5),
                (11, 5),
                (11, 5),
                (11, 5),
            ]
        );
    }
}
