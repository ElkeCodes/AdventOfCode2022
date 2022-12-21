use std::{cmp, collections::HashMap};

use itertools::Itertools;

#[derive(Debug, PartialEq)]
enum Element {
    Rock,
    Sand,
}

type Coordinate = (usize, usize);

type Grid = HashMap<Coordinate, Element>;

pub fn part1(input: String) {
    println!("{}", part1_impl(&input))
}

pub fn part2(input: String) {
    println!("{}", part2_impl(&input))
}

fn get_coordinates_between_points(c1: Coordinate, c2: Coordinate) -> Vec<Coordinate> {
    let min_x = cmp::min(c1.0, c2.0);
    let max_x = cmp::max(c1.0, c2.0);
    let min_y = cmp::min(c1.1, c2.1);
    let max_y = cmp::max(c1.1, c2.1);
    (min_x..=max_x)
        .map(|x| (min_y..=max_y).map(move |y| (x, y)))
        .flatten()
        .collect_vec()
}

fn parse_coordinate(s: &str) -> Coordinate {
    let mut splits = s.split(",");
    (
        splits.nth(0).unwrap().parse().unwrap(),
        splits.nth(0).unwrap().parse().unwrap(),
    )
}

fn parse_lines(input: &str) -> (Grid, usize, usize, usize) {
    let mut result = Grid::new();
    let mut max_y = 0;
    let mut min_x = 500;
    let mut max_x = 500;
    input.lines().for_each(|line| {
        let mut parts = line.split(" -> ");
        let mut start_part = parts.next().unwrap();
        parts.for_each(|part| {
            get_coordinates_between_points(parse_coordinate(start_part), parse_coordinate(part))
                .iter()
                .for_each(|c| {
                    result.insert(*c, Element::Rock);
                    max_y = cmp::max(max_y, c.1);
                    min_x = cmp::min(min_x, c.0);
                    max_x = cmp::max(max_x, c.0);
                    start_part = part;
                })
        });
    });
    (result, max_y, min_x, max_x)
}

fn drop_sand_one_down(sand_coordinate: Coordinate, grid: &Grid) -> Coordinate {
    let down_coordinate = (sand_coordinate.0, sand_coordinate.1 + 1);
    let down_left_coordinate = (sand_coordinate.0 - 1, sand_coordinate.1 + 1);
    let down_right_coordinate = (sand_coordinate.0 + 1, sand_coordinate.1 + 1);
    match grid.get(&down_coordinate) {
        Some(_) => match grid.get(&down_left_coordinate) {
            Some(_) => match grid.get(&down_right_coordinate) {
                Some(_) => sand_coordinate,
                None => down_right_coordinate,
            },
            None => down_left_coordinate,
        },
        None => down_coordinate,
    }
}

fn part1_impl(input: &str) -> usize {
    let (mut grid, max_y, _, _) = parse_lines(input);
    let mut sand_dropped = 0;
    'outer: loop {
        let mut sand_coordinate: Coordinate = (500, 0);
        sand_dropped += 1;
        loop {
            let new_coordinate = drop_sand_one_down(sand_coordinate, &grid);
            if sand_coordinate.1 > max_y {
                break 'outer;
            }
            if new_coordinate == sand_coordinate {
                break;
            }
            sand_coordinate = new_coordinate;
        }
        grid.insert(sand_coordinate, Element::Sand);
    }
    sand_dropped - 1
}

fn part2_impl(input: &str) -> usize {
    let (mut grid, max_y, min_x, max_x) = parse_lines(input);
    (min_x - max_y..=max_x + max_y).for_each(|x| {
        grid.insert((x, max_y + 2), Element::Rock);
    });
    let mut sand_dropped = 0;
    'outer: loop {
        let mut sand_coordinate: Coordinate = (500, 0);
        sand_dropped += 1;
        loop {
            let new_coordinate = drop_sand_one_down(sand_coordinate, &grid);
            if new_coordinate == (500, 0) {
                break 'outer;
            }
            if new_coordinate == sand_coordinate {
                break;
            }
            sand_coordinate = new_coordinate;
        }
        grid.insert(sand_coordinate, Element::Sand);
    }
    sand_dropped
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    static TEST_INPUT: &str = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_part1() {
        assert_eq!(part1_impl(TEST_INPUT), 24);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2_impl(TEST_INPUT), 93);
    }

    #[test]
    fn test_parse_lines() {
        assert_eq!(
            parse_lines(TEST_INPUT),
            (
                Grid::from([
                    ((498, 4), Element::Rock),
                    ((498, 5), Element::Rock),
                    ((498, 6), Element::Rock),
                    ((497, 6), Element::Rock),
                    ((496, 6), Element::Rock),
                    ((503, 4), Element::Rock),
                    ((503, 4), Element::Rock),
                    ((502, 4), Element::Rock),
                    ((502, 5), Element::Rock),
                    ((502, 6), Element::Rock),
                    ((502, 7), Element::Rock),
                    ((502, 8), Element::Rock),
                    ((502, 9), Element::Rock),
                    ((501, 9), Element::Rock),
                    ((500, 9), Element::Rock),
                    ((499, 9), Element::Rock),
                    ((498, 9), Element::Rock),
                    ((497, 9), Element::Rock),
                    ((496, 9), Element::Rock),
                    ((495, 9), Element::Rock),
                    ((494, 9), Element::Rock),
                ]),
                9,
                494,
                503
            )
        )
    }

    #[test]
    fn test_parse_coordinate() {
        assert_eq!(parse_coordinate("498,4"), (498, 4))
    }

    #[test]
    fn test_get_coordinates_between_points() {
        assert_eq!(
            get_coordinates_between_points((498, 4), (498, 6)),
            vec![(498, 4), (498, 5), (498, 6)]
        );
        assert_eq!(
            get_coordinates_between_points((503, 4), (502, 4)),
            vec![(502, 4), (503, 4)]
        );
        assert_eq!(
            get_coordinates_between_points((502, 4), (502, 9)),
            vec![(502, 4), (502, 5), (502, 6), (502, 7), (502, 8), (502, 9)]
        );
    }

    #[test]
    fn test_drop_sand_one_down() {
        let (mut grid, _, _, _) = parse_lines(TEST_INPUT);
        assert_eq!(drop_sand_one_down((500, 0), &grid), (500, 1));
        assert_eq!(drop_sand_one_down((500, 1), &grid), (500, 2));
        assert_eq!(drop_sand_one_down((500, 2), &grid), (500, 3));
        assert_eq!(drop_sand_one_down((500, 3), &grid), (500, 4));
        assert_eq!(drop_sand_one_down((500, 4), &grid), (500, 5));
        assert_eq!(drop_sand_one_down((500, 5), &grid), (500, 6));
        assert_eq!(drop_sand_one_down((500, 6), &grid), (500, 7));
        assert_eq!(drop_sand_one_down((500, 7), &grid), (500, 8));
        assert_eq!(drop_sand_one_down((500, 8), &grid), (500, 8));
        grid.insert((500, 8), Element::Sand);
        assert_eq!(drop_sand_one_down((500, 7), &grid), (499, 8));
        assert_eq!(drop_sand_one_down((499, 8), &grid), (499, 8));
        grid.insert((499, 8), Element::Sand);
        assert_eq!(drop_sand_one_down((500, 7), &grid), (501, 8));
        assert_eq!(drop_sand_one_down((501, 8), &grid), (501, 8));
        grid.insert((501, 8), Element::Sand);
        assert_eq!(drop_sand_one_down((500, 7), &grid), (500, 7));
        grid.insert((500, 7), Element::Sand);
        assert_eq!(drop_sand_one_down((500, 6), &grid), (499, 7));
        assert_eq!(drop_sand_one_down((499, 7), &grid), (498, 8));
        assert_eq!(drop_sand_one_down((498, 8), &grid), (498, 8));
        grid.insert((498, 8), Element::Sand);
        assert_eq!(
            grid,
            Grid::from([
                ((498, 4), Element::Rock),
                ((498, 5), Element::Rock),
                ((498, 6), Element::Rock),
                ((497, 6), Element::Rock),
                ((496, 6), Element::Rock),
                ((503, 4), Element::Rock),
                ((503, 4), Element::Rock),
                ((502, 4), Element::Rock),
                ((502, 5), Element::Rock),
                ((502, 6), Element::Rock),
                ((502, 7), Element::Rock),
                ((502, 8), Element::Rock),
                ((502, 9), Element::Rock),
                ((501, 9), Element::Rock),
                ((500, 9), Element::Rock),
                ((499, 9), Element::Rock),
                ((498, 9), Element::Rock),
                ((497, 9), Element::Rock),
                ((496, 9), Element::Rock),
                ((495, 9), Element::Rock),
                ((494, 9), Element::Rock),
                ((500, 8), Element::Sand),
                ((499, 8), Element::Sand),
                ((501, 8), Element::Sand),
                ((500, 7), Element::Sand),
                ((498, 8), Element::Sand),
            ])
        );
    }
}
