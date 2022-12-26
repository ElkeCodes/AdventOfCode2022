use std::{cmp, collections::HashMap};

use itertools::Itertools;
use num::abs;

#[derive(Debug, PartialEq, Copy, Clone)]
struct Valve {
    flow_rate: usize,
    tunnels: Vec<&str>,
}

pub fn part1(input: String) {
    println!("{}", part1_impl(&input, 2000000))
}

pub fn part2(input: String) {
    println!("{}", part2_impl(&input, 4000000, 4000000))
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

fn part1_impl(input: &str, y: isize) -> usize {
    0
}

fn part2_impl(input: &str, max_x: isize, max_y: isize) -> isize {
    0
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    static TEST_INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB\nValve BB has flow rate=13; tunnels lead to valves CC, AA\nValve CC has flow rate=2; tunnels lead to valves DD, BB\nValve DD has flow rate=20; tunnels lead to valves CC, AA, EE\nValve EE has flow rate=3; tunnels lead to valves FF, DD\nValve FF has flow rate=0; tunnels lead to valves EE, GG\nValve GG has flow rate=0; tunnels lead to valves FF, HH\nValve HH has flow rate=22; tunnel leads to valve GG\nValve II has flow rate=0; tunnels lead to valves AA, JJ\nValve JJ has flow rate=21; tunnel leads to valve II";

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
}
