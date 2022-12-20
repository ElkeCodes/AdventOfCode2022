use std::{cmp::Ordering, slice::Iter};
use itertools::Itertools;

#[derive(Debug, PartialEq)]
enum Token {
    Open,
    Close,
    Comma,
    Digit(u32),
}
type Tokens = Vec<Token>;

#[derive(Debug, PartialEq, Clone)]
enum Packet {
    List(Vec<Packet>),
    Number(u32),
}

pub fn part1(input: String) {
    println!("{}", part1_impl(&input))
}

pub fn part2(input: String) {
    println!("{}", part2_impl(&input))
}

fn parse_line(input: &str) -> Tokens {
    let mut input_iter = input.chars().peekable();
    let mut tokens = Tokens::new();
    loop {
        let peek = input_iter.peek();
        match peek {
            None => break,
            Some('[') => {
                input_iter.next();
                tokens.push(Token::Open)
            }
            Some(']') => {
                input_iter.next();
                tokens.push(Token::Close)
            }
            Some(',') => {
                input_iter.next();
                tokens.push(Token::Comma)
            }
            Some(x) => {
                let mut digits = String::new();
                digits.push(*x);
                input_iter.next();

                loop {
                    let next_digit = match input_iter.peek() {
                        None => break,
                        Some(c) => *c,
                    };

                    if !next_digit.is_digit(10) {
                        break;
                    }

                    input_iter.next();
                    digits.push(next_digit);
                }
                tokens.push(Token::Digit(digits.parse().unwrap()))
            }
        }
    }
    tokens
}

fn parse_lines(input: &str) -> Vec<(Packet, Packet)> {
    let mut lines = input.lines().peekable();
    let mut result = vec![];
    while let Some(_) = lines.peek() {
        let left = lines.nth(0).unwrap();
        let right = lines.nth(0).unwrap();
        result.push((
            parse_tokens_to_packets(parse_line(left)),
            parse_tokens_to_packets(parse_line(right)),
        ));
        lines.next();
    }
    result
}

fn parse_tokens_to_packets_inner(token_iter: &mut Iter<Token>) -> Vec<Packet> {
    let mut elems = vec![];

    let mut current_token = token_iter.next().unwrap();
    match current_token {
        Token::Close => elems,
        _ => {
            loop {
                match current_token {
                    Token::Digit(x) => elems.push(Packet::Number(*x)),
                    Token::Open => {
                        elems.push(Packet::List(parse_tokens_to_packets_inner(token_iter)))
                    }
                    _ => panic!("unexpected token 1"),
                }
                match token_iter.next().unwrap() {
                    Token::Close => break,
                    Token::Comma => {
                        current_token = token_iter.next().unwrap();
                        continue;
                    }
                    token => panic!("unexpected token {:#?}", token),
                }
            }
            elems
        }
    }
}

fn parse_tokens_to_packets(tokens: Tokens) -> Packet {
    let mut token_iter = tokens.iter();
    token_iter.next();

    Packet::List(parse_tokens_to_packets_inner(&mut token_iter))
}

fn compare_packets(left_packet: &Packet, right_packet: &Packet) -> Ordering {
    match (left_packet, right_packet) {
        (Packet::Number(lx), Packet::Number(rx)) => lx.cmp(rx),
        (Packet::Number(lx), Packet::List(_)) => {
            compare_packets(&Packet::List(vec![Packet::Number(*lx)]), right_packet)
        }
        (Packet::List(_), Packet::Number(rx)) => {
            compare_packets(left_packet, &Packet::List(vec![Packet::Number(*rx)]))
        }
        (Packet::List(l_contents), Packet::List(r_contents)) => {
            let mut l_contents_iter = l_contents.iter();
            let mut r_contents_iter = r_contents.iter();

            loop {
                let left = l_contents_iter.next();
                let right = r_contents_iter.next();

                match (left, right) {
                    (None, Some(_)) => return Ordering::Less,
                    (Some(_), None) => return Ordering::Greater,
                    (None, None) => return Ordering::Equal,
                    (Some(left), Some(right)) => match compare_packets(left, right) {
                        Ordering::Equal => continue,
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return Ordering::Greater,
                    },
                }
            }
        }
    }
}

fn part1_impl(input: &str) -> usize {
    let packets = parse_lines(input);
    packets
        .iter()
        .enumerate()
        .fold(
            0,
            |acc, (index, (left_packet, right_packet))| match compare_packets(
                left_packet,
                right_packet,
            ) {
                Ordering::Greater => acc,
                Ordering::Equal => acc,
                Ordering::Less => acc + index + 1,
            },
        )
}

fn part2_impl(input: &str) -> usize {
    let mut packets = parse_lines(input);
    packets.push((
        Packet::List(vec![Packet::List(vec![Packet::Number(2)])]),
        Packet::List(vec![Packet::List(vec![Packet::Number(6)])]),
    ));
    packets
        .iter()
        .fold(vec![], |mut acc, (left_packet, right_packet)| {
            acc.push(left_packet);
            acc.push(right_packet);
            acc
        })
        .iter()
        .sorted_by(|&&p1, &&p2| compare_packets(p1, p2))
        .enumerate()
        .fold(1, |acc, (index, &packet)| {
            if *packet == Packet::List(vec![Packet::List(vec![Packet::Number(2)])])
                || *packet == Packet::List(vec![Packet::List(vec![Packet::Number(6)])])
            {
                acc * (index + 1)
            } else {
                acc
            }
        })
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    static TEST_INPUT_SHORT: &str = "[[1],[2,3,4]]\n[[1],4]";
    static TEST_INPUT: &str = "[1,1,3,1,1]\n[1,1,5,1,1]\n\n[[1],[2,3,4]]\n[[1],4]\n\n[9]\n[[8,7,6]]\n\n[[4,4],4,4]\n[[4,4],4,4,4]\n\n[7,7,7,7]\n[7,7,7]\n\n[]\n[3]\n\n[[[]]]\n[[]]\n\n[1,[2,[3,[4,[5,6,7]]]],8,9]\n[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_part1() {
        assert_eq!(part1_impl(TEST_INPUT), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2_impl(TEST_INPUT), 140);
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("[[1],[20,3,4]]"),
            Tokens::from(vec![
                Token::Open,
                Token::Open,
                Token::Digit(1),
                Token::Close,
                Token::Comma,
                Token::Open,
                Token::Digit(20),
                Token::Comma,
                Token::Digit(3),
                Token::Comma,
                Token::Digit(4),
                Token::Close,
                Token::Close,
            ])
        );
    }

    #[test]
    fn test_parse_lines() {
        assert_eq!(
            parse_lines(TEST_INPUT_SHORT),
            vec![(
                Packet::List(vec![
                    Packet::List(vec![Packet::Number(1)]),
                    Packet::List(vec![Packet::Number(2), Packet::Number(3), Packet::Number(4)])
                ]),
                Packet::List(vec![Packet::List(vec![Packet::Number(1)]), Packet::Number(4)])
            )]
        )
    }

    #[test]
    fn test_parse_tokens_to_packets() {
        assert_eq!(
            parse_tokens_to_packets(Tokens::from(vec![
                Token::Open,
                Token::Open,
                Token::Digit(1),
                Token::Close,
                Token::Comma,
                Token::Digit(4),
                Token::Close
            ])),
            Packet::List(vec![Packet::List(vec![Packet::Number(1)]), Packet::Number(4)])
        );
        assert_eq!(
            parse_tokens_to_packets(Tokens::from(vec![
                Token::Open,
                Token::Open,
                Token::Digit(1),
                Token::Close,
                Token::Comma,
                Token::Open,
                Token::Digit(2),
                Token::Comma,
                Token::Digit(3),
                Token::Comma,
                Token::Digit(4),
                Token::Close,
                Token::Close,
            ])),
            Packet::List(vec![
                Packet::List(vec![Packet::Number(1)]),
                Packet::List(vec![Packet::Number(2), Packet::Number(3), Packet::Number(4)])
            ])
        )
    }

    #[test]
    fn test_compare_packets() {
        // [1,1,3,1,1] vs [1,1,5,1,1]
        assert_eq!(
            compare_packets(
                &Packet::List(vec![
                    Packet::Number(1),
                    Packet::Number(1),
                    Packet::Number(3),
                    Packet::Number(1),
                    Packet::Number(1),
                ]),
                &Packet::List(vec![
                    Packet::Number(1),
                    Packet::Number(1),
                    Packet::Number(5),
                    Packet::Number(1),
                    Packet::Number(1),
                ]),
            ),
            Ordering::Less
        );
        // [[1],[2,3,4]] vs [[1],4]
        assert_eq!(
            compare_packets(
                &Packet::List(vec![
                    Packet::List(vec![Packet::Number(1),]),
                    Packet::List(vec![Packet::Number(2), Packet::Number(3), Packet::Number(4),]),
                ]),
                &Packet::List(vec![
                    Packet::List(vec![Packet::Number(1),]),
                    Packet::Number(4),
                ]),
            ),
            Ordering::Less
        );
        // [9] vs [[8,7,6]]
        assert_eq!(
            compare_packets(
                &Packet::List(vec![Packet::Number(9),]),
                &Packet::List(vec![Packet::List(vec![
                    Packet::Number(8),
                    Packet::Number(7),
                    Packet::Number(6),
                ]),]),
            ),
            Ordering::Greater
        );
        // [[4,4],4,4] vs [[4,4],4,4,4]
        assert_eq!(
            compare_packets(
                &Packet::List(vec![
                    Packet::List(vec![Packet::Number(4), Packet::Number(4),]),
                    Packet::Number(4),
                    Packet::Number(4),
                ]),
                &Packet::List(vec![
                    Packet::List(vec![Packet::Number(4), Packet::Number(4),]),
                    Packet::Number(4),
                    Packet::Number(4),
                    Packet::Number(4),
                ]),
            ),
            Ordering::Less
        );
        // [7,7,7,7] vs [7,7,7]
        assert_eq!(
            compare_packets(
                &Packet::List(vec![
                    Packet::Number(7),
                    Packet::Number(7),
                    Packet::Number(7),
                    Packet::Number(7),
                ]),
                &Packet::List(vec![Packet::Number(7), Packet::Number(7), Packet::Number(7),]),
            ),
            Ordering::Greater
        );
    }
}
