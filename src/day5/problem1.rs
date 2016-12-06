use crypto::md5::Md5;
use crypto::digest::Digest;

struct WarGame {
    id: String,
    index: usize,
    password: String,
}

impl WarGame {
    fn new(id: &str) -> Self {
        WarGame {
            id: id.into(),
            index: 0,
            password: String::with_capacity(8),
        }
    }

    fn md5(&self) -> String {
        let to_hash = format!("{}{}", self.id, self.index);
        let mut digest = Md5::new();

        digest.input_str(&to_hash);

        digest.result_str()
    }

    fn find_char(&mut self) -> char {
        loop {
            let md5 = self.md5();
            self.index += 1;

            if md5.starts_with("00000") {
                return md5.chars().skip(5).next().unwrap()
            }
        }
    }

    fn break_password(&mut self) -> &str {
        for _ in 0..8 {
            let pass_char = self.find_char();
            self.password.push(pass_char);
        }
        &self.password
    }
}

pub fn run(id: &str) -> String {
    let mut war_game = WarGame::new(id);

    war_game.break_password().into()
}

pub fn challenge() -> String {
    use std::io::prelude::*;
    use std::fs::File;

    let mut f = File::open("resources/day5-problem1-input.txt").unwrap();
    let mut input = String::default();

    f.read_to_string(&mut input).unwrap();

    run(input.trim())
}

#[test]
#[ignore]
fn sample() {
    let mut war_game = WarGame::new("abc");

    assert_eq!(war_game.find_char(), '1');
    assert_eq!(war_game.find_char(), '8');
    assert_eq!(war_game.find_char(), 'f');
}

#[cfg(test)]
mod test {
    #[test]
    #[ignore]
    fn sample() {
        assert_eq!(super::run("abc"), "18f47a30");
    }
}