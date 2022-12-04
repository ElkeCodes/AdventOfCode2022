use itertools::Itertools;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Assignment {
    from: u64,
    to: u64,
}

pub fn part1(input: String) {
    println!("{}", part1_impl(&input))
}

pub fn part2(input: String) {
    println!("{}", part2_impl(&input))
}

fn parse_into_assignment(input: &str) -> Assignment {
    Assignment {
        from: input.split_once('-').unwrap().0.parse::<u64>().unwrap(),
        to: input.split_once('-').unwrap().1.parse::<u64>().unwrap(),
    }
}

fn parse_lines(input: &str) -> Vec<(Assignment, Assignment)> {
    input
        .lines()
        .enumerate()
        .map(|(_, line)| line.split_once(',').unwrap())
        .map(|(first, second)| (parse_into_assignment(first), parse_into_assignment(second)))
        .collect_vec()
}

fn assignment_contains_assignment((assignment1, assignment2): (Assignment, Assignment)) -> bool {
    (assignment1.from <= assignment2.from && assignment1.to >= assignment2.to)
        || (assignment2.from <= assignment1.from && assignment2.to >= assignment1.to)
}

fn assignment_overlaps_assignment((assignment1, assignment2): (Assignment, Assignment)) -> bool {
    assignment_contains_assignment((assignment1, assignment2))
        || (assignment1.to >= assignment2.from && assignment1.to <= assignment2.from)
        || (assignment2.to >= assignment1.from && assignment2.to <= assignment1.from)
        || (assignment1.from >= assignment2.from && assignment1.from <= assignment2.to)
        || (assignment2.from >= assignment1.from && assignment2.from <= assignment1.to)
}

fn part1_impl(input: &str) -> usize {
    let assignments = parse_lines(input);
    *assignments
        .into_iter()
        .counts_by(assignment_contains_assignment)
        .get(&true)
        .unwrap()
}

fn part2_impl(input: &str) -> usize {
    let assignments = parse_lines(input);
    *assignments
        .into_iter()
        .counts_by(assignment_overlaps_assignment)
        .get(&true)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";

    #[test]
    fn test_part1() {
        assert_eq!(part1_impl(TEST_INPUT), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2_impl(TEST_INPUT), 4);
    }

    #[test]
    fn test_assignment_contains_assignment() {
        assert_eq!(
            assignment_contains_assignment((
                Assignment { from: 2, to: 8 },
                Assignment { from: 3, to: 7 }
            )),
            true
        );
        assert_eq!(
            assignment_contains_assignment((
                Assignment { from: 6, to: 6 },
                Assignment { from: 4, to: 6 }
            )),
            true
        );
        assert_eq!(
            assignment_contains_assignment((
                Assignment { from: 2, to: 6 },
                Assignment { from: 4, to: 8 }
            )),
            false
        );
    }

    #[test]
    fn test_assignment_overlaps_assignment() {
        assert_eq!(
            assignment_overlaps_assignment((
                Assignment { from: 2, to: 4 },
                Assignment { from: 6, to: 8 }
            )),
            false
        );
        assert_eq!(
            assignment_overlaps_assignment((
                Assignment { from: 2, to: 3 },
                Assignment { from: 4, to: 5 }
            )),
            false
        );
        assert_eq!(
            assignment_overlaps_assignment((
                Assignment { from: 5, to: 7 },
                Assignment { from: 7, to: 9 }
            )),
            true
        );
        assert_eq!(
            assignment_overlaps_assignment((
                Assignment { from: 2, to: 8 },
                Assignment { from: 3, to: 7 }
            )),
            true
        );
        assert_eq!(
            assignment_overlaps_assignment((
                Assignment { from: 6, to: 6 },
                Assignment { from: 4, to: 6 }
            )),
            true
        );
        assert_eq!(
            assignment_overlaps_assignment((
                Assignment { from: 2, to: 6 },
                Assignment { from: 4, to: 8 }
            )),
            true
        );
        assert_eq!(
            assignment_overlaps_assignment((
                Assignment { from: 94, to: 96 },
                Assignment { from: 5, to: 86 }
            )),
            false
        );
    }

    #[test]
    fn test_parse_lines() {
        assert_eq!(
            parse_lines(TEST_INPUT),
            vec![
                (Assignment { from: 2, to: 4 }, Assignment { from: 6, to: 8 }),
                (Assignment { from: 2, to: 3 }, Assignment { from: 4, to: 5 }),
                (Assignment { from: 5, to: 7 }, Assignment { from: 7, to: 9 }),
                (Assignment { from: 2, to: 8 }, Assignment { from: 3, to: 7 }),
                (Assignment { from: 6, to: 6 }, Assignment { from: 4, to: 6 }),
                (Assignment { from: 2, to: 6 }, Assignment { from: 4, to: 8 })
            ]
        );
    }

    #[test]
    fn test_parse_into_assignment() {
        assert_eq!(parse_into_assignment("2-4"), Assignment { from: 2, to: 4 });
    }
}
