use crypto::md5::Md5;
use crypto::digest::Digest;

struct WarGame {
    id: Vec<u8>,
    index: usize,
    password: String,
    digest: Md5,
}

impl WarGame {
    fn new(id: &str) -> Self {
        WarGame {
            id: id.as_bytes().into(),
            index: 0,
            password: "________".into(),
            digest: Md5::new(),
        }
    }

    fn md5(&mut self) -> [u8; 16] {
        let mut output = [0; 16];

        self.digest.input(&self.id);
        self.digest.input(self.index.to_string().as_bytes());

        self.digest.result(&mut output);

        self.digest.reset();

        output
    }

    fn find_char(&mut self) -> (u8, u8) {
        loop {
            let md5 = self.md5();
            self.index += 1;

            let sum = md5[0] as i32 + md5[1] as i32 + (md5[2] >> 4) as i32;
            if sum == 0 {
                return (md5[2], md5[3] >> 4);
            }

        }
    }

    fn break_password(&mut self) -> &str {
        let mut found = (0..8).map(|_| false).collect::<Vec<_>>();
        loop {
            let (key, pass_char) = self.find_char();
            let key = key as usize;
            let pass_char = match pass_char {
                0 => '0',
                1 => '1',
                2 => '2',
                3 => '3',
                4 => '4',
                5 => '5',
                6 => '6',
                7 => '7',
                8 => '8',
                9 => '9',
                10 => 'a',
                11 => 'b',
                12 => 'c',
                13 => 'd',
                14 => 'e',
                15 => 'f',
                _ => unreachable!(),
            };

            if key < 8 && !found[key] {
                found[key] = true;
                self.password = format!("{}{}{}",
                                        &self.password[0..key],
                                        pass_char,
                                        &self.password[key + 1..]);

                if !found.contains(&false) {
                    return &self.password;
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
    fn sample() {
        assert_eq!(super::run("abc"), "05ace8e3");
    }
}
