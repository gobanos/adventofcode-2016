use std::collections::HashMap;
use regex::Regex;

struct Room {
    id: usize,
    words: Vec<String>,
    letters: HashMap<char, usize>,
    checksum: String,
}

impl Room {
    fn new(id: usize, checksum: &str) -> Room {
        Room {
            id: id,
            words: Vec::new(),
            letters: HashMap::new(),
            checksum: checksum.into(),
        }
    }

    fn add_word(&mut self, word: &str) {
        self.words.push(word.into());
        for c in word.chars() {
            let count = self.letters.get(&c).unwrap_or(&0) + 1;
            self.letters.insert(c, count);
        }
    }

    fn is_real(&self) -> bool {
        let mut sum = String::with_capacity(5);

        for _ in 0..5 {
            let mut max = (' ', 0);
            for (&c, &count) in &self.letters {
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

    fn decrypt(&self) -> String {
        let mut result = String::new();
        let a_int = 'a' as isize;
        for word in &self.words {
            for c in word.chars() {
                let as_int = (c as isize) - a_int as isize;
                let decrypted_int = ((as_int + self.id as isize) % 26 + 26) % 26 + a_int;
                result.push(decrypted_int as u8 as char);
            }
            result.push(' ');
        }
        result.trim().into()
    }
}

pub fn run(input: &str) -> Option<usize> {
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

        if room.is_real() && room.decrypt() == "northpole object storage" {
            return Some(room.id);
        }
    }

    None
}

pub fn challenge() -> usize {
    use std::io::prelude::*;
    use std::fs::File;

    let mut f = File::open("resources/day4-problem1-input.txt").unwrap();
    let mut input = String::default();

    f.read_to_string(&mut input).unwrap();

    run(&input).unwrap()
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

#[test]
fn decrypt() {
    let mut room = Room::new(343, "");

    room.add_word("qzmt");
    room.add_word("zixmtkozy");
    room.add_word("ivhz");

    assert_eq!(room.decrypt(), "very encrypted name");
}
