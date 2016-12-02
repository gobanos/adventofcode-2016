struct Keypad {
    pad: [[char; 3]; 3],
    row: usize,
    col: usize,
}

enum Direction {
    Up,
    Right,
    Left,
    Down,
}

impl Keypad {
    fn current(&self) -> char {
        self.pad[self.row][self.col]
    }

    fn follow(&mut self, instructions: &[Direction]) -> char {
        for direction in instructions.iter() {
            match direction {
                &Direction::Up => {
                    self.row = if self.row == 0 { 0 } else { self.row - 1 };
                },
                &Direction::Right => {
                    self.col = if self.col == 2 { 2 } else { self.col + 1 };
                },
                &Direction::Left => {
                    self.col = if self.col == 0 { 0 } else { self.col - 1 };
                },
                &Direction::Down => {
                    self.row = if self.row == 2 { 2 } else { self.row + 1 };
                },
            }
        }

        self.current()
    }
}

impl Default for Keypad {
    fn default() -> Self {
        Keypad {
            pad: [
                ['1', '2', '3'],
                ['4', '5', '6'],
                ['7', '8', '9'],
            ],
            row: 1,
            col: 1,
        }
    }
}

pub fn run(input: &str) -> String {
    use self::Direction::*;

    let mut keypad = Keypad::default();
    let mut code = String::default();

    for line in input.lines() {
        let instructions = line.trim().chars().map(|c| {
            match c {
                'U' => Up,
                'R' => Right,
                'L' => Left,
                'D' => Down,
                _ =>  unreachable!(),
            }
        });

        code.push(keypad.follow(&instructions.collect::<Vec<_>>()));
    }

    code
}

pub fn challenge() -> String {
    use std::io::prelude::*;
    use std::fs::File;

    let mut f = File::open("resources/day2-problem1-input.txt").unwrap();
    let mut input = String::default();

    f.read_to_string(&mut input).unwrap();

    run(&input)
}

#[test]
fn parsed_sample() {
    use self::Direction::*;

    let mut keypad = Keypad::default();

    // ULL
    assert_eq!(keypad.follow(&[Up, Left, Left]), '1');

    // RRDDD
    assert_eq!(keypad.follow(&[Right, Right, Down, Down, Down]), '9');

    // LURDL
    assert_eq!(keypad.follow(&[Left, Up, Right, Down, Left]), '8');

    // UUUUD
    assert_eq!(keypad.follow(&[Up, Up, Up, Up, Down]), '5');
}

#[cfg(test)]
mod test {
    #[test]
    fn sample() {
        let input = "ULL\nRRDDD\nLURDL\nUUUUD";
        assert_eq!(super::run(input), "1985");
    }
}
