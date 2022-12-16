use std::{cmp, collections::HashMap};

type Coordinate = (usize, usize);
type Tree = u32;
type Trees = HashMap<Coordinate, Tree>;

pub fn part1(input: String) {
    println!("{}", part1_impl(&input))
}

pub fn part2(input: String) {
    println!("{}", part2_impl(&input))
}

fn parse_lines(input: &str) -> (Trees, usize, usize) {
    let mut grid = Trees::new();
    let mut max_x = 0;
    let mut max_y = 0;
    input.lines().enumerate().for_each(|(y, line)| {
        max_y = cmp::max(max_y, y);
        line.chars().enumerate().for_each(|(x, tree)| {
            max_x = cmp::max(max_x, x);
            grid.entry((x, y)).or_insert(tree.to_digit(10).unwrap());
        });
    });
    (grid, max_x, max_y)
}

fn is_visible_from_outside((x, y): Coordinate, trees: &Trees, max_x: usize, max_y: usize) -> bool {
    let test_tree = trees.get(&(x, y)).unwrap();
    (0..x).fold(true, |acc, test_x| {
        acc && trees.get(&(test_x, y)).unwrap() < test_tree
    }) || (0..y).fold(true, |acc, test_y| {
        acc && trees.get(&(x, test_y)).unwrap() < test_tree
    }) || (x + 1..=max_x).fold(true, |acc, test_x| {
        acc && trees.get(&(test_x, y)).unwrap() < test_tree
    }) || (y + 1..=max_y).fold(true, |acc, test_y| {
        acc && trees.get(&(x, test_y)).unwrap() < test_tree
    })
}

fn get_total_viewing_distance(
    (x, y): Coordinate,
    trees: &Trees,
    max_x: usize,
    max_y: usize,
) -> usize {
    let base_tree = trees.get(&(x, y)).unwrap();
    (0..x)
        .rfold((true, 0), |(cont, distance), test_x| {
            let test_tree = trees.get(&(test_x, y)).unwrap();
            if cont {
                (cont && base_tree > test_tree, distance + 1)
            } else {
                (false, distance)
            }
        })
        .1
        * (0..y)
            .rfold((true, 0), |(cont, distance), test_y| {
                let test_tree = trees.get(&(x, test_y)).unwrap();
                if cont {
                    (cont && base_tree > test_tree, distance + 1)
                } else {
                    (false, distance)
                }
            })
            .1
        * (x + 1..=max_x)
            .fold((true, 0), |(cont, distance), test_x| {
                let test_tree = trees.get(&(test_x, y)).unwrap();
                if cont {
                    (cont && base_tree > test_tree, distance + 1)
                } else {
                    (false, distance)
                }
            })
            .1
        * (y + 1..=max_y)
            .fold((true, 0), |(cont, distance), test_y| {
                let test_tree = trees.get(&(x, test_y)).unwrap();
                if cont {
                    (cont && base_tree > test_tree, distance + 1)
                } else {
                    (false, distance)
                }
            })
            .1
}

fn part1_impl(input: &str) -> usize {
    let (trees, max_x, max_y) = parse_lines(input);
    trees.clone().into_iter().fold(0, |acc, ((x, y), _)| {
        if is_visible_from_outside((x, y), &trees, max_x, max_y) {
            acc + 1
        } else {
            acc
        }
    })
}

fn part2_impl(input: &str) -> usize {
    let (trees, max_x, max_y) = parse_lines(input);
    trees.clone().into_iter().fold(0, |acc, ((x, y), _)| {
        cmp::max(
            acc,
            get_total_viewing_distance((x, y), &trees, max_x, max_y),
        )
    })
}
#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "30373\n25512\n65332\n33549\n35390";

    #[test]
    fn test_part1() {
        assert_eq!(part1_impl(TEST_INPUT), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2_impl(TEST_INPUT), 8);
    }

    #[test]
    fn test_parse_lines() {
        let parsed_result = parse_lines(TEST_INPUT);
        assert_eq!(
            parsed_result,
            (
                Trees::from([
                    // row 1
                    ((0, 0), 3),
                    ((1, 0), 0),
                    ((2, 0), 3),
                    ((3, 0), 7),
                    ((4, 0), 3),
                    // row 2
                    ((0, 1), 2),
                    ((1, 1), 5),
                    ((2, 1), 5),
                    ((3, 1), 1),
                    ((4, 1), 2),
                    // row 3
                    ((0, 2), 6),
                    ((1, 2), 5),
                    ((2, 2), 3),
                    ((3, 2), 3),
                    ((4, 2), 2),
                    // row 4
                    ((0, 3), 3),
                    ((1, 3), 3),
                    ((2, 3), 5),
                    ((3, 3), 4),
                    ((4, 3), 9),
                    // row 5
                    ((0, 4), 3),
                    ((1, 4), 5),
                    ((2, 4), 3),
                    ((3, 4), 9),
                    ((4, 4), 0),
                ]),
                4,
                4
            )
        );
    }

    #[test]
    fn test_is_visible_from_outside() {
        let (trees, _, _) = parse_lines(TEST_INPUT);
        assert_eq!(is_visible_from_outside((0, 0), &trees, 4, 4), true);
        assert_eq!(is_visible_from_outside((1, 0), &trees, 4, 4), true);
        assert_eq!(is_visible_from_outside((2, 0), &trees, 4, 4), true);
        assert_eq!(is_visible_from_outside((3, 0), &trees, 4, 4), true);
        assert_eq!(is_visible_from_outside((4, 0), &trees, 4, 4), true);
        assert_eq!(is_visible_from_outside((0, 1), &trees, 4, 4), true);
        assert_eq!(is_visible_from_outside((1, 1), &trees, 4, 4), true);
        assert_eq!(is_visible_from_outside((2, 1), &trees, 4, 4), true);
        assert_eq!(is_visible_from_outside((3, 1), &trees, 4, 4), false);
        assert_eq!(is_visible_from_outside((4, 1), &trees, 4, 4), true);
        assert_eq!(is_visible_from_outside((0, 2), &trees, 4, 4), true);
        assert_eq!(is_visible_from_outside((1, 2), &trees, 4, 4), true);
        assert_eq!(is_visible_from_outside((2, 2), &trees, 4, 4), false);
        assert_eq!(is_visible_from_outside((3, 2), &trees, 4, 4), true);
        assert_eq!(is_visible_from_outside((4, 2), &trees, 4, 4), true);
        assert_eq!(is_visible_from_outside((0, 3), &trees, 4, 4), true);
        assert_eq!(is_visible_from_outside((1, 3), &trees, 4, 4), false);
        assert_eq!(is_visible_from_outside((2, 3), &trees, 4, 4), true);
        assert_eq!(is_visible_from_outside((3, 3), &trees, 4, 4), false);
        assert_eq!(is_visible_from_outside((4, 3), &trees, 4, 4), true);
        assert_eq!(is_visible_from_outside((0, 4), &trees, 4, 4), true);
        assert_eq!(is_visible_from_outside((1, 4), &trees, 4, 4), true);
        assert_eq!(is_visible_from_outside((2, 4), &trees, 4, 4), true);
        assert_eq!(is_visible_from_outside((3, 4), &trees, 4, 4), true);
        assert_eq!(is_visible_from_outside((4, 4), &trees, 4, 4), true);
    }

    #[test]
    fn test_get_total_viewing_distance() {
        let (trees, _, _) = parse_lines(TEST_INPUT);
        assert_eq!(get_total_viewing_distance((2, 1), &trees, 4, 4), 4);
        assert_eq!(get_total_viewing_distance((2, 3), &trees, 4, 4), 8);
    }
}
