use itertools::Itertools;

pub fn part1(input: String) {
    println!("{}", part1_impl(&input))
}

pub fn part2(input: String) {
    println!("{}", part2_impl(&input))
}

fn part1_impl(input: &str) -> usize {
    for i in 0..(input.len() - 3) {
        if input[i..i + 4].chars().all_unique() {
            return i + 4;
        }
    }
    0
}

fn part2_impl(input: &str) -> usize {
    for i in 0..(input.len() - 13) {
        if input[i..i + 14].chars().all_unique() {
            return i + 14;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1_impl("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(part1_impl("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part1_impl("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(part1_impl("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part1_impl("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2_impl("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(part2_impl("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(part2_impl("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(part2_impl("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(part2_impl("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
