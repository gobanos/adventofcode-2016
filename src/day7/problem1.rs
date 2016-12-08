fn is_abba(input: &str) -> bool {
    assert!(input.len() >= 4);

    let input: Vec<_> = input.chars().take(4).collect();

    input[0] == input[3] && input[1] == input[2] && input[0] != input[1]
}

fn is_word_abba(input: &str) -> bool {
    for i in 0..input.len() - 3 {
        if is_abba(&input[i..]) {
            return true;
        }
    }
    false
}

fn is_tls(address: &str) -> bool {
    let words = address.split(|c| c == '[' || c == ']');
    let is_squared = [false, true];

    let mut is_tls = false;

    for (word, &is_squared) in words.zip(is_squared.iter().cycle()) {
        if is_word_abba(word) {
            if is_squared {
                return false;
            }
            is_tls = true;
        }
    }

    is_tls
}

pub fn run(input: &str) -> usize {
    input.lines().filter(|l| is_tls(l.trim())).count()
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
fn parsed_sample1() {
    assert!(is_abba("abba"));
    assert!(!is_abba("aaaa"));
    assert!(!is_abba("qrst"));
}

#[test]
fn parsed_sample2() {
    assert!(is_word_abba("ioxxoj"));
    assert!(!is_word_abba("zxcvbn"));
}

#[test]
fn sample1() {
    assert!(is_tls("abba[mnop]qrst"));
    assert!(!is_tls("abcd[bddb]xyyx"));
    assert!(!is_tls("aaaa[qwer]tyui"));
    assert!(is_tls("ioxxoj[asdfgh]zxcvbn"));
}

#[cfg(test)]
mod test {
    #[test]
    fn sample() {
        let input = "abba[mnop]qrst\nabcd[bddb]xyyx\naaaa[qwer]tyui\nioxxoj[asdfgh]zxcvbn";
        assert_eq!(super::run(input), 2);
    }
}
