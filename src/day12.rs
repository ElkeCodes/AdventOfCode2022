use std::{
    cmp,
    collections::{HashMap, HashSet, VecDeque},
};

type Height = char;
type Coordinate = (usize, usize);
type HeightMap = HashMap<Coordinate, Height>;

pub fn part1(input: String) {
    println!("{}", part1_impl(&input))
}

pub fn part2(input: String) {
    println!("{}", part2_impl(&input))
}

fn parse_lines(input: &str) -> (HeightMap, usize, usize, Coordinate, Coordinate) {
    let mut height_map = HeightMap::new();
    let mut max_x = 0;
    let mut max_y = 0;
    let mut start_coordinate: Coordinate = (0, 0);
    let mut end_coordinate: Coordinate = (0, 0);
    input.lines().enumerate().for_each(|(y, line)| {
        max_y = cmp::max(max_y, y);
        line.chars().enumerate().for_each(|(x, height)| {
            max_x = cmp::max(max_x, x);
            if height == 'S' {
                start_coordinate = (x, y);
                height_map.entry(start_coordinate).or_insert('a');
            } else if height == 'E' {
                end_coordinate = (x, y);
                height_map.entry(end_coordinate).or_insert('z');
            } else {
                height_map.entry((x, y)).or_insert(height);
            }
        });
    });
    (height_map, max_x, max_y, start_coordinate, end_coordinate)
}

fn is_one_higher_or_equal(c1: char, c2: char) -> bool {
    c1 as u32 + 1 == c2 as u32 || c1 as u32 >= c2 as u32
}

fn is_one_lower_or_equal(c2: char, c1: char) -> bool {
    is_one_higher_or_equal(c1, c2)
}

fn get_adjacents(
    (x, y): Coordinate,
    height_map: &HeightMap,
    max_x: usize,
    max_y: usize,
    is_valid_adjacent: fn(char, char) -> bool,
) -> Vec<Coordinate> {
    let mut result = vec![];
    let test_coordinate = height_map.get(&(x, y)).unwrap();
    for test_x in cmp::max(1, x) - 1..=cmp::min(x + 1, max_x) {
        if is_valid_adjacent(*test_coordinate, *(height_map.get(&(test_x, y)).unwrap()))
            && x != test_x
        {
            result.push((test_x, y));
        }
    }
    for test_y in cmp::max(1, y) - 1..=cmp::min(y + 1, max_y) {
        if is_valid_adjacent(*test_coordinate, *(height_map.get(&(x, test_y)).unwrap()))
            && y != test_y
        {
            result.push((x, test_y));
        }
    }
    result
}

fn bfs(
    height_map: HeightMap,
    start_coordinate: Coordinate,
    max_x: usize,
    max_y: usize,
    is_end_coordinate: &dyn Fn(Coordinate) -> bool,
    is_valid_adjacent: fn(char, char) -> bool,
) -> Option<Vec<Option<Coordinate>>> {
    let mut queue = VecDeque::new();
    let mut visited_vertices = HashSet::with_capacity(height_map.len());
    let mut prev: HashMap<Coordinate, Option<Coordinate>> =
        HashMap::with_capacity(height_map.len());

    queue.push_back(start_coordinate);
    visited_vertices.insert(start_coordinate);

    let mut last_node = start_coordinate;
    while let Some(node) = queue.pop_front() {
        if is_end_coordinate(node) {
            last_node = node;
            break;
        }

        for adjacent_node in get_adjacents(node, &height_map, max_x, max_y, is_valid_adjacent) {
            if !visited_vertices.contains(&adjacent_node) {
                visited_vertices.insert(adjacent_node);
                prev.insert(adjacent_node, Some(node));
                queue.push_back(adjacent_node);
            }
        }
    }

    let mut path = Vec::new();
    let mut at = Some(last_node);
    while at != None {
        path.push(at);
        if prev.contains_key(&at.unwrap()) {
            at = *prev.get(&at.unwrap()).unwrap();
        } else {
            at = None;
        }
    }
    path.reverse();

    return match path[0] {
        Some(x) if x == start_coordinate => Some(path),
        _ => None,
    };
}

fn part1_impl(input: &str) -> usize {
    let (heightmap, max_x, max_y, start_coordinate, end_coordinate) = parse_lines(input);

    let comparator = |coordinate| end_coordinate == coordinate;
    match bfs(
        heightmap,
        start_coordinate,
        max_x,
        max_y,
        &comparator,
        is_one_higher_or_equal,
    ) {
        Some(path) => path.len() - 1,
        None => panic!("No path found"),
    }
}

fn part2_impl(input: &str) -> usize {
    let (heightmap, max_x, max_y, _, end_coordinate) = parse_lines(input);

    let comparator = |coordinate| *heightmap.get(&coordinate).unwrap() == 'a';
    match bfs(
        heightmap.clone(),
        end_coordinate,
        max_x,
        max_y,
        &comparator,
        is_one_lower_or_equal,
    ) {
        Some(path) => path.len() - 1,
        None => panic!("No path found"),
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    static TEST_INPUT: &str = "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi";

    #[test]
    fn test_part1() {
        assert_eq!(part1_impl(TEST_INPUT), 31);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2_impl(TEST_INPUT), 29);
    }

    #[test]
    fn test_parse_lines() {
        assert_eq!(
            parse_lines(TEST_INPUT),
            (
                HeightMap::from([
                    ((0, 0), 'a'),
                    ((0, 1), 'a'),
                    ((0, 2), 'a'),
                    ((0, 3), 'a'),
                    ((0, 4), 'a'),
                    ((1, 0), 'a'),
                    ((1, 1), 'b'),
                    ((1, 2), 'c'),
                    ((1, 3), 'c'),
                    ((1, 4), 'b'),
                    ((2, 0), 'b'),
                    ((2, 1), 'c'),
                    ((2, 2), 'c'),
                    ((2, 3), 'c'),
                    ((2, 4), 'd'),
                    ((3, 0), 'q'),
                    ((3, 1), 'r'),
                    ((3, 2), 's'),
                    ((3, 3), 't'),
                    ((3, 4), 'e'),
                    ((4, 0), 'p'),
                    ((4, 1), 'y'),
                    ((4, 2), 'z'),
                    ((4, 3), 'u'),
                    ((4, 4), 'f'),
                    ((5, 0), 'o'),
                    ((5, 1), 'x'),
                    ((5, 2), 'z'),
                    ((5, 3), 'v'),
                    ((5, 4), 'g'),
                    ((6, 0), 'n'),
                    ((6, 1), 'x'),
                    ((6, 2), 'x'),
                    ((6, 3), 'w'),
                    ((6, 4), 'h'),
                    ((7, 0), 'm'),
                    ((7, 1), 'l'),
                    ((7, 2), 'k'),
                    ((7, 3), 'j'),
                    ((7, 4), 'i'),
                ]),
                7,
                4,
                (0, 0),
                (5, 2)
            )
        )
    }

    #[test]
    fn test_is_one_higher() {
        assert_eq!(is_one_higher_or_equal('a', 'b'), true);
        assert_eq!(is_one_higher_or_equal('a', 'c'), false);
        assert_eq!(is_one_higher_or_equal('a', 'f'), false);
        assert_eq!(is_one_higher_or_equal('b', 'a'), true);
        assert_eq!(is_one_higher_or_equal('f', 'a'), true);
        assert_eq!(is_one_higher_or_equal('b', 'b'), true);
        assert_eq!(is_one_higher_or_equal('z', 'z'), true);
    }

    #[test]
    fn test_get_adjacents() {
        let (height_map, max_x, max_y, _, _) = parse_lines(TEST_INPUT);
        assert_eq!(
            get_adjacents((0, 0), &height_map, max_x, max_y, is_one_higher_or_equal),
            vec![(1, 0), (0, 1)]
        );
        assert_eq!(
            get_adjacents((1, 1), &height_map, max_x, max_y, is_one_higher_or_equal),
            vec![(0, 1), (2, 1), (1, 0), (1, 2)]
        );
        assert_eq!(
            get_adjacents((2, 2), &height_map, max_x, max_y, is_one_higher_or_equal),
            vec![(1, 2), (2, 1), (2, 3)]
        );
        assert_eq!(
            get_adjacents((4, 2), &height_map, max_x, max_y, is_one_higher_or_equal),
            vec![(3, 2), (5, 2), (4, 1), (4, 3)]
        );
    }

    #[test]
    fn test_bfs() {
        let (height_map, max_x, max_y, start_coordinate, end_coordinate) = parse_lines(TEST_INPUT);
        let comparator = |x| x == end_coordinate;
        assert_eq!(
            bfs(
                height_map.clone(),
                start_coordinate,
                max_x,
                max_y,
                &comparator,
                is_one_higher_or_equal
            ),
            Some(vec![
                Some((0, 0)),
                Some((1, 0)),
                Some((2, 0)),
                Some((2, 1)),
                Some((2, 2)),
                Some((2, 3)),
                Some((2, 4)),
                Some((3, 4)),
                Some((4, 4)),
                Some((5, 4)),
                Some((6, 4)),
                Some((7, 4)),
                Some((7, 3)),
                Some((7, 2)),
                Some((7, 1)),
                Some((7, 0)),
                Some((6, 0)),
                Some((5, 0)),
                Some((4, 0)),
                Some((3, 0)),
                Some((3, 1)),
                Some((3, 2)),
                Some((3, 3)),
                Some((4, 3)),
                Some((5, 3)),
                Some((6, 3)),
                Some((6, 2)),
                Some((6, 1)),
                Some((5, 1)),
                Some((4, 1)),
                Some((4, 2)),
                Some((5, 2))
            ])
        );

        let two_two_comparator = |x: Coordinate| x == (2, 2);
        assert_eq!(
            bfs(
                height_map.clone(),
                (1, 3),
                max_x,
                max_y,
                &two_two_comparator,
                is_one_higher_or_equal
            ),
            Some(vec![Some((1, 3)), Some((2, 3)), Some((2, 2))])
        )
    }
}
