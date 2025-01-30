use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{self, BufRead, BufReader},
};

mod part2;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum Key {
    Value(char),
    Invalid,
}

fn arrow_from_dir(dir: (i64, i64)) -> char {
    match dir {
        (0, 1) => '>',
        (0, -1) => '<',
        (1, 0) => 'v',
        (-1, 0) => '^',
        _ => panic!(),
    }
}

#[derive(Debug)]
struct Keypad {
    inner: Vec<Vec<Key>>,
    char_map: HashMap<char, (i64, i64)>,
    width: i64,
    height: i64,
}

impl Keypad {
    fn new(keypad: Vec<Vec<Key>>) -> Self {
        let mut char_map = HashMap::new();
        for i in 0..keypad.len() {
            for j in 0..keypad[0].len() {
                if let Key::Value(val) = keypad[i][j] {
                    char_map.insert(val, (i as i64, j as i64));
                }
            }
        }
        Self {
            width: keypad[0].len() as i64,
            height: keypad.len() as i64,
            inner: keypad,
            char_map,
        }
    }

    fn robot_sequence(&mut self, sequence: &[char]) -> Vec<Vec<char>> {
        let mut sequences = Vec::new();
        for (from, to) in sequence.into_iter().tuple_windows() {
            sequences.push(self.move_to_key(*from, *to));
        }
        sequences
            .iter()
            .map(|s| s.iter())
            .multi_cartesian_product()
            .map(|seq| seq.into_iter().flatten().map(|x| *x).collect::<Vec<char>>())
            .collect()
    }

    fn move_to_key(&self, from: char, key: char) -> HashSet<Vec<char>> {
        let mut sequences = HashSet::new();
        let mut visit = VecDeque::new();
        visit.push_back((*self.char_map.get(&from).unwrap(), Vec::new()));
        let mut min_length = None;
        let mut visited = HashSet::new();
        while let Some((pos, mut sequence)) = visit.pop_front() {
            visited.insert(pos);
            match self.inner[pos.0 as usize][pos.1 as usize] {
                Key::Value(val) => {
                    if let Some(min_len) = min_length {
                        if sequence.len() + 1 > min_len {
                            continue;
                        }
                    }
                    if val == key {
                        sequence.push('A');
                        min_length = Some(sequence.len());
                        sequences.insert(sequence);
                    } else {
                        for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                            let next = self.next_pos(pos, dir);
                            let next = if let Some(next) = next {
                                next
                            } else {
                                continue;
                            };

                            if visited.get(&next).is_none() {
                                let mut sequence = sequence.clone();
                                sequence.push(arrow_from_dir(dir));
                                visit.push_back((next, sequence));
                            }
                        }
                    }
                }
                Key::Invalid => {}
            }
        }
        sequences
    }

    fn next_pos(&self, pos: (i64, i64), dir: (i64, i64)) -> Option<(i64, i64)> {
        let next = (pos.0 + dir.0, pos.1 + dir.1);
        if self.is_valid_pos(next) {
            Some(next)
        } else {
            None
        }
    }

    fn is_valid_pos(&self, pos: (i64, i64)) -> bool {
        pos.0 >= 0 && pos.0 < self.height && pos.1 >= 0 && pos.1 < self.width
    }
}

struct Chain {
    numpad: Keypad,
    arrowpad: Keypad,
    cache: HashMap<(char, char, i64), HashSet<Vec<char>>>,
    len: i64,
}

impl Chain {
    pub fn new(len: i64) -> Self {
        let numlayout = vec![
            vec![Key::Value('7'), Key::Value('8'), Key::Value('9')],
            vec![Key::Value('4'), Key::Value('5'), Key::Value('6')],
            vec![Key::Value('1'), Key::Value('2'), Key::Value('3')],
            vec![Key::Invalid, Key::Value('0'), Key::Value('A')],
        ];
        let arrowlayout = vec![
            vec![Key::Invalid, Key::Value('^'), Key::Value('A')],
            vec![Key::Value('<'), Key::Value('v'), Key::Value('>')],
        ];
        Self {
            numpad: Keypad::new(numlayout.clone()),
            arrowpad: Keypad::new(arrowlayout.clone()),
            cache: HashMap::new(),
            len,
        }
    }

    fn robot_sequence(&self, sequence: &[char], depth: i64) -> HashSet<Vec<char>> {
        let res = HashSet::new();
        for pair in sequence.into_iter().tuple_windows::<(&char, &char)>() {
            if let Some(seqs) = self.cache.get(&(*pair.0, *pair.1, depth)) {
                return seqs.clone();
            }

            let keypad = if depth == self.len {
                &self.numpad
            } else {
                &self.arrowpad
            };

            if depth == 1 {
                return keypad.move_to_key(*pair.0, *pair.1);
            }

            for mut seq in keypad.move_to_key(*pair.0, *pair.1) {
                seq.insert(0, 'A');
                self.robot_sequence(seq.as_slice(), depth - 1);
            }
        }
        res
    }
}

fn main() {
    // let numlayout = vec![
    //     vec![Key::Value('7'), Key::Value('8'), Key::Value('9')],
    //     vec![Key::Value('4'), Key::Value('5'), Key::Value('6')],
    //     vec![Key::Value('1'), Key::Value('2'), Key::Value('3')],
    //     vec![Key::Invalid, Key::Value('0'), Key::Value('A')],
    // ];
    // let arrowlayout = vec![
    //     vec![Key::Invalid, Key::Value('^'), Key::Value('A')],
    //     vec![Key::Value('<'), Key::Value('v'), Key::Value('>')],
    // ];

    // let mut sum = 0;
    // let input = File::open("input").unwrap();
    // for line in io::BufReader::new(input).lines() {
    //     let mut robots = vec![Keypad::new(numlayout.clone())];
    //     (0..2).for_each(|_| robots.push(Keypad::new(arrowlayout.clone())));
    //     let line = line.unwrap();
    //     let num_sequence = line.trim();
    //     let mut input_sequences =
    //         HashSet::from_iter(vec![num_sequence.chars().collect::<Vec<char>>()]);
    //     for mut robot in robots {
    //         let mut output_sequences = HashSet::new();
    //         for mut s in input_sequences {
    //             s.insert(0, 'A');
    //             output_sequences.extend(robot.robot_sequence(s.as_slice()));
    //         }
    //         let shortest_len = output_sequences
    //             .iter()
    //             .fold(None, |acc, x| {
    //                 if let Some(min) = acc {
    //                     if x.len() < min {
    //                         Some(x.len())
    //                     } else {
    //                         Some(min)
    //                     }
    //                 } else {
    //                     Some(x.len())
    //                 }
    //             })
    //             .unwrap();
    //         let output_sequences = output_sequences
    //             .into_iter()
    //             .filter(|s| s.len() == shortest_len)
    //             .collect::<HashSet<Vec<char>>>();
    //         input_sequences = output_sequences;
    //     }

    //     let shortest = input_sequences.into_iter().next().unwrap().len();
    //     let num_part = num_sequence
    //         .split("A")
    //         .next()
    //         .unwrap()
    //         .parse::<usize>()
    //         .unwrap();
    //     sum += shortest * num_part;
    // }
    // println!("{sum}");
    part2::run();
}
