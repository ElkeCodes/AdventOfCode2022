use itertools::Itertools;
use std::char;

pub fn part1(input: String) {
    println!("{}", part1_impl(&input))
}

pub fn part2(input: String) {
    println!("{}", part2_impl(&input))
}
#[derive(Clone, Copy)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Clone, Copy)]
enum Intention {
    Lose,
    Draw,
    Win,
}

fn transform_to_shape(c: Option<char>) -> Shape {
    match c {
        Some('A') => Shape::Rock,
        Some('B') => Shape::Paper,
        Some('C') => Shape::Scissors,
        _ => panic!("Unknown shape"),
    }
}

fn transform_to_player_shape(c: Option<char>) -> Shape {
    match c {
        Some('X') => Shape::Rock,
        Some('Y') => Shape::Paper,
        Some('Z') => Shape::Scissors,
        _ => panic!("Unknown shape"),
    }
}

fn transform_to_intention(c: Option<char>) -> Intention {
    match c {
        Some('X') => Intention::Lose,
        Some('Y') => Intention::Draw,
        Some('Z') => Intention::Win,
        _ => panic!("Unknown intention"),
    }
}

fn parse_lines(input: &str) -> Vec<(Shape, Shape, Intention)> {
    input
        .lines()
        .enumerate()
        .map(|(_, line)| {
            (
                transform_to_shape(line.chars().nth(0)),
                transform_to_player_shape(line.chars().nth(2)),
                transform_to_intention(line.chars().nth(2)),
            )
        })
        .collect_vec()
}

fn get_score_for_player_shape(shape: Shape) -> u128 {
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}

fn get_shape_for_intention(opponent: Shape, intention: Intention) -> Shape {
    match opponent {
        Shape::Rock => match intention {
            Intention::Lose => Shape::Scissors,
            Intention::Draw => Shape::Rock,
            Intention::Win => Shape::Paper,
        },
        Shape::Paper => match intention {
            Intention::Lose => Shape::Rock,
            Intention::Draw => Shape::Paper,
            Intention::Win => Shape::Scissors,
        },
        Shape::Scissors => match intention {
            Intention::Lose => Shape::Paper,
            Intention::Draw => Shape::Scissors,
            Intention::Win => Shape::Rock,
        },
    }
}

fn get_score_fight(opponent: Shape, player: Shape) -> u128 {
    match opponent {
        Shape::Rock => match player {
            Shape::Scissors => 0,
            Shape::Rock => 3,
            Shape::Paper => 6,
        },
        Shape::Paper => match player {
            Shape::Scissors => 6,
            Shape::Rock => 0,
            Shape::Paper => 3,
        },
        Shape::Scissors => match player {
            Shape::Scissors => 3,
            Shape::Rock => 6,
            Shape::Paper => 0,
        },
    }
}

fn part1_impl(input: &str) -> u128 {
    let lines = parse_lines(input);
    lines.into_iter().fold(0, |score, (opponent, player, _)| {
        score + get_score_fight(opponent, player) + get_score_for_player_shape(player)
    })
}

fn part2_impl(input: &str) -> u128 {
    let lines = parse_lines(input);
    lines
        .into_iter()
        .fold(0, |score, (opponent, _, intention)| {
            score
                + get_score_fight(opponent, get_shape_for_intention(opponent, intention))
                + get_score_for_player_shape(get_shape_for_intention(opponent, intention))
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "A Y\nB X\nC Z";

    #[test]
    fn test_part1() {
        assert_eq!(part1_impl(TEST_INPUT), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2_impl(TEST_INPUT), 12);
    }
}
