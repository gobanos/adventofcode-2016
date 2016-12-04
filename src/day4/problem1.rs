use std::collections::HashMap;
use regex::Regex;

struct Room {
    id: usize,
    letters: HashMap<char, usize>,
    checksum: String,
}

impl Room {
    fn new(id: usize, checksum: &str) -> Room {
        Room {
            id: id,
            letters: HashMap::new(),
            checksum: checksum.into(),
        }
    }

    fn add_word(&mut self, word: &str) {
        for c in word.chars() {
            let count = self.letters.get(&c).unwrap_or(&0) + 1;
            self.letters.insert(c, count);
        }
    }

    fn is_real(&self) -> bool {
        let mut sum = String::with_capacity(5);

        for _ in 0..5 {
            let mut max = (' ', 0);
            for (&c, &count) in self.letters.iter() {
                if sum.contains(c) {
                    continue;
                }
                let max_count = max.1;
                if count > max_count || (count == max_count && c < max.0) {
                    max = (c, count);
                }
            }
            sum.push(max.0);
        }

        sum == self.checksum
    }
}

pub fn run(input: &str) -> usize {
    let mut sum = 0;

    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?m)^([a-z-]+)-(\d+)\[([a-z]{5})\]$").unwrap();
    }

    for cap in RE.captures_iter(input) {
        let words = cap.at(1).unwrap();
        let id = cap.at(2).unwrap().parse().unwrap();
        let checksum = cap.at(3).unwrap();

        let mut room = Room::new(id, checksum);

        for word in words.split('-') {
            room.add_word(word);
        }

        if room.is_real() {
            sum += room.id;
        }
    }

    sum
}

pub fn challenge() -> usize {
    use std::io::prelude::*;
    use std::fs::File;

    let mut f = File::open("resources/day4-problem1-input.txt").unwrap();
    let mut input = String::default();

    f.read_to_string(&mut input).unwrap();

    run(&input)
}

#[test]
fn parsed_sample1() {
    let mut room = Room::new(123, "abxyz");

    room.add_word("aaaaa");
    room.add_word("bbb");
    room.add_word("z");
    room.add_word("y");
    room.add_word("x");

    assert!(room.is_real());
}

#[test]
fn parsed_sample2() {
    let mut room = Room::new(987, "abcde");

    room.add_word("a");
    room.add_word("b");
    room.add_word("c");
    room.add_word("d");
    room.add_word("e");
    room.add_word("f");
    room.add_word("g");
    room.add_word("h");

    assert!(room.is_real());
}

#[test]
fn parsed_sample3() {
    let mut room = Room::new(404, "oarel");

    room.add_word("not");
    room.add_word("a");
    room.add_word("real");
    room.add_word("room");

    assert!(room.is_real());
}

#[test]
fn parsed_sample4() {
    let mut room = Room::new(200, "decoy");

    room.add_word("totally");
    room.add_word("real");
    room.add_word("room");

    assert!(!room.is_real());
}

#[cfg(test)]
mod test {
    #[test]
    fn sample() {
        let sum = super::run("aaaaa-bbb-z-y-x-123[abxyz]\na-b-c-d-e-f-g-h-987[abcde]\nnot-a-real-room-404[oarel]\ntotally-real-room-200[decoy]");
        assert_eq!(sum, 1514);
    }
}
