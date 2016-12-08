use std::collections::{HashMap, HashSet};
use std::mem;

#[derive(Hash, Eq, PartialEq, Debug)]
struct ABA {
    a: char,
    b: char,
}

impl ABA {
    fn new(a: char, b: char) -> ABA {
        ABA { a: a, b: b }
    }

    fn revert(&mut self) {
        mem::swap(&mut self.a, &mut self.b);
    }
}

fn is_aba(input: &str) -> Option<ABA> {
    assert!(input.len() >= 3);

    let input: Vec<_> = input.chars().take(3).collect();

    if input[0] == input[2] && input[0] != input[1] {
        Some(ABA::new(input[0], input[1]))
    } else {
        None
    }
}

fn get_word_aba(input: &str, reverted: bool) -> Vec<ABA> {
    let mut result = Vec::new();

    for i in 0..input.len() - 2 {
        if let Some(mut aba) = is_aba(&input[i..]) {
            if reverted {
                aba.revert();
            }
            result.push(aba);
        }
    }

    result
}

fn is_ssl(address: &str) -> bool {
    let words = address.split(|c| c == '[' || c == ']');
    let is_squared = [false, true];

    let mut hash_map = HashMap::with_capacity(2);
    hash_map.insert(true, HashSet::new());
    hash_map.insert(false, HashSet::new());

    for (word, &is_squared) in words.zip(is_squared.iter().cycle()) {
        for aba in get_word_aba(word, is_squared) {
            if hash_map[&!is_squared].contains(&aba) {
                return true;
            }
            hash_map.get_mut(&is_squared).unwrap().insert(aba);
        }
    }

    false
}

pub fn run(input: &str) -> usize {
    input.lines().filter(|l| is_ssl(l.trim())).count()
}

pub fn challenge() -> usize {
    use std::io::prelude::*;
    use std::fs::File;

    let mut f = File::open("resources/day7-problem1-input.txt").unwrap();
    let mut input = String::default();

    f.read_to_string(&mut input).unwrap();

    run(&input)
}

#[test]
fn aba() {
    assert_eq!(is_aba("aba"), Some(ABA { a: 'a', b: 'b' }));
}

#[test]
fn word_aba() {
    assert_eq!(get_word_aba("aba", false), vec![ABA { a: 'a', b: 'b' }]);
    assert_eq!(get_word_aba("aba", true), vec![ABA { a: 'b', b: 'a' }]);
}

#[test]
fn sample1() {
    assert!(is_ssl("aba[bab]xyz"));
    assert!(!is_ssl("xyx[xyx]xyx"));
    assert!(is_ssl("aaa[kek]eke"));
    assert!(is_ssl("zazbz[bzb]cdb"));
}

#[cfg(test)]
mod test {
    #[test]
    fn sample() {
        let input = "aba[bab]xyz\nxyx[xyx]xyx\naaa[kek]eke\nzazbz[bzb]cdb";
        assert_eq!(super::run(input), 3);
    }
}
