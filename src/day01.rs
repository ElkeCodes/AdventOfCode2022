pub fn part1(input: String) {
    println!("{}", part1_impl(&input))
}

pub fn part2(input: String) {
    println!("{}", part2_impl(&input))
}

fn parse_elves(input: &str) -> Vec<u128> {
    let mut elves: Vec<u128> = vec![];
    let mut current_elf = 0;
    input.lines().enumerate().for_each(|(_, line)| {
        if line.is_empty() {
            elves.push(current_elf);
            current_elf = 0;
        } else {
            current_elf += line.parse::<u128>().unwrap();
        }
    });
    elves.push(current_elf);
    elves.sort_by(|x, y| y.cmp(x));
    elves
}

fn part1_impl(input: &str) -> u128 {
    let elves = parse_elves(input);
    *elves.first().unwrap()
}

fn part2_impl(input: &str) -> u128 {
    let elves = parse_elves(input);
    elves[0..3].into_iter().fold(0, |acc, el| acc + el)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";

    #[test]
    fn test_part1() {
        assert_eq!(part1_impl(TEST_INPUT), 24000);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2_impl(TEST_INPUT), 45000);
    }
}
