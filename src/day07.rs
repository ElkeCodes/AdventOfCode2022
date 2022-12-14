use std::{cell::RefCell, cmp, rc::Rc};

use itertools::Itertools;

#[derive(Debug, PartialEq, Copy, Clone)]
enum EntryType {
    Directory,
    File,
}

#[derive(Debug, PartialEq, Clone)]
struct Entry {
    entry_type: EntryType,
    name: String,
    size: isize,
    parent: Option<Rc<RefCell<Entry>>>,
    children: Vec<Rc<RefCell<Entry>>>,
}

pub fn part1(input: String) {
    println!("{}", part1_impl(&input))
}

pub fn part2(input: String) {
    println!("{}", part2_impl(&input))
}

fn parse_lines(input: &str) -> Rc<RefCell<Entry>> {
    let directory_tree = Rc::new(RefCell::new(Entry {
        entry_type: EntryType::Directory,
        name: String::from("/"),
        size: 0,
        parent: None,
        children: Vec::default(),
    }));
    let mut current_node = Rc::clone(&directory_tree);
    let mut lines = input.lines().skip(1).peekable();
    'main: loop {
        match lines.next() {
            Some(line) => {
                if line.starts_with("$ ls") {
                    let mut new_contents = Vec::default();
                    loop {
                        match lines.peek() {
                            Some(&next_line) => {
                                if next_line.starts_with("dir") {
                                    let current_line = lines
                                        .next()
                                        .unwrap()
                                        .split_whitespace()
                                        .collect::<Vec<&str>>();

                                    new_contents.push(Rc::new(RefCell::new(Entry {
                                        entry_type: EntryType::Directory,
                                        name: String::from(current_line[1]),
                                        size: 0,
                                        parent: Some(Rc::clone(&current_node)),
                                        children: Vec::default(),
                                    })));
                                } else if next_line.chars().next().unwrap().is_numeric() {
                                    let current_line = lines
                                        .next()
                                        .unwrap()
                                        .split_whitespace()
                                        .collect::<Vec<&str>>();
                                    new_contents.push(Rc::new(RefCell::new(Entry {
                                        entry_type: EntryType::File,
                                        name: String::from(current_line[1]),
                                        size: current_line[0].parse::<isize>().unwrap(),
                                        parent: None,
                                        children: Vec::default(),
                                    })));
                                } else {
                                    break;
                                }
                            }
                            _ => break,
                        }
                    }
                    current_node.borrow_mut().children = new_contents;
                } else if line.starts_with("$ cd") {
                    let target_directory = line.split(" ").collect_vec()[2];
                    let new_node = Rc::clone(&current_node);
                    if target_directory == ".." {
                        current_node = Rc::clone(new_node.borrow_mut().parent.as_ref().unwrap());
                    } else {
                        current_node = Rc::clone(
                            new_node
                                .borrow_mut()
                                .children
                                .iter()
                                .find(|x| {
                                    let y = x.borrow_mut();
                                    y.name == target_directory
                                        && y.entry_type == EntryType::Directory
                                })
                                .unwrap(),
                        );
                    }
                } else {
                    break;
                }
            }
            _ => break 'main,
        }
    }

    directory_tree
}

fn calculate_directory_size(directory: &Rc<RefCell<Entry>>) -> isize {
    let ref_directory = directory.as_ref().borrow_mut();
    unsafe {
        let result = ref_directory.children.iter().fold(0, |acc, child| {
            let entry = (*child).as_ptr();
            match (*entry).entry_type {
                EntryType::Directory => acc + calculate_directory_size(&child),
                EntryType::File => acc + (*entry).size,
            }
        });
        result
    }
}

fn sum_directories(directory: &Rc<RefCell<Entry>>) -> isize {
    let ref_directory = directory.as_ref().borrow_mut();
    unsafe {
        let result = ref_directory.children.iter().fold(0, |acc, child| {
            let entry = (*child).as_ptr();
            match (*entry).entry_type {
                EntryType::Directory => {
                    let r = calculate_directory_size(&child);
                    if r <= 100000 {
                        acc + r + sum_directories(&child)
                    } else {
                        acc + sum_directories(&child)
                    }
                }
                _ => acc,
            }
        });
        result
    }
}

fn remove_directory(directory: &Rc<RefCell<Entry>>, total_size: isize, smallest: isize) -> isize {
    let ref_directory = directory.as_ref().borrow_mut();
    unsafe {
        let result = ref_directory.children.iter().fold(smallest, |acc, child| {
            let entry = (*child).as_ptr();
            match (*entry).entry_type {
                EntryType::Directory => {
                    let r = calculate_directory_size(&child);
                    if r <= acc && 70000000 - total_size + r >= 30000000 {
                        cmp::min(r, remove_directory(&child, total_size, smallest))
                    } else {
                        cmp::min(acc, remove_directory(&child, total_size, smallest))
                    }
                }
                _ => acc,
            }
        });
        result
    }
}
fn part1_impl(input: &str) -> isize {
    let tree = parse_lines(input);
    sum_directories(&tree)
}

fn part2_impl(input: &str) -> isize {
    let tree = parse_lines(input);
    remove_directory(&tree, calculate_directory_size(&tree), 70000000)
}
#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_part1() {
        assert_eq!(part1_impl(TEST_INPUT), 95437);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2_impl(TEST_INPUT), 24933642);
    }

    #[test]
    fn test_calculate_directory_size() {
        assert_eq!(
            calculate_directory_size(&Rc::new(RefCell::new(Entry {
                entry_type: EntryType::Directory,
                name: String::from("e"),
                size: 0,
                parent: None,
                children: vec![Rc::new(RefCell::new(Entry {
                    entry_type: EntryType::File,
                    size: 584,
                    name: String::from("i"),
                    parent: None,
                    children: vec![]
                }))]
            }))),
            584
        );
        assert_eq!(
            calculate_directory_size(&Rc::from(RefCell::from(Entry {
                entry_type: EntryType::Directory,
                name: String::from("a"),
                size: 0,
                parent: None,
                children: vec![
                    Rc::from(RefCell::from(Entry {
                        entry_type: EntryType::Directory,
                        name: String::from("e"),
                        size: 0,
                        parent: None,
                        children: vec![Rc::from(RefCell::from(Entry {
                            entry_type: EntryType::File,
                            name: String::from("i"),
                            size: 584,
                            parent: None,
                            children: vec![]
                        })),]
                    })),
                    Rc::from(RefCell::from(Entry {
                        entry_type: EntryType::File,
                        name: String::from("f"),
                        size: 29116,
                        parent: None,
                        children: vec![]
                    })),
                    Rc::from(RefCell::from(Entry {
                        entry_type: EntryType::File,
                        name: String::from("g"),
                        size: 2557,
                        parent: None,
                        children: vec![]
                    })),
                    Rc::from(RefCell::from(Entry {
                        entry_type: EntryType::File,
                        name: String::from("h.lst"),
                        size: 62596,
                        parent: None,
                        children: vec![]
                    })),
                ]
            })),),
            94853
        )
    }

    #[test]
    fn test_parse_lines() {
        let parsed_result = parse_lines(TEST_INPUT);

        // /
        let directory_top = parsed_result.borrow_mut();
        assert_eq!(directory_top.name, "/");
        assert_eq!(directory_top.children.len(), 4);

        // /a
        let directory_a = directory_top.children[0].borrow_mut();
        assert_eq!(directory_a.name, "a");
        assert_eq!(directory_a.children.len(), 4);

        // /a/e
        let directory_e = directory_a.children[0].borrow_mut();
        assert_eq!(directory_e.name, "e");
        assert_eq!(directory_e.children.len(), 1);

        // /a/e/i
        let file_i = directory_e.children[0].borrow_mut();
        assert_eq!(file_i.name, "i");
        assert_eq!(file_i.size, 584);

        // /a/f
        let file_f = directory_a.children[1].borrow_mut();
        assert_eq!(file_f.name, "f");
        assert_eq!(file_f.size, 29116);

        // /a/g
        let file_g = directory_a.children[2].borrow_mut();
        assert_eq!(file_g.name, "g");
        assert_eq!(file_g.size, 2557);

        // /a/h.lst
        let file_hlst = directory_a.children[3].borrow_mut();
        assert_eq!(file_hlst.name, "h.lst");
        assert_eq!(file_hlst.size, 62596);

        // /b.txt
        let file_btxt = directory_top.children[1].borrow_mut();
        assert_eq!(file_btxt.name, "b.txt");
        assert_eq!(file_btxt.size, 14848514);

        // /c.dat
        let file_cdat = directory_top.children[2].borrow_mut();
        assert_eq!(file_cdat.name, "c.dat");
        assert_eq!(file_cdat.size, 8504156);

        // /d
        let directory_d = directory_top.children[3].borrow_mut();
        assert_eq!(directory_d.name, "d");
        assert_eq!(directory_d.children.len(), 4);

        // /d/j
        let file_j = directory_d.children[0].borrow_mut();
        assert_eq!(file_j.name, "j");
        assert_eq!(file_j.size, 4060174);

        // /d/d.log
        let file_dlog = directory_d.children[1].borrow_mut();
        assert_eq!(file_dlog.name, "d.log");
        assert_eq!(file_dlog.size, 8033020);

        // /d/d.ext
        let file_dext = directory_d.children[2].borrow_mut();
        assert_eq!(file_dext.name, "d.ext");
        assert_eq!(file_dext.size, 5626152);

        // /d/k
        let file_k = directory_d.children[3].borrow_mut();
        assert_eq!(file_k.name, "k");
        assert_eq!(file_k.size, 7214296);
    }
}
