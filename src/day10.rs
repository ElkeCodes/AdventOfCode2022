use itertools::Itertools;

#[derive(Debug, PartialEq)]
enum Instruction {
    ADDX(isize),
    NOOP(),
}

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
            if line.starts_with("noop") {
                vec![Instruction::NOOP()]
            } else {
                let splits = line.split(" ").collect_vec();
                let amount = splits[1].parse::<isize>().unwrap();
                vec![Instruction::NOOP(), Instruction::ADDX(amount)]
            }
        })
        .flatten()
        .collect_vec()
}

fn part1_impl(input: &str) -> isize {
    let to_check: Vec<usize> = vec![20, 60, 100, 140, 180, 220];
    let mut current_value: isize = 1;
    let mut previous_value: isize = 1;
    let mut total = 0;
    parse_lines(input)
        .iter()
        .enumerate()
        .for_each(|(cycle, instruction)| {
            if to_check.contains(&cycle) {
                total += previous_value * (cycle as isize);
            }
            previous_value = current_value;
            match instruction {
                Instruction::ADDX(amount) => current_value += amount,
                Instruction::NOOP() => (),
            }
        });
    total
}

fn part2_impl(input: &str) -> String {
    let mut next_sprite_position: isize = 1;
    let mut sprite_position: isize = 1;
    let mut result = String::from("");
    parse_lines(input)
        .iter()
        .enumerate()
        .for_each(|(cycle, instruction)| {
            sprite_position = next_sprite_position;
            if (sprite_position - 1..=sprite_position + 1).contains(&((cycle as isize) % 40)) {
                result.push_str("#");
            } else {
                result.push_str(".");
            }
            match instruction {
                Instruction::ADDX(amount) => next_sprite_position += amount,
                Instruction::NOOP() => (),
            }
        });
    println!("{:#?}", &result[0..39]);
    println!("{:#?}", &result[40..79]);
    println!("{:#?}", &result[80..119]);
    println!("{:#?}", &result[120..159]);
    println!("{:#?}", &result[160..199]);
    println!("{:#?}", &result[200..239]);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "addx 15\naddx -11\naddx 6\naddx -3\naddx 5\naddx -1\naddx -8\naddx 13\naddx 4\nnoop\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx -35\naddx 1\naddx 24\naddx -19\naddx 1\naddx 16\naddx -11\nnoop\nnoop\naddx 21\naddx -15\nnoop\nnoop\naddx -3\naddx 9\naddx 1\naddx -3\naddx 8\naddx 1\naddx 5\nnoop\nnoop\nnoop\nnoop\nnoop\naddx -36\nnoop\naddx 1\naddx 7\nnoop\nnoop\nnoop\naddx 2\naddx 6\nnoop\nnoop\nnoop\nnoop\nnoop\naddx 1\nnoop\nnoop\naddx 7\naddx 1\nnoop\naddx -13\naddx 13\naddx 7\nnoop\naddx 1\naddx -33\nnoop\nnoop\nnoop\naddx 2\nnoop\nnoop\nnoop\naddx 8\nnoop\naddx -1\naddx 2\naddx 1\nnoop\naddx 17\naddx -9\naddx 1\naddx 1\naddx -3\naddx 11\nnoop\nnoop\naddx 1\nnoop\naddx 1\nnoop\nnoop\naddx -13\naddx -19\naddx 1\naddx 3\naddx 26\naddx -30\naddx 12\naddx -1\naddx 3\naddx 1\nnoop\nnoop\nnoop\naddx -9\naddx 18\naddx 1\naddx 2\nnoop\nnoop\naddx 9\nnoop\nnoop\nnoop\naddx -1\naddx 2\naddx -37\naddx 1\naddx 3\nnoop\naddx 15\naddx -21\naddx 22\naddx -6\naddx 1\nnoop\naddx 2\naddx 1\nnoop\naddx -10\nnoop\nnoop\naddx 20\naddx 1\naddx 2\naddx 2\naddx -6\naddx -11\nnoop\nnoop\nnoop";

    #[test]
    fn test_part1() {
        assert_eq!(part1_impl(TEST_INPUT), 13140);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2_impl(TEST_INPUT), "##..##..##..##..##..##..##..##..##..##..###...###...###...###...###...###...###.####....####....####....####....####....#####.....#####.....#####.....#####.....######......######......######......###########.......#######.......#######.....");
    }

    #[test]
    fn test_parse_lines() {
        assert_eq!(
            parse_lines("addx 3\nnoop\naddx 5"),
            vec![
                Instruction::NOOP(),
                Instruction::ADDX(3),
                Instruction::NOOP(),
                Instruction::NOOP(),
                Instruction::ADDX(5)
            ]
        )
    }
}
