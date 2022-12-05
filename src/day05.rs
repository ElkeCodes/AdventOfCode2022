use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, LinkedList};

type Stack = LinkedList<char>;
type CrateStacks = HashMap<usize, Stack>;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

pub fn part1(input: String) {
    println!("{}", part1_impl(&input))
}

pub fn part2(input: String) {
    println!("{}", part2_impl(&input))
}

fn parse_lines(input: &str) -> (CrateStacks, Vec<Move>) {
    let mut crate_stacks = CrateStacks::default();
    let mut moves = vec![];
    let crates_regex = Regex::new(r"(\[[A-Z]\])+").unwrap();
    input.lines().enumerate().for_each(|(_, line)| {
        if line.starts_with("move") {
            let mut parts = line.split(" ");
            moves.push(Move {
                amount: parts.nth(1).unwrap().parse().unwrap(),
                from: parts.nth(1).unwrap().parse().unwrap(),
                to: parts.nth(1).unwrap().parse().unwrap(),
            });
        } else if crates_regex.is_match(line) {
            line.chars().enumerate().for_each(|(index, c)| {
                if c != ' ' && c != '[' && c != ']' {
                    crate_stacks
                        .entry(((index - 1) / 4) + 1)
                        .or_insert(Stack::default())
                        .push_back(c)
                }
            });
        }
    });
    (crate_stacks, moves)
}

fn part1_impl(input: &str) -> String {
    let (mut crate_stacks, moves) = parse_lines(input);
    moves.into_iter().for_each(|m| {
        let mut pushed_stack = Stack::default();
        crate_stacks.entry(m.from).and_modify(|stack| {
            for _ in 0..m.amount {
                let c = (*stack).pop_front().unwrap();
                pushed_stack.push_back(c);
            }
        });
        for _ in 0..m.amount {
            let c = pushed_stack.pop_front().unwrap();
            crate_stacks.entry(m.to).and_modify(|s| s.push_front(c));
        }
    });
    crate_stacks.into_iter().sorted().fold(
        String::default(),
        |mut acc: String, (_, mut stack): (_, Stack)| {
            acc.push(stack.pop_front().unwrap());
            acc
        },
    )
}

fn part2_impl(input: &str) -> String {
    let (mut crate_stacks, moves) = parse_lines(input);
    moves.into_iter().for_each(|m| {
        let mut pushed_stack = Stack::default();
        crate_stacks.entry(m.from).and_modify(|stack| {
            for _ in 0..m.amount {
                let c = (*stack).pop_front().unwrap();
                pushed_stack.push_back(c);
            }
        });
        for _ in 0..m.amount {
            let c = pushed_stack.pop_back().unwrap();
            crate_stacks.entry(m.to).and_modify(|s| s.push_front(c));
        }
    });
    crate_stacks.into_iter().sorted().fold(
        String::default(),
        |mut acc: String, (_, mut stack): (_, Stack)| {
            acc.push(stack.pop_front().unwrap());
            acc
        },
    )
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    static TEST_INPUT: &str = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2";

    #[test]
    fn test_part1() {
        assert_eq!(part1_impl(TEST_INPUT), String::from("CMZ"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2_impl(TEST_INPUT), String::from("MCD"));
    }

    #[test]
    fn test_parse_lines() {
        assert_eq!(
            parse_lines(TEST_INPUT),
            (
                CrateStacks::from([
                    (1, Stack::from(['Z', 'N'])),
                    (2, Stack::from(['M', 'C', 'D'])),
                    (3, Stack::from(['P']))
                ]),
                vec![
                    Move {
                        amount: 1,
                        from: 2,
                        to: 1
                    },
                    Move {
                        amount: 3,
                        from: 1,
                        to: 3
                    },
                    Move {
                        amount: 2,
                        from: 2,
                        to: 1
                    },
                    Move {
                        amount: 1,
                        from: 1,
                        to: 2
                    }
                ]
            )
        );
    }
}
