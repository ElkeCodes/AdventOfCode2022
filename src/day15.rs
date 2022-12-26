use std::{
    cmp,
    collections::{HashMap, HashSet},
    hash::Hash,
    vec,
};

use itertools::Itertools;
use num::abs;

type Coordinate = (isize, isize);

#[derive(Debug, PartialEq, Copy, Clone)]
struct Sensor {
    beacon_position: Coordinate,
    position: Coordinate,
}

pub fn part1(input: String) {
    println!("{}", part1_impl(&input, 2000000))
}

pub fn part2(input: String) {
    println!("{}", part2_impl(&input, 4000000, 4000000))
}

fn parse_coordinate(s: &str) -> Coordinate {
    let mut splits = s.split(", y=");
    let x_part = splits.nth(0).unwrap().trim_start_matches("x=");
    let y_part = splits.nth(0).unwrap();
    (x_part.parse().unwrap(), y_part.parse().unwrap())
}

fn parse_lines(input: &str) -> Vec<Sensor> {
    input
        .lines()
        .map(|line| {
            let mut splits = line.split(": closest beacon is at ");
            let sensor_part = splits.nth(0).unwrap().trim_start_matches("Sensor at ");
            let beacon_part = splits.nth(0).unwrap();
            Sensor {
                beacon_position: parse_coordinate(beacon_part),
                position: parse_coordinate(sensor_part),
            }
        })
        .collect_vec()
}

fn get_manhattan_distance(c1: Coordinate, c2: Coordinate) -> isize {
    abs(c1.0 - c2.0) + abs(c1.1 - c2.1)
}

fn is_in_range(c: Coordinate, sensor: &Sensor) -> bool {
    get_manhattan_distance(c, sensor.position)
        <= get_manhattan_distance(sensor.position, sensor.beacon_position)
}

fn calculate_intersection(
    c11: Coordinate,
    c12: Coordinate,
    c21: Coordinate,
    c22: Coordinate,
) -> Option<Coordinate> {
    let a1 = (c12.1 - c11.1) as i128;
    let b1 = (c11.0 - c12.0) as i128;
    let c1 = a1 * c11.0 as i128 + b1 * c11.1 as i128;

    let a2 = (c22.1 - c21.1) as i128;
    let b2 = (c21.0 - c22.0) as i128;
    let c2 = a2 * c21.0 as i128 + b2 * c21.1 as i128;

    let delta = a1 * b2 - a2 * b1;

    if delta == 0 {
        return None;
    }

    Some((
        ((b2 * c1 - b1 * c2) / delta) as isize,
        ((a1 * c2 - a2 * c1) / delta) as isize,
    ))
}

fn part1_impl(input: &str, y: isize) -> usize {
    let sensors = parse_lines(input);
    let max_x = sensors.iter().fold(0, |acc, sensor| {
        cmp::max(acc, cmp::max(sensor.position.0, sensor.beacon_position.0))
    });
    let min_x = sensors.iter().fold(0, |acc, sensor| {
        cmp::min(acc, cmp::min(sensor.position.0, sensor.beacon_position.0))
    });
    let farthest_ditance = sensors.iter().fold(0 as isize, |acc, sensor| {
        cmp::max(
            acc,
            get_manhattan_distance(sensor.position, sensor.beacon_position),
        )
    });
    let positions_reachable =
        (min_x - farthest_ditance..=max_x + farthest_ditance).fold(vec![], |mut acc, test_x| {
            match sensors
                .iter()
                .find(|&sensor| is_in_range((test_x, y), sensor))
            {
                Some(_) => acc.push((test_x, y)),
                None => (),
            }
            acc
        });
    let filtered_positions_reachable = positions_reachable
        .iter()
        .filter(|&c| {
            sensors
                .iter()
                .find(|sensor| sensor.position == *c || sensor.beacon_position == *c)
                == None
        })
        .collect_vec();
    filtered_positions_reachable.len()
}

fn part2_impl(input: &str, max_x: isize, max_y: isize) -> isize {
    let sensors = parse_lines(input);
    let min_x = 0;
    let min_y = 0;
    let beacon_point = sensors
        .iter()
        .map(|sensor| {
            let manhattan_distance_extra =
                get_manhattan_distance(sensor.position, sensor.beacon_position) + 1;
            let north = (
                sensor.position.0,
                sensor.position.1 + manhattan_distance_extra,
            );
            let east = (
                sensor.position.0 + manhattan_distance_extra,
                sensor.position.1,
            );
            let south = (
                sensor.position.0,
                sensor.position.1 - manhattan_distance_extra,
            );
            let west = (
                sensor.position.0 - manhattan_distance_extra,
                sensor.position.1,
            );
            [(north, east), (east, south), (south, west), (west, north)]
        })
        .flatten()
        .combinations(2)
        .filter_map(|lines| calculate_intersection(lines[0].0, lines[0].1, lines[1].0, lines[1].1))
        .filter(|(x, y)| 0 <= *x && *x <= max_x && 0 <= *y && *y <= max_y)
        .find(|(x, y)| {
            sensors.iter().all(|s| {
                get_manhattan_distance(s.position, s.beacon_position)
                    < get_manhattan_distance(s.position, (*x, *y))
            })
        })
        .unwrap();
    beacon_point.0 * 4_000_000 + beacon_point.1
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    static TEST_INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15\nSensor at x=9, y=16: closest beacon is at x=10, y=16\nSensor at x=13, y=2: closest beacon is at x=15, y=3\nSensor at x=12, y=14: closest beacon is at x=10, y=16\nSensor at x=10, y=20: closest beacon is at x=10, y=16\nSensor at x=14, y=17: closest beacon is at x=10, y=16\nSensor at x=8, y=7: closest beacon is at x=2, y=10\nSensor at x=2, y=0: closest beacon is at x=2, y=10\nSensor at x=0, y=11: closest beacon is at x=2, y=10\nSensor at x=20, y=14: closest beacon is at x=25, y=17\nSensor at x=17, y=20: closest beacon is at x=21, y=22\nSensor at x=16, y=7: closest beacon is at x=15, y=3\nSensor at x=14, y=3: closest beacon is at x=15, y=3\nSensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_part1() {
        assert_eq!(part1_impl(TEST_INPUT, 10), 26);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2_impl(TEST_INPUT, 20, 20), 56000011);
    }

    #[test]
    fn test_parse_lines() {
        assert_eq!(
            parse_lines(TEST_INPUT),
            vec![
                Sensor {
                    beacon_position: (-2, 15),
                    position: (2, 18)
                },
                Sensor {
                    beacon_position: (10, 16),
                    position: (9, 16)
                },
                Sensor {
                    beacon_position: (15, 3),
                    position: (13, 2)
                },
                Sensor {
                    beacon_position: (10, 16),
                    position: (12, 14)
                },
                Sensor {
                    beacon_position: (10, 16),
                    position: (10, 20)
                },
                Sensor {
                    beacon_position: (10, 16),
                    position: (14, 17)
                },
                Sensor {
                    beacon_position: (2, 10),
                    position: (8, 7)
                },
                Sensor {
                    beacon_position: (2, 10),
                    position: (2, 0)
                },
                Sensor {
                    beacon_position: (2, 10),
                    position: (0, 11)
                },
                Sensor {
                    beacon_position: (25, 17),
                    position: (20, 14)
                },
                Sensor {
                    beacon_position: (21, 22),
                    position: (17, 20)
                },
                Sensor {
                    beacon_position: (15, 3),
                    position: (16, 7)
                },
                Sensor {
                    beacon_position: (15, 3),
                    position: (14, 3)
                },
                Sensor {
                    beacon_position: (15, 3),
                    position: (20, 1)
                }
            ]
        )
    }

    #[test]
    fn test_parse_coordinate() {
        assert_eq!(parse_coordinate("x=-2, y=18"), (-2, 18))
    }

    #[test]
    fn test_get_manhattan_distance() {
        assert_eq!(get_manhattan_distance((0, 0), (5, 10)), 15);
        assert_eq!(get_manhattan_distance((23, 6), (17, 9)), 9);
    }

    #[test]
    fn test_is_in_range() {
        assert_eq!(
            is_in_range(
                (0, 0),
                &Sensor {
                    beacon_position: (2, 10),
                    position: (8, 7)
                }
            ),
            false
        );
        assert_eq!(
            is_in_range(
                (2, 10),
                &Sensor {
                    beacon_position: (2, 10),
                    position: (8, 7)
                }
            ),
            true
        );
        assert_eq!(
            is_in_range(
                (1, 10),
                &Sensor {
                    beacon_position: (2, 10),
                    position: (8, 7)
                }
            ),
            false
        );
    }

    #[test]
    fn test_calculate_intersection() {
        assert_eq!(
            calculate_intersection((4, 0), (6, 10), (0, 3), (10, 7)),
            Some((5, 5))
        );
        assert_eq!(calculate_intersection((0, 0), (1, 1), (1, 2), (4, 5)), None)
    }
}
