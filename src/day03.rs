use itertools::Itertools;
use std::char;

type Ruckssack<'a> = (&'a str, &'a str, &'a str);

pub fn part1(input: String) {
    println!("{}", part1_impl(&input))
}

pub fn part2(input: String) {
    println!("{}", part2_impl(&input))
}

fn parse_lines(input: &str) -> Vec<Ruckssack> {
    input
        .lines()
        .enumerate()
        .map(|(_, line)| (&line[..(line.len() / 2)], &line[(line.len() / 2)..], line))
        .collect_vec()
}

fn transform_to_priority(c: char) -> u128 {
    let x = c as u128;
    match x {
        97..=123 => x - 96, // lowercase letters
        65..=96 => x - 38,  // uppercase letters
        _ => 0,
    }
}

fn find_duplicates_in_rucksack(rucksacks: &Vec<Ruckssack>) -> Vec<u128> {
    rucksacks
        .into_iter()
        .map(
            |rucksack| match rucksack.0.chars().find(|x| rucksack.1.chars().contains(x)) {
                Some(x) => transform_to_priority(x),
                None => 0,
            },
        )
        .collect_vec()
}

fn find_three_elves_groups(rucksacks: &Vec<Ruckssack>) -> Vec<u128> {
    rucksacks
        .chunks(3)
        .map(|a| match a {
            &[r1, r2, r3] => {
                match r1
                    .2
                    .chars()
                    .find(|x| r2.2.chars().contains(x) && r3.2.chars().contains(x))
                {
                    Some(x) => transform_to_priority(x),
                    None => 0,
                }
            }
            _ => 0,
        })
        .collect_vec()
}

fn part1_impl(input: &str) -> u128 {
    let rucksacks = parse_lines(input);
    find_duplicates_in_rucksack(&rucksacks)
        .into_iter()
        .sum::<u128>()
}

fn part2_impl(input: &str) -> u128 {
    let rucksacks = parse_lines(input);
    find_three_elves_groups(&rucksacks)
        .into_iter()
        .sum::<u128>()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part1() {
        assert_eq!(part1_impl(TEST_INPUT), 157);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2_impl(TEST_INPUT), 70);
    }

    #[test]
    fn test_transform_to_priority() {
        assert_eq!(transform_to_priority('a'), 1);
        assert_eq!(transform_to_priority('z'), 26);
        assert_eq!(transform_to_priority('A'), 27);
        assert_eq!(transform_to_priority('Z'), 52);
    }
}
