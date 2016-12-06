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
            password: "________".into(),
        }
    }

    fn md5(&self) -> String {
        let to_hash = format!("{}{}", self.id, self.index);
        let mut digest = Md5::new();

        digest.input_str(&to_hash);

        digest.result_str()
    }

    fn find_char(&mut self) -> (char, char) {
        loop {
            let md5 = self.md5();
            self.index += 1;

            if md5.starts_with("00000") {
                let mut iter = md5.chars().skip(5);
                return (
                    iter.next().unwrap(),
                    iter.next().unwrap(),
                )
            }
        }
    }

    fn break_password(&mut self) -> &str {
        let mut found = (0..8).map(|_| false).collect::<Vec<_>>();
        loop {
            let (key, pass_char) = self.find_char();
            if let Some(key) = key.to_digit(10) {
                let key = key as usize;
                if key < 8 && !found[key] {
                    found[key] = true;
                    self.password = format!("{}{}{}", &self.password[0..key], pass_char, &self.password[key+1..]);

                    if !found.contains(&false) {
                        return &self.password
                    }
                }
            }
        }
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

#[cfg(test)]
mod test {
    #[test]
    #[ignore]
    fn sample() {
        assert_eq!(super::run("abc"), "05ace8e3");
    }
}