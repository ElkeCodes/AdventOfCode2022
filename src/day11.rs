use num::integer::Integer;
use std::{borrow::BorrowMut, result, vec};
use num_bigint::BigUint;

use itertools::Itertools;

// BigUint
type Item = u128;

#[derive(Debug, PartialEq, Copy, Clone)]
enum OperationType {
    Addition,
    Multiplication,
}
#[derive(Debug, PartialEq, Copy, Clone)]
enum Parameter {
    Old,
    Number(Item),
}
#[derive(Debug, PartialEq, Copy, Clone)]
struct Operation {
    first_parameter: Parameter,
    operation_type: OperationType,
    second_parameter: Parameter,
}
#[derive(Debug, PartialEq, Copy, Clone)]
struct Test {
    divisible_by: Item,
    true_throw_destination: usize,
    false_throw_destination: usize,
}

#[derive(Debug, PartialEq, Clone)]
struct Monkey {
    inspections: usize,
    items: Vec<Item>,
    operation: Operation,
    test: Test,
}

impl Monkey {
    pub fn add_item(&mut self, item: Item) {
        self.items.push(item)
    }
}

pub fn part1(input: String) {
    println!("{}", part1_impl(&input))
}

pub fn part2(input: String) {
    println!("{}", part2_impl(&input))
}

fn parse_lines(input: &str) -> Vec<Monkey> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .chunks(6)
        .into_iter()
        .map(|mut monkey_data| {
            let items = monkey_data
                .nth(1)
                .unwrap()
                .split(": ")
                .nth(1)
                .unwrap()
                .split(", ")
                .map(|x| x.parse::<Item>().unwrap())
                .collect_vec();
            let operation_data = monkey_data
                .nth(0)
                .unwrap()
                .split(": new = ")
                .nth(1)
                .unwrap();
            let first_parameter = operation_data.split(" ").nth(0).unwrap();
            let operation_type = operation_data.split(" ").nth(1).unwrap();
            let second_parameter = operation_data.split(" ").nth(2).unwrap();
            Monkey {
                inspections: 0,
                items: items,
                operation: Operation {
                    first_parameter: if first_parameter.starts_with("old") {
                        Parameter::Old
                    } else {
                        Parameter::Number(first_parameter.parse::<Item>().unwrap())
                    },
                    operation_type: if operation_type == "*" {
                        OperationType::Multiplication
                    } else {
                        OperationType::Addition
                    },
                    second_parameter: if second_parameter.starts_with("old") {
                        Parameter::Old
                    } else {
                        Parameter::Number(second_parameter.parse::<Item>().unwrap())
                    },
                },
                test: Test {
                    divisible_by: monkey_data
                        .nth(0)
                        .unwrap()
                        .split(" divisible by ")
                        .nth(1)
                        .unwrap()
                        .parse::<Item>()
                        .unwrap(),
                    true_throw_destination: monkey_data
                        .nth(0)
                        .unwrap()
                        .split("throw to monkey ")
                        .nth(1)
                        .unwrap()
                        .parse::<usize>()
                        .unwrap(),
                    false_throw_destination: monkey_data
                        .nth(0)
                        .unwrap()
                        .split("throw to monkey ")
                        .nth(1)
                        .unwrap()
                        .parse::<usize>()
                        .unwrap(),
                },
            }
        })
        .collect_vec()
}

fn get_parameter(parameter: Parameter, item: Item) -> Item {
    match parameter {
        Parameter::Number(n) => n,
        Parameter::Old => item,
    }
}

fn perform_operation(operation: Operation, item: Item) -> Item {
    match operation.operation_type {
        OperationType::Addition => {
            get_parameter(operation.first_parameter, item)
                + get_parameter(operation.second_parameter, item)
        }
        OperationType::Multiplication => {
            get_parameter(operation.first_parameter, item)
                * get_parameter(operation.second_parameter, item)
        }
    }
}

fn perform_test(test: Test, item: Item) -> bool {
    item % test.divisible_by == 0
}

fn perform_round(monkeys: &mut Vec<Monkey>, should_divide: bool) -> &mut Vec<Monkey> {
    for monkey_id in 0..monkeys.len() {
        let mut move_items: Vec<(usize, Item)> = vec![];
        {
            let monkey = &mut monkeys[monkey_id];
            monkey.inspections += monkey.items.len();
            for item_id in 0..monkey.items.len() {
                let item = monkey.items[item_id];
                let calculated_item = perform_operation(monkey.operation, item);
                let result_item = if should_divide {
                    calculated_item.div_floor(&3)
                } else {
                    calculated_item
                };
                if perform_test(monkey.test, result_item) {
                    move_items.push((monkey.test.true_throw_destination, result_item));
                } else {
                    move_items.push((monkey.test.false_throw_destination, result_item));
                }
            }
            monkey.items = vec![];
        }
        move_items
            .iter()
            .for_each(|(destination, item): &(usize, Item)| {
                monkeys[*destination].add_item(*item);
            });
    }
    monkeys
}

fn part1_impl(input: &str) -> usize {
    let mut parsed_monkeys = parse_lines(input);
    let mut monkeys = parsed_monkeys.borrow_mut();
    for _ in 0..20 {
        monkeys = perform_round(monkeys, true);
    }
    monkeys
        .iter()
        .map(|monkey| monkey.inspections)
        .sorted()
        .rev()
        .take(2)
        .product()
}

fn part2_impl(input: &str) -> usize {
    let mut parsed_monkeys = parse_lines(input);
    let mut monkeys = parsed_monkeys.borrow_mut();
    for _ in 0..1000 {
        monkeys = perform_round(monkeys, false);
    }
    monkeys
        .iter()
        .map(|monkey| monkey.inspections)
        .sorted()
        .rev()
        .take(2)
        .product()
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    static TEST_INPUT: &str = "Monkey 0:\n    Starting items: 79, 98\n    Operation: new = old * 19\n    Test: divisible by 23\n      If true: throw to monkey 2\n      If false: throw to monkey 3\n  \n  Monkey 1:\n    Starting items: 54, 65, 75, 74\n    Operation: new = old + 6\n    Test: divisible by 19\n      If true: throw to monkey 2\n      If false: throw to monkey 0\n  \n  Monkey 2:\n    Starting items: 79, 60, 97\n    Operation: new = old * old\n    Test: divisible by 13\n      If true: throw to monkey 1\n      If false: throw to monkey 3\n  \n  Monkey 3:\n    Starting items: 74\n    Operation: new = old + 3\n    Test: divisible by 17\n      If true: throw to monkey 0\n      If false: throw to monkey 1";

    #[test]
    fn test_part1() {
        assert_eq!(part1_impl(TEST_INPUT), 10605);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2_impl(TEST_INPUT), 2713310158);
    }

    #[test]
    fn test_parse_lines() {
        let monkeys = parse_lines(TEST_INPUT);
        assert_eq!(
            monkeys[0],
            Monkey {
                inspections: 0,
                items: vec![79, 98],
                operation: Operation {
                    first_parameter: Parameter::Old,
                    operation_type: OperationType::Multiplication,
                    second_parameter: Parameter::Number(19)
                },
                test: Test {
                    divisible_by: 23,
                    true_throw_destination: 2,
                    false_throw_destination: 3
                }
            }
        );
        assert_eq!(
            monkeys[1],
            Monkey {
                inspections: 0,
                items: vec![54, 65, 75, 74],
                operation: Operation {
                    first_parameter: Parameter::Old,
                    operation_type: OperationType::Addition,
                    second_parameter: Parameter::Number(6)
                },
                test: Test {
                    divisible_by: 19,
                    true_throw_destination: 2,
                    false_throw_destination: 0
                }
            }
        );
        assert_eq!(
            monkeys[2],
            Monkey {
                inspections: 0,
                items: vec![79, 60, 97],
                operation: Operation {
                    first_parameter: Parameter::Old,
                    operation_type: OperationType::Multiplication,
                    second_parameter: Parameter::Old
                },
                test: Test {
                    divisible_by: 13,
                    true_throw_destination: 1,
                    false_throw_destination: 3
                }
            }
        );
        assert_eq!(
            monkeys[3],
            Monkey {
                inspections: 0,
                items: vec![74],
                operation: Operation {
                    first_parameter: Parameter::Old,
                    operation_type: OperationType::Addition,
                    second_parameter: Parameter::Number(3)
                },
                test: Test {
                    divisible_by: 17,
                    true_throw_destination: 0,
                    false_throw_destination: 1
                }
            }
        )
    }

    #[test]
    fn test_perform_round() {
        let mut monkeys = vec![
            Monkey {
                inspections: 0,
                items: vec![79, 98],
                operation: Operation {
                    first_parameter: Parameter::Old,
                    operation_type: OperationType::Multiplication,
                    second_parameter: Parameter::Number(19),
                },
                test: Test {
                    divisible_by: 23,
                    true_throw_destination: 2,
                    false_throw_destination: 3,
                },
            },
            Monkey {
                inspections: 0,
                items: vec![54, 65, 75, 74],
                operation: Operation {
                    first_parameter: Parameter::Old,
                    operation_type: OperationType::Addition,
                    second_parameter: Parameter::Number(6),
                },
                test: Test {
                    divisible_by: 19,
                    true_throw_destination: 2,
                    false_throw_destination: 0,
                },
            },
            Monkey {
                inspections: 0,
                items: vec![79, 60, 97],
                operation: Operation {
                    first_parameter: Parameter::Old,
                    operation_type: OperationType::Multiplication,
                    second_parameter: Parameter::Old,
                },
                test: Test {
                    divisible_by: 13,
                    true_throw_destination: 1,
                    false_throw_destination: 3,
                },
            },
            Monkey {
                inspections: 0,
                items: vec![74],
                operation: Operation {
                    first_parameter: Parameter::Old,
                    operation_type: OperationType::Addition,
                    second_parameter: Parameter::Number(3),
                },
                test: Test {
                    divisible_by: 17,
                    true_throw_destination: 0,
                    false_throw_destination: 1,
                },
            },
        ];
        assert_eq!(
            *perform_round(&mut monkeys, true),
            vec![
                Monkey {
                    inspections: 2,
                    items: vec![20, 23, 27, 26],
                    operation: Operation {
                        first_parameter: Parameter::Old,
                        operation_type: OperationType::Multiplication,
                        second_parameter: Parameter::Number(19),
                    },
                    test: Test {
                        divisible_by: 23,
                        true_throw_destination: 2,
                        false_throw_destination: 3,
                    },
                },
                Monkey {
                    inspections: 4,
                    items: vec![2080, 25, 167, 207, 401, 1046],
                    operation: Operation {
                        first_parameter: Parameter::Old,
                        operation_type: OperationType::Addition,
                        second_parameter: Parameter::Number(6),
                    },
                    test: Test {
                        divisible_by: 19,
                        true_throw_destination: 2,
                        false_throw_destination: 0,
                    },
                },
                Monkey {
                    inspections: 3,
                    items: vec![],
                    operation: Operation {
                        first_parameter: Parameter::Old,
                        operation_type: OperationType::Multiplication,
                        second_parameter: Parameter::Old,
                    },
                    test: Test {
                        divisible_by: 13,
                        true_throw_destination: 1,
                        false_throw_destination: 3,
                    },
                },
                Monkey {
                    inspections: 5,
                    items: vec![],
                    operation: Operation {
                        first_parameter: Parameter::Old,
                        operation_type: OperationType::Addition,
                        second_parameter: Parameter::Number(3),
                    },
                    test: Test {
                        divisible_by: 17,
                        true_throw_destination: 0,
                        false_throw_destination: 1,
                    },
                },
            ]
        )
    }

    #[test]
    fn test_get_parameter() {
        assert_eq!(get_parameter(Parameter::Number(5), 3), 5);
        assert_eq!(get_parameter(Parameter::Old, 3), 3);
    }

    #[test]
    fn test_perform_operation() {
        assert_eq!(
            perform_operation(
                Operation {
                    first_parameter: Parameter::Old,
                    operation_type: OperationType::Multiplication,
                    second_parameter: Parameter::Number(19)
                },
                79
            ),
            1501
        );
        assert_eq!(
            perform_operation(
                Operation {
                    first_parameter: Parameter::Old,
                    operation_type: OperationType::Addition,
                    second_parameter: Parameter::Number(6)
                },
                54
            ),
            60
        );
        assert_eq!(
            perform_operation(
                Operation {
                    first_parameter: Parameter::Old,
                    operation_type: OperationType::Multiplication,
                    second_parameter: Parameter::Old
                },
                79
            ),
            6241
        )
    }

    #[test]
    fn test_perform_test() {
        assert_eq!(
            perform_test(
                Test {
                    divisible_by: 17,
                    true_throw_destination: 0,
                    false_throw_destination: 1
                },
                34
            ),
            true
        );
        assert_eq!(
            perform_test(
                Test {
                    divisible_by: 17,
                    true_throw_destination: 0,
                    false_throw_destination: 1
                },
                32
            ),
            false
        );
    }
}
